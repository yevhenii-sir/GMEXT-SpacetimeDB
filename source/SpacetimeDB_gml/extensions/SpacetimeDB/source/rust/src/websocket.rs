//! WebSocket connection management.
//!
//! Handles the full connection lifecycle: URI scheme normalization, compression
//! negotiation, token acquisition, WebSocket handshake with protocol headers,
//! Brotli/Gzip decompression, BSATN message parsing, idle-timeout keepalive
//! (Ping/Pong), reconnect with fixed interval, and the outgoing message
//! write loop.
//!
//! Key improvements ported from the official `spacetimedb-sdk`:
//! - Idle-timeout keepalive (Ping/Pong) to detect half-open connections
//! - Gzip decompression alongside Brotli
//! - Compression-tag constants from `spacetimedb-client-api-messages`
//! - Compression negotiation via `?compression=` URI parameter
//! - URI scheme normalization (`http` → `ws`, `https` → `wss`)

use crate::gml_client::{
    attempt_reconnect, emit_event, resubscribe_all, ClientState, CONTRACT_VERSION, GLOBAL_RUNTIME,
};
use crate::json_bridge::handle_server_message;
use bytes::Bytes;
use futures::channel::mpsc as fc_mpsc;
use futures::{SinkExt, StreamExt, TryStreamExt};
use parking_lot::Mutex;
use serde_json::json;
use spacetimedb_client_api_messages::websocket::common;
use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
use spacetimedb_client_api_messages::websocket::v3 as ws_v3;
use spacetimedb_sats::bsatn;
use std::io::Read;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::{sleep, Instant};
use tokio_tungstenite::{
    connect_async_with_config,
    tungstenite::{
        client::IntoClientRequest,
        protocol::{Message as WsMsg, WebSocketConfig},
    },
    MaybeTlsStream, WebSocketStream,
};

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Idle timeout before sending a keepalive Ping.
/// Matches the SDK's default of 30 seconds.
const IDLE_TIMEOUT: Duration = Duration::from_secs(30);

// ---------------------------------------------------------------------------
// URI helpers (from SDK patterns)
// ---------------------------------------------------------------------------

/// Normalize URI scheme: `http` → `ws`, `https` → `wss`, `ws`/`wss` unchanged.
/// Returns an error message for unknown schemes.
fn normalize_scheme(uri_str: &str) -> Result<String, String> {
    if let Ok(uri) = uri_str.parse::<http::Uri>() {
        let mut parts = uri.into_parts();
        let scheme = match parts.scheme.take() {
            Some(s) => match s.as_str() {
                "ws" | "wss" => s,
                "http" => "ws".parse().unwrap(),
                "https" => "wss".parse().unwrap(),
                unknown => return Err(format!("unknown URI scheme: {unknown}")),
            },
            None => "ws".parse().unwrap(),
        };
        parts.scheme = Some(scheme);
        if let Ok(normalized) = http::Uri::from_parts(parts) {
            return Ok(normalized.to_string());
        }
    }
    // Fallback: string replacement for simple cases
    let s = uri_str.trim_end_matches('/');
    if s.starts_with("http://") {
        Ok(s.replacen("http://", "ws://", 1))
    } else if s.starts_with("https://") {
        Ok(s.replacen("https://", "wss://", 1))
    } else {
        Ok(s.to_string())
    }
}

/// Build the full WebSocket URI with compression negotiation parameter.
fn build_ws_uri(base: &str, db: &str, compression: &str) -> String {
    let base = base.trim_end_matches('/');
    let mut uri = format!("{}/v1/database/{}/subscribe", base, db);

    // Add compression negotiation parameter (tells the server which
    // compression to use for server→client messages).
    match compression.to_lowercase().as_str() {
        "brotli" => uri.push_str("?compression=Brotli"),
        "gzip" => uri.push_str("?compression=Gzip"),
        "none" | "" => uri.push_str("?compression=None"),
        _ => uri.push_str("?compression=Brotli"), // default to brotli
    }

    uri
}

/// Decompress a server message payload based on the compression tag byte.
/// Uses the official constants from `spacetimedb-client-api-messages`.
/// Returns the decompressed bytes, or an error description.
fn decompress_message(raw: &[u8]) -> Result<Vec<u8>, String> {
    if raw.is_empty() {
        return Err("empty message (no compression tag)".to_string());
    }

    let tag = raw[0];
    let payload = &raw[1..];

    match tag {
        _ if tag == common::SERVER_MSG_COMPRESSION_TAG_NONE => Ok(payload.to_vec()),
        _ if tag == common::SERVER_MSG_COMPRESSION_TAG_BROTLI => {
            let mut decompressed = Vec::new();
            let mut dec = brotli::Decompressor::new(std::io::Cursor::new(payload), 4096);
            match dec.read_to_end(&mut decompressed) {
                Ok(_) => Ok(decompressed),
                Err(e) => Err(format!("Brotli decode error: {}", e)),
            }
        }
        _ if tag == common::SERVER_MSG_COMPRESSION_TAG_GZIP => {
            let mut decompressed = Vec::new();
            let mut dec = flate2::read::GzDecoder::new(payload);
            match dec.read_to_end(&mut decompressed) {
                Ok(_) => Ok(decompressed),
                Err(e) => Err(format!("Gzip decode error: {}", e)),
            }
        }
        _ => Err(format!("unknown compression tag: {tag:#x}")),
    }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Spawn the WebSocket connection loop on the global tokio runtime.
///
/// This replaces the old inline `GLOBAL_RUNTIME.spawn(async move { ... })` block
/// in `stdb_connect`. The returned `JoinHandle` should be stored in
/// `ClientState::task`.
pub fn spawn_connection_loop(entry: Arc<Mutex<ClientState>>) -> tokio::task::JoinHandle<()> {
    GLOBAL_RUNTIME.spawn(async move {
        connection_loop(&entry).await;
    })
}

// ---------------------------------------------------------------------------
// Main connection loop (with reconnect)
// ---------------------------------------------------------------------------

async fn connection_loop(entry: &Arc<Mutex<ClientState>>) {
    'reconnect: loop {
        // Check if stop was requested (e.g. explicit disconnect)
        if entry.lock().stop_requested {
            break 'reconnect;
        }

        // Read connection params from state (supports reconnect)
        let (raw_uri, db_str, compression, reconnect_token) = {
            let s = entry.lock();
            let uri_str = s.uri.clone().unwrap_or_default();
            let db = s.db.clone().unwrap_or_default();
            let comp = s.compression_mode.clone();
            let tok = s.saved_token.clone().or_else(|| s.cached_token.clone());
            (uri_str, db, comp, tok)
        };

        // Normalize scheme (http→ws, https→wss)
        let normalized_uri = match normalize_scheme(&raw_uri) {
            Ok(u) => u,
            Err(e) => {
                {
                    let mut s = entry.lock();
                    s.last_error = format!("invalid uri: {}", e);
                }
                emit_event(
                    entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": format!("invalid uri: {}", e)}}),
                );
                if attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
        };

        // Build full WS URI with compression negotiation
        let full_ws_uri = build_ws_uri(&normalized_uri, &db_str, &compression);

        // Validate URI
        if full_ws_uri.parse::<http::Uri>().is_err() {
            {
                let mut s = entry.lock();
                s.last_error = "invalid uri".to_string();
            }
            emit_event(
                entry,
                json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": "invalid uri"}}),
            );
            if attempt_reconnect(entry).await {
                continue 'reconnect;
            }
            break 'reconnect;
        }

        // Acquire token if needed (anonymous connect)
        if reconnect_token.is_none() {
            let cached = entry.lock().cached_token.clone();
            if cached.is_none() {
                acquire_token_anonymous(entry, &full_ws_uri).await;
            }
        }

        // Build the authenticated WebSocket request
        let req_res = IntoClientRequest::into_client_request(full_ws_uri.as_str());
        let mut req = match req_res {
            Ok(r) => r,
            Err(e) => {
                {
                    let mut s = entry.lock();
                    s.last_error = format!("request build error: {e}");
                }
                emit_event(
                    entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": format!("request build failed: {}", e)}}),
                );
                if attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
        };

        // Set protocol + auth headers
        {
            use tokio_tungstenite::tungstenite::http::header::HeaderName;
            use tokio_tungstenite::tungstenite::http::HeaderValue;
            req.headers_mut().insert(
                HeaderName::from_static("sec-websocket-protocol"),
                HeaderValue::from_static(ws_v3::BIN_PROTOCOL),
            );

            let chosen_token: Option<String> = if let Some(t) = reconnect_token.as_ref() {
                Some(t.clone())
            } else {
                entry.lock().cached_token.clone()
            };
            if let Some(ref tok) = chosen_token {
                let auth = format!("Bearer {}", tok);
                req.headers_mut().insert(
                    HeaderName::from_static("authorization"),
                    HeaderValue::from_str(&auth).unwrap(),
                );
            }
        }

        // Connect
        let connect_res = connect_async_with_config(
            req,
            Some(
                WebSocketConfig::default()
                    .max_frame_size(None)
                    .max_message_size(None),
            ),
            false,
        )
        .await;

        let ws_stream = match connect_res {
            Ok((stream, _)) => stream,
            Err(e) => {
                {
                    let mut s = entry.lock();
                    s.last_error = format!("ws connect error: {e}");
                }
                emit_event(
                    entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": "connect failed", "error": format!("{}", e)}}),
                );
                if attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
        };

        // Set up outgoing channel
        let (out_tx, out_rx): (fc_mpsc::Sender<Vec<u8>>, fc_mpsc::Receiver<Vec<u8>>) =
            fc_mpsc::channel(32);
        {
            let mut s = entry.lock();
            s.outgoing = Some(out_tx.clone());
            s.connected = true;
        }

        // Determine if this is a reconnect or first-ever connection.
        // If reconnect_attempt > 0 but we've never successfully connected before,
        // this is actually the first successful connection (initial connect failed then
        // succeeded via the reconnect loop) — emit "connected" instead of "reconnected".
        let (is_reconnect, has_ever_connected) = {
            let s = entry.lock();
            (s.reconnect_attempt > 0, s.has_ever_connected)
        };

        if is_reconnect && has_ever_connected {
            // True reconnect: re-send subscriptions and notify
            resubscribe_all(entry);
            emit_event(
                entry,
                json!({"type": "reconnected", "contract_version": CONTRACT_VERSION}),
            );
            entry.lock().reconnect_attempt = 0;
        } else {
            // First successful connection (or initial connect that failed then succeeded)
            let (uri_disp, db_disp) = {
                let s = entry.lock();
                (
                    s.uri.clone().unwrap_or_default(),
                    s.db.clone().unwrap_or_default(),
                )
            };
            emit_event(
                entry,
                json!({"type": "connected", "contract_version": CONTRACT_VERSION, "payload": {"uri": uri_disp, "db": db_disp}}),
            );
            entry.lock().has_ever_connected = true;
            entry.lock().reconnect_attempt = 0;
        }

        // Run the message loop (single select loop with idle timeout)
        let loop_result = message_loop(entry, ws_stream, out_rx).await;

        // Clean up
        {
            let mut s = entry.lock();
            s.connected = false;
            s.outgoing = None;
            s.cache.clear();
        }

        match loop_result {
            LoopExit::CleanClose => {
                emit_event(
                    entry,
                    json!({"type":"disconnected","contract_version": CONTRACT_VERSION}),
                );
                if take_force_reconnect(entry) || attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
            LoopExit::Error(msg) => {
                emit_event(
                    entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": msg}}),
                );
                if take_force_reconnect(entry) || attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
            LoopExit::IdleTimeout => {
                emit_event(
                    entry,
                    json!({"type":"idle_timeout","contract_version": CONTRACT_VERSION, "payload": {"message": "connection timed out (no pong received)"}}),
                );
                if take_force_reconnect(entry) || attempt_reconnect(entry).await {
                    continue 'reconnect;
                }
                break 'reconnect;
            }
        }
    } // end 'reconnect loop
}

/// Consume a one-shot `force_reconnect` request (token swap).
fn take_force_reconnect(entry: &Arc<Mutex<ClientState>>) -> bool {
    let mut s = entry.lock();
    if s.force_reconnect {
        s.force_reconnect = false;
        // Ensure the successful connect path treats this as a reconnect so
        // subscriptions are restored via resubscribe_all.
        s.reconnect_attempt = s.reconnect_attempt.max(1);
        true
    } else {
        false
    }
}

// ---------------------------------------------------------------------------
// Message loop result
// ---------------------------------------------------------------------------

enum LoopExit {
    CleanClose,
    Error(String),
    IdleTimeout,
}

// ---------------------------------------------------------------------------
// Message loop with idle timeout (from SDK pattern)
// ---------------------------------------------------------------------------

/// Single `tokio::select!` loop handling incoming messages, outgoing messages,
/// and idle-timeout keepalive (Ping/Pong).
///
/// This is modeled after the SDK's `WsConnection::message_loop` but adapted
/// for our GML bridge architecture. The retry queue is handled by a separate
/// task since it's our custom feature not present in the SDK.
async fn message_loop(
    entry: &Arc<Mutex<ClientState>>,
    ws_stream: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
    mut out_rx: fc_mpsc::Receiver<Vec<u8>>,
) -> LoopExit {
    let (mut sink, mut read) = ws_stream.split();

    // Idle timeout / keepalive state (from SDK)
    let mut idle_timeout_interval =
        tokio::time::interval_at(Instant::now() + IDLE_TIMEOUT, IDLE_TIMEOUT);
    let mut idle = true;
    let mut want_pong = false;

    // Retry task: periodically retries failed outgoing messages
    let retry_task = {
        let entry = Arc::clone(entry);
        tokio::spawn(async move {
            const MAX_ATTEMPTS: u32 = 5;
            loop {
                let maybe_item: Option<(Vec<u8>, u32)>;
                let tx_clone_opt: Option<fc_mpsc::Sender<Vec<u8>>>;
                {
                    let mut s = entry.lock();
                    if !s.connected {
                        break;
                    }
                    tx_clone_opt = s.outgoing.clone();
                    maybe_item = s.retry_queue.pop_front();
                }

                if let Some((bytes, attempts)) = maybe_item {
                    if let Some(mut _tx) = tx_clone_opt {
                        if _tx.try_send(bytes.clone()).is_err() {
                            {
                                let mut s = entry.lock();
                                let next = attempts.saturating_add(1);
                                if next >= MAX_ATTEMPTS {
                                    emit_event(
                                        &entry,
                                        json!({"type":"retry_exhausted","contract_version": CONTRACT_VERSION}),
                                    );
                                } else {
                                    s.retry_queue.push_back((bytes.clone(), next));
                                }
                            }
                            let backoff_ms = 100u64.saturating_mul(1u64 << (attempts.min(6)));
                            sleep(Duration::from_millis(backoff_ms)).await;
                        } else {
                            emit_event(
                                &entry,
                                json!({"type":"retry_sent","contract_version": CONTRACT_VERSION}),
                            );
                        }
                    } else {
                        {
                            let mut s = entry.lock();
                            s.retry_queue.push_front((bytes, attempts));
                        }
                        sleep(Duration::from_millis(200)).await;
                    }
                } else {
                    sleep(Duration::from_millis(100)).await;
                }
            }
        })
    };

    let result = loop {
        tokio::select! {
            // Branch 1: incoming WebSocket messages
            incoming = read.try_next() => match incoming {
                Err(tokio_tungstenite::tungstenite::error::Error::ConnectionClosed) | Ok(None) => {
                    break LoopExit::CleanClose;
                }
                Err(e) => {
                    break LoopExit::Error(format!("ws error: {}", e));
                }
                Ok(Some(WsMsg::Binary(buf))) => {
                    idle = false;
                    if buf.is_empty() {
                        continue;
                    }

                    match decompress_message(&buf) {
                        Ok(decompressed) => {
                            // v3 protocol: a single frame may contain multiple
                            // BSATN-encoded ServerMessage values concatenated together.
                            // We decode them in a loop, advancing through the buffer.
                            // Using from_reader with a mutable slice reference so the
                            // reader position advances after each message is consumed.
                            let mut reader = decompressed.as_slice();
                            let total_len = reader.len();
                            let mut messages_in_batch = 0u32;

                            while !reader.is_empty() {
                                match bsatn::from_reader::<ws_v2::ServerMessage>(&mut reader) {
                                    Ok(server_msg) => {
                                        handle_server_message(entry, server_msg);
                                        messages_in_batch += 1;
                                    }
                                    Err(e) => {
                                        // If we've already processed at least one message in this batch,
                                        // report a warning but don't abort — partial progress is better than none.
                                        let consumed = total_len - reader.len();
                                        if messages_in_batch > 0 {
                                            emit_event(
                                                entry,
                                                json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": format!(
                                                    "bsatn decode error in batch at byte {}/{}: {}",
                                                    consumed, total_len, e
                                                )}}),
                                            );
                                        } else {
                                            emit_event(
                                                entry,
                                                json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": format!("bsatn decode error: {}", e)}}),
                                            );
                                        }
                                        break;
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            emit_event(
                                entry,
                                json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": e}}),
                            );
                        }
                    }
                }
                Ok(Some(WsMsg::Ping(_payload))) => {
                    log::trace!("received ping");
                    idle = false;
                    // No need to explicitly respond with a Pong,
                    // tungstenite handles this automatically.
                    // See https://github.com/snapview/tokio-tungstenite/issues/88
                }
                Ok(Some(WsMsg::Pong(_payload))) => {
                    log::trace!("received pong");
                    idle = false;
                    want_pong = false;
                }
                Ok(Some(WsMsg::Close(_))) => {
                    break LoopExit::CleanClose;
                }
                Ok(Some(_other)) => {
                    idle = false;
                    // Ignore Text, Frame, etc.
                }
            },

            // Branch 2: idle timeout / keepalive (from SDK)
            _ = idle_timeout_interval.tick() => {
                if std::mem::replace(&mut idle, true) {
                    if want_pong {
                        // No data received while waiting for pong — connection is dead
                        log::warn!("Connection timed out (idle, no pong received)");
                        break LoopExit::IdleTimeout;
                    }

                    log::trace!("sending client ping (idle timeout)");
                    let ping = WsMsg::Ping(Bytes::new());
                    if let Err(e) = sink.send(ping).await {
                        break LoopExit::Error(format!("error sending ping: {:?}", e));
                    }
                    want_pong = true;
                }
            },

            // Branch 3: outgoing messages
            outgoing = out_rx.next() => match outgoing {
                Some(bytes) => {
                    let msg = WsMsg::Binary(bytes.into());
                    if let Err(e) = sink.send(msg).await {
                        break LoopExit::Error(format!("error sending outgoing message: {:?}", e));
                    }
                }
                None => {
                    // Channel closed — no more outgoing messages
                    let _ = sink.close().await;
                    break LoopExit::CleanClose;
                }
            },
        }
    };

    retry_task.abort();
    result
}

// ---------------------------------------------------------------------------
// Anonymous token acquisition
// ---------------------------------------------------------------------------

/// Connect without auth, read the `InitialConnection` message to obtain
/// a token, then close the temporary connection.
async fn acquire_token_anonymous(entry: &Arc<Mutex<ClientState>>, full_ws_uri: &str) {
    use std::time::Duration as StdDuration;
    use tokio::time::timeout;

    if let Ok(mut one_req) = IntoClientRequest::into_client_request(full_ws_uri) {
        one_req.headers_mut().insert(
            http::header::SEC_WEBSOCKET_PROTOCOL,
            http::header::HeaderValue::from_static(ws_v3::BIN_PROTOCOL),
        );

        let unauth_res = connect_async_with_config(
            one_req,
            Some(
                WebSocketConfig::default()
                    .max_frame_size(None)
                    .max_message_size(None),
            ),
            false,
        )
        .await;

        if let Ok((unauth_stream, _)) = unauth_res {
            let (mut unauth_sink, mut unauth_read) = unauth_stream.split();
            if let Ok(Some(Ok(WsMsg::Binary(buf)))) =
                timeout(StdDuration::from_secs(2), unauth_read.next()).await
            {
                if let Ok(decompressed) = decompress_message(&buf) {
                    if let Ok(ws_v2::ServerMessage::InitialConnection(ic)) =
                        bsatn::from_slice::<ws_v2::ServerMessage>(&decompressed)
                    {
                        let token = ic.token.to_string();
                        {
                            let mut s = entry.lock();
                            s.cached_token = Some(token.clone());
                        }
                        emit_event(
                            entry,
                            json!({"type":"connect_token_acquired","contract_version": CONTRACT_VERSION, "payload": {"token_length": token.len()}}),
                        );
                    }
                }
                // ignore decompression/parse errors during token acquisition
            }
            let _ = unauth_sink.close().await;
        }
    }
}
