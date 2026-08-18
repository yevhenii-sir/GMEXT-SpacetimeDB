//! MANUAL: SpacetimeDB FFI bodies adapted into extgen user stubs.
//! Generated stubs were replaced once; extgen will NOT overwrite this file (IfMissing).

#![allow(unused_variables)]

use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;

use crate::client_cache::{ensure_schema_primary_key, pk_key_from_value};
use crate::gm_wire_json::{
    gm_array_to_json, gm_struct_to_json, gmvalue_to_json, value_to_struct_stream,
    values_to_array_stream,
};
use crate::gml_client::{
    emit_event, emit_meta_event, get_client, try_send_outgoing, CONTRACT_VERSION,
};
use gm_ext_wire::{ArrayStream, GMBuffer, GMValueOwned, StructStream};

fn optional_token(s: &str) -> Option<String> {
    if s.is_empty() {
        None
    } else {
        Some(s.to_string())
    }
}

fn take_dropped_event(st: &mut crate::gml_client::ClientState) -> Option<serde_json::Value> {
    if st.events_dropped == 0 {
        return None;
    }
    let dropped = st.events_dropped;
    st.events_dropped = 0;
    Some(json!({
        "type": "events_dropped",
        "contract_version": CONTRACT_VERSION,
        "payload": { "count": dropped }
    }))
}

pub fn stdb_ping() -> String {
    "ok".to_string()
}

pub fn stdb_create_client() -> f64 {
    crate::gml_client::create_client()
}

pub fn stdb_destroy_client(handle: f64) -> f64 {
    crate::gml_client::destroy_client(handle)
}

pub fn stdb_connect_simple(
    handle: f64,
    uri: &str,
    db_name_or_address: &str,
    auth_token_or_null: &str,
) -> f64 {
    if uri.is_empty() || db_name_or_address.is_empty() {
        return -1.0;
    }

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    {
        let mut s = entry.lock();
        s.uri = Some(uri.to_string());
        s.db = Some(db_name_or_address.to_string());
        s.saved_token = optional_token(auth_token_or_null);
        s.reconnect_attempt = 0;
        s.stop_requested = false;
        s.has_ever_connected = false;
    }

    let task = crate::websocket::spawn_connection_loop(Arc::clone(&entry));
    entry.lock().task = Some(task);
    0.0
}

pub fn stdb_disconnect(handle: f64) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let mut s = entry.lock();
    s.stop_requested = true;
    s.reconnect_enabled = false;
    s.outgoing = None;
    if let Some(task) = s.task.take() {
        task.abort();
    }
    s.connected = false;
    s.cache.clear();
    s.events.push_back(json!({"type":"disconnected"}));
    0.0
}

pub fn stdb_reconnect_with_token(handle: f64, new_token: &str) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let need_spawn = {
        let mut s = entry.lock();
        s.saved_token = optional_token(new_token);
        s.cached_token = None;
        s.force_reconnect = true;
        s.reconnect_attempt = s.reconnect_attempt.max(1);
        s.stop_requested = false;
        let _ = s.outgoing.take();
        let task_alive = s.task.as_ref().is_some_and(|t| !t.is_finished());
        !s.connected && !task_alive && s.uri.is_some() && s.db.is_some()
    };

    if need_spawn {
        let task = crate::websocket::spawn_connection_loop(Arc::clone(&entry));
        entry.lock().task = Some(task);
    }

    emit_event(
        &entry,
        json!({"type":"token_swap_requested","contract_version": CONTRACT_VERSION}),
    );
    0.0
}

pub fn stdb_poll_event(handle: f64) -> StructStream {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return StructStream::new(),
    };

    let mut st = entry.lock();
    if let Some(dropped) = take_dropped_event(&mut st) {
        return value_to_struct_stream(&dropped);
    }
    match st.events.pop_front() {
        Some(ev) => value_to_struct_stream(&ev),
        None => StructStream::new(),
    }
}

pub fn stdb_poll_events_batch(handle: f64) -> ArrayStream {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return ArrayStream::new(),
    };

    let mut st = entry.lock();
    let mut values = Vec::new();
    if let Some(dropped) = take_dropped_event(&mut st) {
        values.push(dropped);
    }
    while let Some(ev) = st.events.pop_front() {
        values.push(ev);
    }
    values_to_array_stream(values)
}

pub fn stdb_get_last_error(handle: f64) -> String {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return String::new(),
    };
    let err = entry.lock().last_error.clone();
    err
}

pub fn stdb_debug_state(handle: f64) -> StructStream {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => {
            let mut s = StructStream::new();
            s.add_string("error", "invalid client handle");
            return s;
        }
    };

    let st = entry.lock();
    let mut subs_arr = serde_json::Map::new();
    for (qid, sql) in &st.subscription_qid_to_sql {
        subs_arr.insert(qid.to_string(), serde_json::Value::String(sql.clone()));
    }
    let mut batch_arr = serde_json::Map::new();
    for (qid, sql_array) in &st.subscription_qid_to_sql_array {
        batch_arr.insert(
            qid.to_string(),
            serde_json::Value::Array(
                sql_array
                    .iter()
                    .map(|v| serde_json::Value::String(v.clone()))
                    .collect(),
            ),
        );
    }

    let state = json!({
        "connected": st.connected,
        "has_ever_connected": st.has_ever_connected,
        "reconnect_attempt": st.reconnect_attempt,
        "reconnect_enabled": st.reconnect_enabled,
        "reconnect_base_delay_ms": st.reconnect_base_delay_ms,
        "reconnect_max_delay_ms": st.reconnect_max_delay_ms,
        "reconnect_max_attempts": st.reconnect_max_attempts,
        "next_query_set_id": st.next_query_set_id,
        "subscription_qid_to_sql": subs_arr,
        "subscription_qid_to_sql_array": batch_arr,
        "subscription_sql_to_qid_count": st.subscription_sql_to_qid.len(),
        "pending_events_count": st.events.len(),
        "events_dropped": st.events_dropped,
        "meta_events_enabled": st.meta_events_enabled,
        "retry_queue_count": st.retry_queue.len(),
        "outgoing_present": st.outgoing.is_some(),
        "compression_mode": st.compression_mode,
    });
    drop(st);
    value_to_struct_stream(&state)
}

pub fn stdb_set_auto_reconnect(
    handle: f64,
    enabled: f64,
    max_attempts: f64,
    base_delay_ms: f64,
    max_delay_ms: f64,
) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let mut s = entry.lock();
    s.reconnect_enabled = enabled != 0.0;
    if max_attempts > 0.0 {
        s.reconnect_max_attempts = max_attempts as u32;
    }
    if base_delay_ms > 0.0 {
        s.reconnect_base_delay_ms = base_delay_ms as u64;
    }
    if max_delay_ms > 0.0 {
        s.reconnect_max_delay_ms = max_delay_ms as u64;
    }
    0.0
}

pub fn stdb_set_compression_mode(handle: f64, mode: &str) -> f64 {
    let normalized = mode.to_lowercase();
    if !["none", "brotli", "gzip"].contains(&normalized.as_str()) {
        return -1.0;
    }

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };
    entry.lock().compression_mode = normalized;
    0.0
}

pub fn stdb_set_log_level(level: f64) -> f64 {
    crate::logger::set_log_level(level.clamp(0.0, 5.0) as u8);
    level
}

pub fn stdb_set_default_request_timeout_ms(handle: f64, timeout_ms: f64) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };
    entry.lock().default_request_timeout_ms = timeout_ms as i64;
    0.0
}

pub fn stdb_set_meta_events(handle: f64, enabled: f64) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };
    entry.lock().meta_events_enabled = enabled != 0.0;
    0.0
}

pub fn stdb_cancel_request(handle: f64, request_id: f64) -> f64 {
    let req = request_id as u32;
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let mut s = entry.lock();
    if s.pending_requests.remove(&req).is_some() {
        drop(s);
        emit_event(
            &entry,
            json!({"type":"request_cancelled","contract_version": CONTRACT_VERSION, "request_id": req}),
        );
        0.0
    } else {
        -1.0
    }
}

pub fn stdb_register_schema(
    handle: f64,
    table_name: String,
    schema: HashMap<String, GMValueOwned>,
) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let schema_val = ensure_schema_primary_key(gm_struct_to_json(&schema));
    {
        let mut s = entry.lock();
        s.table_schemas.insert(table_name.clone(), schema_val);
    }
    emit_meta_event(
        &entry,
        json!({"type":"schema_registered","contract_version": CONTRACT_VERSION, "payload": {"table": table_name}}),
    );
    0.0
}

pub fn stdb_register_schemas(handle: f64, all_schemas: HashMap<String, GMValueOwned>) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let parsed = gm_struct_to_json(&all_schemas);
    {
        let mut s = entry.lock();
        if let Some(obj) = parsed.as_object() {
            for (table, schema) in obj {
                s.table_schemas
                    .insert(table.clone(), ensure_schema_primary_key(schema.clone()));
            }
        } else {
            s.table_schemas
                .insert("__bulk__".to_string(), ensure_schema_primary_key(parsed));
        }
    }
    emit_meta_event(
        &entry,
        json!({"type":"schemas_registered","contract_version": CONTRACT_VERSION}),
    );
    0.0
}

pub fn stdb_register_struct_schema(
    handle: f64,
    struct_name: String,
    schema: Vec<GMValueOwned>,
) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let val = gm_array_to_json(&schema);
    entry
        .lock()
        .struct_schemas
        .insert(struct_name, val);
    0.0
}

pub fn stdb_register_reducer_error_schema(
    handle: f64,
    reducer_name: String,
    schema: GMValueOwned,
) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    // Keep string form for bsatn_json decoder (expects schema JSON text).
    let schema_json = match &schema {
        GMValueOwned::String(s) => s.clone(),
        other => gmvalue_to_json(other).to_string(),
    };

    {
        let mut s = entry.lock();
        s.reducer_error_schemas
            .insert(reducer_name.clone(), schema_json);
    }
    emit_meta_event(
        &entry,
        json!({"type":"reducer_error_schema_registered","contract_version": CONTRACT_VERSION, "payload": {"reducer": reducer_name}}),
    );
    0.0
}

pub fn stdb_subscribe_sql(handle: f64, sql: &str) -> f64 {
    if sql.is_empty() {
        return -1.0;
    }
    let sql_s = sql.to_string();

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let qid = {
        let mut s = entry.lock();
        if let Some(&existing_qid) = s.subscription_sql_to_qid.get(&sql_s) {
            return existing_qid as f64;
        }
        let q = s.next_query_set_id;
        s.next_query_set_id += 1;
        s.subscription_sql_to_qid.insert(sql_s.clone(), q);
        s.subscription_qid_to_sql.insert(q, sql_s.clone());
        q
    };

    {
        let outgoing_present = entry.lock().outgoing.is_some();
        if outgoing_present {
            use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
            use spacetimedb_sats::bsatn;

            let subscribe = ws_v2::ClientMessage::Subscribe(ws_v2::Subscribe {
                request_id: qid,
                query_set_id: spacetimedb_client_api_messages::websocket::common::QuerySetId {
                    id: qid,
                },
                query_strings: vec![sql_s.clone().into_boxed_str()].into_boxed_slice(),
            });

            if let Ok(bytes) = bsatn::to_vec(&subscribe) {
                let bytes_len = bytes.len();
                if !try_send_outgoing(&entry, bytes.clone()) {
                    let mut s = entry.lock();
                    s.retry_queue.push_back((bytes.clone(), 0));
                    drop(s);
                    emit_meta_event(
                        &entry,
                        json!({"type":"ws_tx_client_message","kind":"Subscribe","query_set_id": qid, "bytes_len": bytes_len}),
                    );
                    emit_meta_event(
                        &entry,
                        json!({"type":"queued_for_retry","query_set_id": qid}),
                    );
                }
            } else {
                emit_event(
                    &entry,
                    json!({"type":"error","message":"bsatn encode failed for subscribe"}),
                );
            }
        }
    }

    emit_meta_event(
        &entry,
        json!({"type":"subscribe_requested","query_set_id": qid, "sql": sql_s}),
    );
    qid as f64
}

pub fn stdb_subscribe_all(handle: f64, sqls: Vec<GMValueOwned>) -> f64 {
    let parsed: Vec<String> = sqls
        .into_iter()
        .filter_map(|v| match v {
            GMValueOwned::String(s) => Some(s),
            _ => None,
        })
        .collect();

    if parsed.is_empty() {
        return -1.0;
    }

    let dedup_key = format!("__BATCH__:{}", parsed.join("\n"));

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let qid = {
        let mut s = entry.lock();
        if let Some(&existing_qid) = s.subscription_sql_to_qid.get(&dedup_key) {
            return existing_qid as f64;
        }
        let q = s.next_query_set_id;
        s.next_query_set_id += 1;
        s.subscription_sql_to_qid.insert(dedup_key.clone(), q);
        s.subscription_qid_to_sql.insert(q, dedup_key);
        s.subscription_qid_to_sql_array.insert(q, parsed.clone());
        q
    };

    let boxed_queries: Vec<Box<str>> = parsed.into_iter().map(|s| s.into_boxed_str()).collect();

    {
        let outgoing_present = entry.lock().outgoing.is_some();
        if outgoing_present {
            use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
            use spacetimedb_sats::bsatn;

            let subscribe = ws_v2::ClientMessage::Subscribe(ws_v2::Subscribe {
                request_id: qid,
                query_set_id: spacetimedb_client_api_messages::websocket::common::QuerySetId {
                    id: qid,
                },
                query_strings: boxed_queries.into_boxed_slice(),
            });

            if let Ok(bytes) = bsatn::to_vec(&subscribe) {
                if !try_send_outgoing(&entry, bytes.clone()) {
                    let mut s = entry.lock();
                    s.retry_queue.push_back((bytes, 0));
                }
            }
        }
    }

    emit_meta_event(
        &entry,
        json!({"type":"subscribe_requested","query_set_id": qid, "sql":"BATCH"}),
    );
    qid as f64
}

pub fn stdb_unsubscribe_sql(handle: f64, query_set_id: f64) -> f64 {
    let qid = query_set_id as u32;
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    let sql_removed = {
        let mut s = entry.lock();
        let sql_opt = s.subscription_qid_to_sql.remove(&qid);
        if let Some(ref sql) = sql_opt {
            s.subscription_sql_to_qid.remove(sql);
        }
        s.subscription_qid_to_sql_array.remove(&qid);
        s.seen_subscribe_applied.remove(&qid);
        sql_opt
    };

    if sql_removed.is_none() {
        emit_event(
            &entry,
            json!({"type":"error","message":"unknown query_set_id","query_set_id": qid}),
        );
        return -1.0;
    }

    let is_connected = {
        let s = entry.lock();
        s.connected && s.outgoing.is_some()
    };

    if is_connected {
        use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
        use spacetimedb_sats::bsatn;

        let unsub = ws_v2::ClientMessage::Unsubscribe(ws_v2::Unsubscribe {
            request_id: qid,
            query_set_id: spacetimedb_client_api_messages::websocket::common::QuerySetId {
                id: qid,
            },
            flags: ws_v2::UnsubscribeFlags::Default,
        });

        if let Ok(bytes) = bsatn::to_vec(&unsub) {
            if !try_send_outgoing(&entry, bytes.clone()) {
                let mut s = entry.lock();
                s.retry_queue.push_back((bytes, 0));
                drop(s);
                emit_meta_event(
                    &entry,
                    json!({"type":"queued_for_retry","query_set_id": qid}),
                );
            }
        } else {
            emit_event(
                &entry,
                json!({"type":"error","message":"bsatn encode failed for unsubscribe"}),
            );
        }
    }

    emit_meta_event(
        &entry,
        json!({"type":"unsubscribe_requested","query_set_id": qid}),
    );
    0.0
}

pub fn stdb_call_reducer(
    handle: f64,
    name: String,
    request_id: f64,
    args: GMBuffer,
) -> f64 {
    if name.is_empty() {
        return -1.0;
    }
    let req_id = request_id as u32;
    let bytes_vec = args.as_slice().to_vec();

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    {
        let outgoing_present = entry.lock().outgoing.is_some();
        if outgoing_present {
            use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
            use spacetimedb_sats::bsatn;
            let call = ws_v2::ClientMessage::CallReducer(ws_v2::CallReducer {
                request_id: req_id,
                flags: ws_v2::CallReducerFlags::Default,
                reducer: name.clone().into_boxed_str(),
                args: bytes::Bytes::from(bytes_vec),
            });
            if let Ok(bytes) = bsatn::to_vec(&call) {
                if !try_send_outgoing(&entry, bytes.clone()) {
                    entry.lock().retry_queue.push_back((bytes, 0));
                    emit_meta_event(
                        &entry,
                        json!({"type":"queued_for_retry","contract_version": CONTRACT_VERSION, "request_id": req_id}),
                    );
                }
            } else {
                emit_event(
                    &entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": "bsatn encode failed for CallReducer"}}),
                );
            }
        }
    }

    entry.lock().pending_requests.insert(req_id, name.clone());
    emit_meta_event(
        &entry,
        json!({"type":"reducer_sent","contract_version": CONTRACT_VERSION, "request_id": req_id, "reducer": name}),
    );
    0.0
}

pub fn stdb_call_procedure(
    handle: f64,
    name: String,
    request_id: f64,
    args: GMBuffer,
) -> f64 {
    if name.is_empty() {
        return -1.0;
    }
    let req_id = request_id as u32;
    let bytes_vec = args.as_slice().to_vec();

    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };

    {
        let outgoing_present = entry.lock().outgoing.is_some();
        if outgoing_present {
            use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
            use spacetimedb_sats::bsatn;
            let call = ws_v2::ClientMessage::CallProcedure(ws_v2::CallProcedure {
                request_id: req_id,
                flags: ws_v2::CallProcedureFlags::default(),
                procedure: name.clone().into_boxed_str(),
                args: bytes::Bytes::from(bytes_vec),
            });
            if let Ok(bytes) = bsatn::to_vec(&call) {
                if !try_send_outgoing(&entry, bytes.clone()) {
                    entry.lock().retry_queue.push_back((bytes, 0));
                    emit_meta_event(
                        &entry,
                        json!({"type":"queued_for_retry","contract_version": CONTRACT_VERSION, "request_id": req_id}),
                    );
                }
            } else {
                emit_event(
                    &entry,
                    json!({"type":"error","contract_version": CONTRACT_VERSION, "payload": {"message": "bsatn encode failed for CallProcedure"}}),
                );
            }
        }
    }

    entry.lock().pending_requests.insert(req_id, name.clone());
    emit_meta_event(
        &entry,
        json!({"type":"procedure_sent","contract_version": CONTRACT_VERSION, "request_id": req_id, "procedure": name}),
    );
    0.0
}

// ---------------------------------------------------------------------------
// Native row cache API
// ---------------------------------------------------------------------------

pub fn stdb_table_count(handle: f64, table_name: &str) -> f64 {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return -1.0,
    };
    let n = entry.lock().cache.count(table_name) as f64;
    n
}

pub fn stdb_table_iter(handle: f64, table_name: &str) -> ArrayStream {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return ArrayStream::new(),
    };
    let rows = entry.lock().cache.iter_values(table_name);
    values_to_array_stream(rows)
}

pub fn stdb_table_find(handle: f64, table_name: String, pk: GMValueOwned) -> StructStream {
    let entry = match get_client(handle) {
        Some(e) => e,
        None => return StructStream::new(),
    };
    let pk_json = gmvalue_to_json(&pk);
    let Some(key) = pk_key_from_value(&pk_json) else {
        entry.lock().last_error = "stdb_table_find: invalid pk".to_string();
        return StructStream::new();
    };
    let out = match entry.lock().cache.find(&table_name, &key) {
        Some(row) => value_to_struct_stream(&row),
        None => StructStream::new(),
    };
    out
}
