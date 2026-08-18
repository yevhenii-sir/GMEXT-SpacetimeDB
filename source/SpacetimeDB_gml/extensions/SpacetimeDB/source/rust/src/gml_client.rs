//! GML Client state management.
//!
//! Contains the `ClientState` struct, handle management globals,
//! and helper functions used by both the FFI layer and the WebSocket loop.

use crate::client_cache::ClientCache;
use futures::channel::mpsc as fc_mpsc;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ffi::CString;
use std::sync::Arc;
use std::time::Duration;
use tokio::runtime::Runtime;
use tokio::time::sleep;

use libc::{c_char, c_double};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Protocol contract version sent in every event JSON.
pub const CONTRACT_VERSION: u32 = 1;

/// Soft cap on pending events. When exceeded, oldest events are dropped.
pub const MAX_PENDING_EVENTS: usize = 4096;

// ---------------------------------------------------------------------------
// Handle type & global state
// ---------------------------------------------------------------------------

pub type Handle = u64;

static NEXT_HANDLE: Lazy<Mutex<Handle>> = Lazy::new(|| Mutex::new(1));

pub static HANDLES: Lazy<Mutex<HashMap<Handle, Arc<Mutex<ClientState>>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

pub static GLOBAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name("spacetimedb-gms-runtime")
        .build()
        .expect("failed to create tokio runtime")
});

thread_local! {
    pub static TLS_RETURN: Mutex<Option<CString>> = const { Mutex::new(None) };
}

// ---------------------------------------------------------------------------
// ClientState
// ---------------------------------------------------------------------------

pub struct ClientState {
    pub connected: bool,
    pub last_error: String,
    pub events: VecDeque<Value>,
    /// Count of events dropped since last poll due to `MAX_PENDING_EVENTS`.
    pub events_dropped: u64,
    /// When false, noisy meta events (reducer_sent, ws_tx_*, queued_for_retry, …) are suppressed.
    pub meta_events_enabled: bool,
    pub task: Option<tokio::task::JoinHandle<()>>,
    pub outgoing: Option<fc_mpsc::Sender<Vec<u8>>>,
    pub retry_queue: VecDeque<(Vec<u8>, u32)>,
    pub next_query_set_id: u32,
    pub subscription_sql_to_qid: HashMap<String, u32>,
    pub subscription_qid_to_sql: HashMap<u32, String>,
    /// For BATCH_SUBSCRIBE: stores the original SQL array per qid for resubscription.
    pub subscription_qid_to_sql_array: HashMap<u32, Vec<String>>,
    pub pending_requests: HashMap<u32, String>,
    pub default_request_timeout_ms: i64,
    // Runtime table schemas registered from GameMaker (pre-parsed JSON Values)
    pub table_schemas: HashMap<String, Value>,
    pub struct_schemas: HashMap<String, Value>,
    // Reducer name -> error type schema (for decoding Err payloads)
    pub reducer_error_schemas: HashMap<String, String>,
    pub seen_subscribe_applied: HashSet<u32>,
    pub cached_token: Option<String>,
    pub compression_mode: String,
    // Reconnect support
    pub reconnect_enabled: bool,
    pub reconnect_max_attempts: u32,
    pub reconnect_base_delay_ms: u64,
    pub reconnect_max_delay_ms: u64,
    pub reconnect_attempt: u32,
    pub uri: Option<String>,
    pub db: Option<String>,
    pub saved_token: Option<String>,
    // Flag to signal the reconnect loop to stop (e.g. on explicit disconnect)
    pub stop_requested: bool,
    /// When set, the connection loop reconnects once even if auto-reconnect is off.
    /// Used for token-swap rebuild flows (anonymous → authenticated).
    pub force_reconnect: bool,
    /// Tracks whether this client has ever successfully connected.
    /// Used to emit "connected" vs "reconnected" events correctly.
    pub has_ever_connected: bool,
    /// Decoded subscribed rows (see `client_cache` module).
    pub cache: ClientCache,
}

impl ClientState {
    pub fn new() -> Self {
        ClientState {
            connected: false,
            last_error: String::new(),
            events: VecDeque::new(),
            events_dropped: 0,
            meta_events_enabled: false,
            task: None,
            outgoing: None,
            retry_queue: VecDeque::new(),
            next_query_set_id: 1,
            subscription_sql_to_qid: HashMap::new(),
            subscription_qid_to_sql: HashMap::new(),
            subscription_qid_to_sql_array: HashMap::new(),
            pending_requests: HashMap::new(),
            default_request_timeout_ms: 0,
            table_schemas: HashMap::new(),
            struct_schemas: HashMap::new(),
            reducer_error_schemas: HashMap::new(),
            seen_subscribe_applied: HashSet::new(),
            cached_token: None,
            compression_mode: String::from("none"),
            reconnect_enabled: false,
            reconnect_max_attempts: 10,
            reconnect_base_delay_ms: 1000,
            reconnect_max_delay_ms: 30000,
            reconnect_attempt: 0,
            uri: None,
            db: None,
            saved_token: None,
            stop_requested: false,
            force_reconnect: false,
            has_ever_connected: false,
            cache: ClientCache::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Store a string in thread-local storage and return a raw C pointer valid
/// for the duration of the current FFI call.
/// Unused after extgen: generated FFI uses `gm_ext_wire::store_tls_string`.
#[allow(dead_code)]
pub fn store_tls_string(s: String) -> *const c_char {
    let c = CString::new(s).unwrap_or_else(|_| CString::new("<invalidutf8>").unwrap());
    let p = c.as_ptr();
    TLS_RETURN.with(|cell| {
        *cell.lock() = Some(c);
    });
    p
}

/// Push a JSON event onto the client's event queue (stored as Value; no string yet).
pub fn emit_event(entry: &Arc<Mutex<ClientState>>, v: Value) {
    let mut st = entry.lock();
    while st.events.len() >= MAX_PENDING_EVENTS {
        st.events.pop_front();
        st.events_dropped = st.events_dropped.saturating_add(1);
    }
    st.events.push_back(v);
}

/// Push a noisy/meta event only when `meta_events_enabled` is set.
pub fn emit_meta_event(entry: &Arc<Mutex<ClientState>>, v: Value) {
    if entry.lock().meta_events_enabled {
        emit_event(entry, v);
    }
}

/// Try to send bytes through the outgoing WebSocket channel.
/// Returns `true` if the send was accepted, `false` otherwise.
pub fn try_send_outgoing(entry: &Arc<Mutex<ClientState>>, bytes: Vec<u8>) -> bool {
    let tx_opt = {
        let s = entry.lock();
        s.outgoing.clone()
    };
    if let Some(mut tx) = tx_opt {
        tx.try_send(bytes).is_ok()
    } else {
        false
    }
}

/// Attempt reconnect with exponential backoff.
///
/// - **1st attempt**: immediate (0ms delay) — handles transient disconnects quickly.
/// - **2nd+ attempts**: `base_delay * 2^(attempt-1)`, capped at `max_delay_ms`.
///
/// Returns `true` if we should continue the reconnect loop, `false` if we should give up.
pub async fn attempt_reconnect(entry: &Arc<Mutex<ClientState>>) -> bool {
    let (reconnect, attempt, max_att, base_delay, max_delay) = {
        let s = entry.lock();
        (
            s.reconnect_enabled,
            s.reconnect_attempt,
            s.reconnect_max_attempts,
            s.reconnect_base_delay_ms,
            s.reconnect_max_delay_ms,
        )
    };
    if !reconnect {
        emit_event(
            entry,
            json!({"type":"reconnect_failed","contract_version": CONTRACT_VERSION, "payload": {"reason": "reconnect not enabled"}}),
        );
        return false;
    }
    if attempt >= max_att {
        emit_event(
            entry,
            json!({"type":"reconnect_failed","contract_version": CONTRACT_VERSION, "payload": {"reason": "max attempts reached", "attempt": attempt, "max_attempts": max_att}}),
        );
        return false;
    }
    // Check if stop was requested
    if entry.lock().stop_requested {
        return false;
    }

    let att;
    let delay_ms;
    {
        let mut s = entry.lock();
        s.reconnect_attempt += 1;
        att = s.reconnect_attempt;
    }

    // First attempt: immediate (0ms). Subsequent: exponential backoff capped at max_delay.
    if att == 1 {
        delay_ms = 0;
    } else {
        // att >= 2: delay = base_delay * 2^(att-2), capped at max_delay
        // att=2 → base_delay, att=3 → 2*base_delay, att=4 → 4*base_delay, ...
        let exp = att.saturating_sub(2);
        delay_ms = base_delay
            .saturating_mul(1u64 << exp.min(30))
            .min(max_delay);
    }

    emit_event(
        entry,
        json!({"type":"reconnecting","contract_version": CONTRACT_VERSION, "attempt": att, "max_attempts": max_att, "delay_ms": delay_ms}),
    );

    if delay_ms > 0 {
        sleep(Duration::from_millis(delay_ms)).await;
    }

    // Check again after sleeping
    if entry.lock().stop_requested {
        return false;
    }
    true
}

/// Re-send all subscriptions after a successful reconnect.
pub fn resubscribe_all(entry: &Arc<Mutex<ClientState>>) {
    let subs: Vec<(u32, String)> = {
        let s = entry.lock();
        s.subscription_qid_to_sql
            .iter()
            .map(|(qid, sql)| (*qid, sql.clone()))
            .collect()
    };
    let batch_subs: Vec<(u32, Vec<String>)> = {
        let s = entry.lock();
        s.subscription_qid_to_sql_array
            .iter()
            .map(|(qid, arr)| (*qid, arr.clone()))
            .collect()
    };

    if subs.is_empty() && batch_subs.is_empty() {
        return;
    }

    // Clear seen_subscribe_applied so server's SubscribeApplied passes through.
    // Clear retry_queue since after reconnect all subs are re-sent anyway,
    // and any pending Unsubscribe/CallReducer for the old connection is now stale.
    // Clear pending_requests (reducer/procedure calls) — they must NOT be auto-retried
    // on reconnect because they are not idempotent. The GML layer handles queuing.
    {
        let mut s = entry.lock();
        s.seen_subscribe_applied.clear();
        s.retry_queue.clear();
        s.pending_requests.clear();
    }

    use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
    use spacetimedb_sats::bsatn;

    // Re-send single-query subscriptions
    for (qid, sql) in subs {
        if sql.starts_with("__BATCH__:") {
            continue; // handled below via batch_subs
        }
        let subscribe = ws_v2::ClientMessage::Subscribe(ws_v2::Subscribe {
            request_id: qid,
            query_set_id: spacetimedb_client_api_messages::websocket::common::QuerySetId {
                id: qid,
            },
            query_strings: vec![sql.clone().into_boxed_str()].into_boxed_slice(),
        });
        if let Ok(bytes) = bsatn::to_vec(&subscribe) {
            if !try_send_outgoing(entry, bytes.clone()) {
                let mut s = entry.lock();
                s.retry_queue.push_back((bytes, 0));
            }
        }
        emit_event(
            entry,
            json!({"type":"resubscribe_sent","contract_version": CONTRACT_VERSION, "query_set_id": qid, "sql": sql}),
        );
    }

    // Re-send batch subscriptions
    for (qid, sql_array) in batch_subs {
        let subscribe = ws_v2::ClientMessage::Subscribe(ws_v2::Subscribe {
            request_id: qid,
            query_set_id: spacetimedb_client_api_messages::websocket::common::QuerySetId {
                id: qid,
            },
            query_strings: sql_array
                .iter()
                .map(|s| s.clone().into_boxed_str())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        });
        if let Ok(bytes) = bsatn::to_vec(&subscribe) {
            if !try_send_outgoing(entry, bytes.clone()) {
                let mut s = entry.lock();
                s.retry_queue.push_back((bytes, 0));
            }
        }
        emit_event(
            entry,
            json!({"type":"resubscribe_sent","contract_version": CONTRACT_VERSION, "query_set_id": qid, "sql": "BATCH_SUBSCRIBE", "query_count": sql_array.len()}),
        );
    }
}

// ---------------------------------------------------------------------------
// Handle management helpers (used by FFI functions)
// ---------------------------------------------------------------------------

/// Allocate a new handle and ClientState, returning the handle value.
pub fn create_client() -> c_double {
    let mut n = NEXT_HANDLE.lock();
    let h = *n;
    *n += 1;
    let state = ClientState::new();
    HANDLES.lock().insert(h, Arc::new(Mutex::new(state)));
    h as c_double
}

/// Destroy a client by handle, aborting its async task if any.
/// Returns 0.0 on success, -1.0 if handle not found.
pub fn destroy_client(handle: c_double) -> c_double {
    let h = handle as Handle;
    let entry_opt = HANDLES.lock().remove(&h);
    let removed = if let Some(entry) = entry_opt {
        let mut s = entry.lock();
        s.cache.clear();
        if let Some(task) = s.task.take() {
            task.abort();
        }
        true
    } else {
        false
    };
    if removed {
        0.0
    } else {
        -1.0
    }
}

/// Look up an `Arc<Mutex<ClientState>>` by handle.
/// Returns `None` if the handle doesn't exist.
pub fn get_client(handle: c_double) -> Option<Arc<Mutex<ClientState>>> {
    let h = handle as Handle;
    let map = HANDLES.lock();
    map.get(&h).cloned()
}
