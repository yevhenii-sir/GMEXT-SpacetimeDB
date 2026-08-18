//! JSON Bridge: Convert SpacetimeDB `ServerMessage` variants into JSON events
//! for the GameMaker event queue.
//!
//! Each `handle_*` function takes the client state entry and the decoded
//! `ServerMessage` variant, performs BSATN→JSON decoding where needed,
//! and pushes JSON event strings onto the event queue.

use crate::bsatn_json::{decode_bsatn_rows, try_decode_bsatn_error};
use crate::client_cache::pk_field_from_schema;
use crate::gml_client::{emit_event, CONTRACT_VERSION};
use parking_lot::Mutex;
use serde_json::{json, Value};
use spacetimedb_client_api_messages::websocket::v2 as ws_v2;
use spacetimedb_sats::ser::serde::SerializeWrapper;
use std::collections::HashMap;
use std::sync::Arc;

use crate::gml_client::ClientState;

// ---------------------------------------------------------------------------
// Top-level dispatcher
// ---------------------------------------------------------------------------

/// Process a decoded `ServerMessage` and emit appropriate JSON events.
pub fn handle_server_message(entry: &Arc<Mutex<ClientState>>, server_msg: ws_v2::ServerMessage) {
    match server_msg {
        ws_v2::ServerMessage::InitialConnection(ic) => handle_initial_connection(entry, ic),
        ws_v2::ServerMessage::SubscribeApplied(ap) => handle_subscribe_applied(entry, ap),
        ws_v2::ServerMessage::UnsubscribeApplied(ua) => handle_unsubscribe_applied(entry, ua),
        ws_v2::ServerMessage::SubscriptionError(se) => handle_subscription_error(entry, se),
        ws_v2::ServerMessage::TransactionUpdate(tu) => handle_transaction_update(entry, tu),
        ws_v2::ServerMessage::OneOffQueryResult(or) => handle_one_off_query_result(entry, or),
        ws_v2::ServerMessage::ReducerResult(rr) => handle_reducer_result(entry, rr),
        ws_v2::ServerMessage::ProcedureResult(pr) => handle_procedure_result(entry, pr),
    }
}

// ---------------------------------------------------------------------------
// Individual handlers
// ---------------------------------------------------------------------------

fn handle_initial_connection(entry: &Arc<Mutex<ClientState>>, ic: ws_v2::InitialConnection) {
    // Identity / ConnectionId are 256/128-bit ints — SerializeWrapper tries to
    // emit them as JSON numbers and fails with "number out of range". Build the
    // payload manually with hex strings (same encoding as Identity::to_hex).
    let identity_hex = ic.identity.to_hex().to_string();
    let connection_id_hex = ic.connection_id.to_hex().to_string();
    let token = ic.token.to_string();

    {
        let mut s = entry.lock();
        s.cached_token = Some(token.clone());
    }

    let payload = json!({
        "identity": identity_hex.clone(),
        "hex_identity": identity_hex,
        "connection_id": connection_id_hex,
        "token": token,
    });

    emit_event(
        entry,
        json!({"type":"initial_connection", "contract_version": CONTRACT_VERSION, "payload": payload.clone()}),
    );

    emit_event(
        entry,
        json!({
            "type": "identity_token",
            "contract_version": CONTRACT_VERSION,
            "payload": {
                "token": payload["token"],
                "identity": payload["identity"],
                "hex_identity": payload["hex_identity"],
                "connection_id": payload["connection_id"],
            }
        }),
    );
}

fn handle_subscribe_applied(entry: &Arc<Mutex<ClientState>>, ap: ws_v2::SubscribeApplied) {
    {
        let mut s = entry.lock();
        s.seen_subscribe_applied.insert(ap.query_set_id.id);
    }

    let json_rows = match serde_json::to_value(SerializeWrapper::from_ref(&ap.rows)) {
        Ok(v) => v,
        Err(e) => json!({"_error":"serialize_failed","msg": e.to_string()}),
    };

    let (table_schemas, struct_schemas) = {
        let s = entry.lock();
        (s.table_schemas.clone(), s.struct_schemas.clone())
    };

    let mut rows_decoded = Value::Null;
    if let Some(tables_val) = json_rows.get("tables") {
        if let Some(tables_arr) = tables_val.as_array() {
            let mut tables_out: Vec<Value> = Vec::new();
            for t in tables_arr.iter() {
                let mut table_obj = serde_json::Map::new();
                let mut table_name_str = String::new();
                if let Some(table_name) = t.get("table").and_then(|v| v.as_str()) {
                    table_name_str = table_name.to_string();
                    table_obj.insert("table".to_string(), Value::String(table_name_str.clone()));
                }

                let mut decoded_rows_array: Vec<Value> = Vec::new();

                if let Some(rows_container) = t.get("rows") {
                    if let Some(rows_data) = rows_container.get("rows_data") {
                        if let Some(hex_str) = rows_data.as_str() {
                            if let Some(schema) = table_schemas.get(&table_name_str) {
                                match decode_bsatn_rows(hex_str, schema, &struct_schemas) {
                                    Ok(parsed_rows) => {
                                        decoded_rows_array = parsed_rows;
                                    }
                                    Err(err_msg) => {
                                        let truncated = if hex_str.len() > 512 {
                                            &hex_str[..512]
                                        } else {
                                            hex_str
                                        };
                                        emit_event(
                                            entry,
                                            json!({"type":"subscribe_decode_error","query_set_id": ap.query_set_id.id, "table": table_name_str, "error": err_msg, "rows_data_hex_truncated": truncated}),
                                        );
                                        table_obj.insert(
                                            "rows_data".to_string(),
                                            Value::String(hex_str.to_string()),
                                        );
                                    }
                                }
                            } else {
                                emit_event(
                                    entry,
                                    json!({"type":"debug","payload":{"message": format!("No schema found for {}", table_name_str)}}),
                                );
                                table_obj.insert(
                                    "rows_data".to_string(),
                                    Value::String(hex_str.to_string()),
                                );
                            }
                        }
                    } else if let Some(rarr) = rows_container.get("rows") {
                        if let Some(arr) = rarr.as_array() {
                            decoded_rows_array = arr.clone();
                        }
                    }
                }

                table_obj.insert("rows_decoded".to_string(), Value::Array(decoded_rows_array));
                tables_out.push(Value::Object(table_obj));
            }
            rows_decoded = Value::Array(tables_out);
        }
    }

    // Apply decoded subscribe snapshot to native cache before emitting events.
    if let Value::Array(ref tables) = rows_decoded {
        cache_apply_subscribe_tables(entry, &table_schemas, tables);
    }

    let tables_count = match &rows_decoded {
        Value::Array(a) => a.len(),
        _ => 0,
    };
    emit_event(
        entry,
        json!({"type":"subscribe_applied_meta","query_set_id": ap.query_set_id.id, "tables_count": tables_count}),
    );

    emit_event(
        entry,
        json!({
            "type":"subscribe_applied",
            "contract_version": CONTRACT_VERSION,
            "request_id": ap.request_id,
            "query_set_id": ap.query_set_id.id,
            "payload": {
                "tables": rows_decoded
            }
        }),
    );
}

fn handle_unsubscribe_applied(entry: &Arc<Mutex<ClientState>>, ua: ws_v2::UnsubscribeApplied) {
    emit_event(
        entry,
        json!({"type":"unsubscribe_applied","contract_version": CONTRACT_VERSION, "request_id": ua.request_id, "query_set_id": ua.query_set_id.id}),
    );
}

fn handle_subscription_error(entry: &Arc<Mutex<ClientState>>, se: ws_v2::SubscriptionError) {
    emit_event(
        entry,
        json!({"type":"subscription_error","contract_version": CONTRACT_VERSION, "query_set_id": se.query_set_id.id, "payload": {"error": se.error}}),
    );
}

fn handle_transaction_update(entry: &Arc<Mutex<ClientState>>, tu: ws_v2::TransactionUpdate) {
    let mut payload = match serde_json::to_value(SerializeWrapper::from_ref(&tu)) {
        Ok(v) => v,
        Err(e) => json!({"_error":"serialize_failed","msg": e.to_string()}),
    };

    let (table_schemas, struct_schemas) = {
        let s = entry.lock();
        (s.table_schemas.clone(), s.struct_schemas.clone())
    };

    if let Some(obj) = payload.as_object_mut() {
        // Decode query_sets tables
        if let Some(query_sets) = obj.get_mut("query_sets").and_then(|v| v.as_array_mut()) {
            for qs in query_sets.iter_mut() {
                if let Some(tables) = qs.get_mut("tables").and_then(|v| v.as_array_mut()) {
                    for t in tables.iter_mut() {
                        decode_table_rows_in_place(t, &table_schemas, &struct_schemas, entry);
                    }
                }
            }
        }

        // Decode client_cache_update tables
        if let Some(client_cache) = obj
            .get_mut("client_cache_update")
            .and_then(|v| v.as_object_mut())
        {
            if let Some(tables) = client_cache
                .get_mut("tables")
                .and_then(|v| v.as_array_mut())
            {
                for t in tables.iter_mut() {
                    decode_client_cache_table_rows_in_place(
                        t,
                        &table_schemas,
                        &struct_schemas,
                        entry,
                    );
                }
            }
        }
    }

    // Mutate cache from decoded PersistentTable diffs before events.
    cache_apply_transaction_payload(entry, &table_schemas, &payload);

    emit_event(
        entry,
        json!({"type":"transaction_update","contract_version": CONTRACT_VERSION, "payload": payload}),
    );

    // Fallback subscribe_applied for unseen query_set_ids
    if let Some(obj2) = payload.as_object() {
        if let Some(query_sets) = obj2.get("query_sets").and_then(|v| v.as_array()) {
            for qs in query_sets.iter() {
                if let Some(qs_obj) = qs.as_object() {
                    if let Some(qid_val) = qs_obj
                        .get("query_set_id")
                        .and_then(|v| v.get("id"))
                        .and_then(|v| v.as_u64())
                    {
                        let qid_u32 = qid_val as u32;
                        let already = {
                            let s = entry.lock();
                            s.seen_subscribe_applied.contains(&qid_u32)
                        };
                        if !already {
                            let mut tables_out: Vec<Value> = Vec::new();
                            if let Some(tables) = qs_obj.get("tables").and_then(|v| v.as_array()) {
                                for t in tables.iter() {
                                    if let Some(tobj) = t.as_object() {
                                        let mut table_obj = serde_json::Map::new();
                                        let mut table_name_str = String::new();
                                        if let Some(tn) =
                                            tobj.get("table_name").and_then(|v| v.as_str())
                                        {
                                            table_name_str = tn.to_string();
                                        }
                                        table_obj.insert(
                                            "table".to_string(),
                                            Value::String(table_name_str.clone()),
                                        );

                                        let mut decoded_rows_array: Vec<Value> = Vec::new();
                                        if let Some(rows_arr) =
                                            tobj.get("rows").and_then(|v| v.as_array())
                                        {
                                            for row in rows_arr.iter() {
                                                // PersistentTable (regular tables)
                                                if let Some(persistent) = row
                                                    .get("PersistentTable")
                                                    .and_then(|v| v.as_object())
                                                {
                                                    if let Some(inserts_decoded) = persistent
                                                        .get("inserts_decoded")
                                                        .and_then(|v| v.as_array())
                                                    {
                                                        decoded_rows_array
                                                            .extend(inserts_decoded.clone());
                                                    }
                                                }
                                                // EventTable (procedural views)
                                                if let Some(event_table) = row
                                                    .get("EventTable")
                                                    .and_then(|v| v.as_object())
                                                {
                                                    if let Some(events_decoded) = event_table
                                                        .get("events_decoded")
                                                        .and_then(|v| v.as_array())
                                                    {
                                                        decoded_rows_array
                                                            .extend(events_decoded.clone());
                                                    }
                                                }
                                            }
                                        }

                                        table_obj.insert(
                                            "rows_decoded".to_string(),
                                            Value::Array(decoded_rows_array),
                                        );
                                        tables_out.push(Value::Object(table_obj));
                                    }
                                }
                            }

                            cache_apply_subscribe_tables(entry, &table_schemas, &tables_out);

                            emit_event(
                                entry,
                                json!({
                                    "type":"subscribe_applied_fallback",
                                    "contract_version": CONTRACT_VERSION,
                                    "query_set_id": qid_u32,
                                    "payload": { "tables": Value::Array(tables_out) }
                                }),
                            );

                            let mut s = entry.lock();
                            s.seen_subscribe_applied.insert(qid_u32);
                        }
                    }
                }
            }
        }
    }
}

fn handle_one_off_query_result(entry: &Arc<Mutex<ClientState>>, or: ws_v2::OneOffQueryResult) {
    emit_event(
        entry,
        json!({"type":"one_off_query_result","contract_version": CONTRACT_VERSION, "request_id": or.request_id}),
    );
}

fn handle_reducer_result(entry: &Arc<Mutex<ClientState>>, rr: ws_v2::ReducerResult) {
    let req_name = entry.lock().pending_requests.remove(&rr.request_id);
    let mut payload = match serde_json::to_value(SerializeWrapper::from_ref(&rr)) {
        Ok(v) => v,
        Err(e) => json!({"_error":"serialize_failed","msg": e.to_string()}),
    };

    let (table_schemas, struct_schemas, reducer_error_schemas) = {
        let s = entry.lock();
        (
            s.table_schemas.clone(),
            s.struct_schemas.clone(),
            s.reducer_error_schemas.clone(),
        )
    };

    // Decode Err payload (BSATN-encoded reducer error type)
    if let Some(err_hex) = payload
        .get("result")
        .and_then(|v| v.get("Err"))
        .and_then(|v| v.as_str())
    {
        let decoded =
            try_decode_bsatn_error(err_hex, &req_name, &struct_schemas, &reducer_error_schemas);
        if let Some(result_obj) = payload.get_mut("result").and_then(|v| v.as_object_mut()) {
            result_obj.insert("Err_decoded".to_string(), decoded);
        }
    }

    // Decode transaction_update rows in Ok variant
    if let Some(result_ok) = payload
        .get_mut("result")
        .and_then(|v| v.get_mut("Ok"))
        .and_then(|v| v.as_object_mut())
    {
        if let Some(tu_obj) = result_ok
            .get_mut("transaction_update")
            .and_then(|v| v.as_object_mut())
        {
            if let Some(query_sets) = tu_obj.get_mut("query_sets").and_then(|v| v.as_array_mut()) {
                for qs in query_sets.iter_mut() {
                    if let Some(tables) = qs.get_mut("tables").and_then(|v| v.as_array_mut()) {
                        for t in tables.iter_mut() {
                            decode_table_rows_in_place(t, &table_schemas, &struct_schemas, entry);
                        }
                    }
                }
            }
        }
    }

    // Cache apply for reducer Ok.transaction_update (GML remaps this to transaction_update).
    if let Some(tu) = payload
        .get("result")
        .and_then(|v| v.get("Ok"))
        .and_then(|v| v.get("transaction_update"))
    {
        cache_apply_transaction_payload(entry, &table_schemas, tu);
    }

    emit_event(
        entry,
        json!({
            "type": "reducer_result",
            "contract_version": CONTRACT_VERSION,
            "request_id": rr.request_id,
            "reducer": req_name,
            "payload": payload
        }),
    );
}

fn handle_procedure_result(entry: &Arc<Mutex<ClientState>>, pr: ws_v2::ProcedureResult) {
    let req_name = entry.lock().pending_requests.remove(&pr.request_id);
    let payload = match serde_json::to_value(SerializeWrapper::from_ref(&pr)) {
        Ok(v) => v,
        Err(e) => json!({"_error":"serialize_failed","msg": e.to_string()}),
    };
    emit_event(
        entry,
        json!({
            "type": "procedure_result",
            "contract_version": CONTRACT_VERSION,
            "request_id": pr.request_id,
            "procedure": req_name,
            "payload": payload
        }),
    );
}

// ---------------------------------------------------------------------------
// Client cache apply helpers
// ---------------------------------------------------------------------------

/// Upsert subscribe snapshot tables (`[{ table, rows_decoded: [...] }, ...]`).
fn cache_apply_subscribe_tables(
    entry: &Arc<Mutex<ClientState>>,
    table_schemas: &HashMap<String, Value>,
    tables: &[Value],
) {
    let mut s = entry.lock();
    for t in tables {
        let name = t
            .get("table")
            .or_else(|| t.get("table_name"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if name.is_empty() {
            continue;
        }
        let pk = table_schemas
            .get(name)
            .map(pk_field_from_schema)
            .unwrap_or_else(|| "id".to_string());
        if let Some(rows) = t.get("rows_decoded").and_then(|v| v.as_array()) {
            // Event-only snapshots may mix event rows; only upsert object rows with the PK.
            let row_objs: Vec<Value> = rows
                .iter()
                .filter(|r| r.is_object())
                .cloned()
                .collect();
            s.cache.apply_inserts(name, &pk, &row_objs);
        }
    }
}

/// Apply PersistentTable inserts/deletes from a transaction_update-shaped Value.
fn cache_apply_transaction_payload(
    entry: &Arc<Mutex<ClientState>>,
    table_schemas: &HashMap<String, Value>,
    payload: &Value,
) {
    let Some(query_sets) = payload.get("query_sets").and_then(|v| v.as_array()) else {
        return;
    };
    let mut s = entry.lock();
    for qs in query_sets {
        let Some(tables) = qs.get("tables").and_then(|v| v.as_array()) else {
            continue;
        };
        for t in tables {
            let name = t
                .get("table_name")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            if name.is_empty() {
                continue;
            }
            let pk = table_schemas
                .get(name)
                .map(pk_field_from_schema)
                .unwrap_or_else(|| "id".to_string());

            let mut inserts: Vec<Value> = Vec::new();
            let mut deletes: Vec<Value> = Vec::new();
            if let Some(rows) = t.get("rows").and_then(|v| v.as_array()) {
                for row in rows {
                    if let Some(persistent) = row.get("PersistentTable").and_then(|v| v.as_object())
                    {
                        if let Some(ins) = persistent
                            .get("inserts_decoded")
                            .and_then(|v| v.as_array())
                        {
                            inserts.extend(ins.iter().filter(|r| r.is_object()).cloned());
                        }
                        if let Some(dels) = persistent
                            .get("deletes_decoded")
                            .and_then(|v| v.as_array())
                        {
                            deletes.extend(dels.iter().filter(|r| r.is_object()).cloned());
                        }
                    }
                    // EventTable intentionally ignored for cache.
                }
            }
            if !inserts.is_empty() || !deletes.is_empty() {
                s.cache.apply_diff(name, &pk, &inserts, &deletes);
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Row decoding helpers (used by both TransactionUpdate and ReducerResult)
// ---------------------------------------------------------------------------

/// Decode BSATN rows in a query_set table entry (PersistentTable pattern).
/// Mutates the JSON value in-place, adding `inserts_decoded`/`deletes_decoded`
/// and removing raw `rows_data`/`size_hint` on success.
fn decode_table_rows_in_place(
    table_val: &mut Value,
    table_schemas: &std::collections::HashMap<String, Value>,
    struct_schemas: &std::collections::HashMap<String, Value>,
    entry: &Arc<Mutex<ClientState>>,
) {
    let table_obj = match table_val.as_object_mut() {
        Some(o) => o,
        None => return,
    };

    let mut t_name_str = String::new();
    if let Some(t_name) = table_obj.get("table_name").and_then(|v| v.as_str()) {
        t_name_str = t_name.to_string();
    } else if let Some(t_id) = table_obj.get("table_id").and_then(|v| v.as_u64()) {
        t_name_str = t_id.to_string();
    }

    let schema_opt = table_schemas.get(&t_name_str);

    if let Some(rows_arr) = table_obj.get_mut("rows").and_then(|v| v.as_array_mut()) {
        for row in rows_arr.iter_mut() {
            // Handle PersistentTable (regular tables with inserts/deletes)
            if let Some(persistent) = row
                .get_mut("PersistentTable")
                .and_then(|v| v.as_object_mut())
            {
                for key in ["inserts", "deletes"] {
                    let mut decode_ok = false;
                    let decoded_val = {
                        if let Some(records) = persistent.get(key) {
                            if let Some(rows_data) =
                                records.get("rows_data").and_then(|v| v.as_str())
                            {
                                if rows_data.is_empty() {
                                    decode_ok = true;
                                    Some(Value::Array(Vec::new()))
                                } else {
                                    let mut decoded = Vec::new();
                                    if let Some(schema) = schema_opt {
                                        match decode_bsatn_rows(
                                            rows_data,
                                            schema,
                                            struct_schemas,
                                        ) {
                                            Ok(r) => {
                                                decoded = r;
                                                decode_ok = true;
                                            }
                                            Err(err_msg) => {
                                                emit_event(
                                                    entry,
                                                    json!({"type":"debug","payload":{"message": format!("Transaction BSATN decode failed for {}: {}", t_name_str, err_msg)}}),
                                                );
                                            }
                                        }
                                    }
                                    Some(Value::Array(decoded))
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    };

                    if let Some(val) = decoded_val {
                        persistent.insert(format!("{}_decoded", key), val);
                        if decode_ok {
                            if let Some(records) = persistent.get_mut(key) {
                                if let Some(rec_obj) = records.as_object_mut() {
                                    rec_obj.remove("rows_data");
                                    rec_obj.remove("size_hint");
                                }
                            }
                        }
                    }
                }
            }

            // Handle EventTable (procedural views with event rows)
            // Event tables have a single "events" field instead of inserts/deletes.
            // We decode it the same way and expose it as "events_decoded" for GML.
            if let Some(event_table) = row.get_mut("EventTable").and_then(|v| v.as_object_mut()) {
                let mut decode_ok = false;
                let decoded_val = {
                    if let Some(events) = event_table.get("events") {
                        if let Some(rows_data) = events.get("rows_data").and_then(|v| v.as_str()) {
                            if rows_data.is_empty() {
                                decode_ok = true;
                                Some(Value::Array(Vec::new()))
                            } else {
                                let mut decoded = Vec::new();
                                if let Some(schema) = schema_opt {
                                    match decode_bsatn_rows(rows_data, schema, struct_schemas) {
                                        Ok(r) => {
                                            decoded = r;
                                            decode_ok = true;
                                        }
                                        Err(err_msg) => {
                                            emit_event(
                                                entry,
                                                json!({"type":"debug","payload":{"message": format!("Transaction BSATN decode failed for event table {}: {}", t_name_str, err_msg)}}),
                                            );
                                        }
                                    }
                                }
                                Some(Value::Array(decoded))
                            }
                        } else if let Some(rarr) = events.get("rows").and_then(|v| v.as_array()) {
                            decode_ok = true;
                            Some(Value::Array(rarr.clone()))
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(val) = decoded_val {
                    event_table.insert("events_decoded".to_string(), val);
                    if decode_ok {
                        if let Some(events) = event_table.get_mut("events") {
                            if let Some(ev_obj) = events.as_object_mut() {
                                ev_obj.remove("rows_data");
                                ev_obj.remove("size_hint");
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Decode BSATN rows in a client_cache_update table entry (flat pattern).
/// Also handles EventTable rows which use an "events" field instead of inserts/deletes.
fn decode_client_cache_table_rows_in_place(
    table_val: &mut Value,
    table_schemas: &std::collections::HashMap<String, Value>,
    struct_schemas: &std::collections::HashMap<String, Value>,
    entry: &Arc<Mutex<ClientState>>,
) {
    let table_obj = match table_val.as_object_mut() {
        Some(o) => o,
        None => return,
    };

    let mut t_name_str = String::new();
    if let Some(t_name) = table_obj.get("table_name").and_then(|v| v.as_str()) {
        t_name_str = t_name.to_string();
    } else if let Some(t_id) = table_obj.get("table_id").and_then(|v| v.as_u64()) {
        t_name_str = t_id.to_string();
    }

    let schema_opt = table_schemas.get(&t_name_str);

    // Handle PersistentTable fields: inserts and deletes
    for key in ["inserts", "deletes"] {
        let mut decode_ok = false;
        let decoded_val = {
            if let Some(records) = table_obj.get(key) {
                if let Some(rows_data) = records.get("rows_data").and_then(|v| v.as_str()) {
                    if rows_data.is_empty() {
                        decode_ok = true;
                        Some(Value::Array(Vec::new()))
                    } else {
                        let mut decoded = Vec::new();
                        if let Some(schema) = schema_opt {
                            match decode_bsatn_rows(rows_data, schema, struct_schemas) {
                                Ok(r) => {
                                    decoded = r;
                                    decode_ok = true;
                                }
                                Err(err_msg) => {
                                    emit_event(
                                        entry,
                                        json!({"type":"debug","payload":{"message": format!("Transaction BSATN decode failed: {}", err_msg)}}),
                                    );
                                }
                            }
                        }
                        Some(Value::Array(decoded))
                    }
                } else if let Some(rarr) = records.get("rows").and_then(|v| v.as_array()) {
                    decode_ok = true;
                    Some(Value::Array(rarr.clone()))
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(val) = decoded_val {
            table_obj.insert(format!("{}_decoded", key), val);
            if decode_ok {
                if let Some(records) = table_obj.get_mut(key) {
                    if let Some(rec_obj) = records.as_object_mut() {
                        rec_obj.remove("rows_data");
                        rec_obj.remove("size_hint");
                    }
                }
            }
        }
    }

    // Handle EventTable field: events
    if let Some(events) = table_obj.get("events") {
        let mut decode_ok = false;
        let decoded_val = {
            if let Some(rows_data) = events.get("rows_data").and_then(|v| v.as_str()) {
                if rows_data.is_empty() {
                    decode_ok = true;
                    Some(Value::Array(Vec::new()))
                } else {
                    let mut decoded = Vec::new();
                    if let Some(schema) = schema_opt {
                        match decode_bsatn_rows(rows_data, schema, struct_schemas) {
                            Ok(r) => {
                                decoded = r;
                                decode_ok = true;
                            }
                            Err(err_msg) => {
                                emit_event(
                                    entry,
                                    json!({"type":"debug","payload":{"message": format!("Event table BSATN decode failed for {}: {}", t_name_str, err_msg)}}),
                                );
                            }
                        }
                    }
                    Some(Value::Array(decoded))
                }
            } else if let Some(rarr) = events.get("rows").and_then(|v| v.as_array()) {
                decode_ok = true;
                Some(Value::Array(rarr.clone()))
            } else {
                None
            }
        };

        if let Some(val) = decoded_val {
            table_obj.insert("events_decoded".to_string(), val);
            if decode_ok {
                if let Some(events) = table_obj.get_mut("events") {
                    if let Some(ev_obj) = events.as_object_mut() {
                        ev_obj.remove("rows_data");
                        ev_obj.remove("size_hint");
                    }
                }
            }
        }
    }
}
