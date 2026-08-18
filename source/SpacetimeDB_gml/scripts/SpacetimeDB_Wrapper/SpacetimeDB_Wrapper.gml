#macro DEBUG_SPACETIMEDB true

/* =========================================================================
   SPACETIMEDB GAMEMAKER SDK CHEAT SHEET
   =========================================================================

   0. NATIVE ROW CACHE (preferred store of truth)
   -----------------------------------------------------
   After subscribe/transaction diffs, Rust keeps decoded rows keyed by primary key.
   Poll events are unchanged; use these for Draw/game state instead of mirroring
   inserts/deletes into a GML array yourself.

   spdb_register_schema(conn, "player", [
       { name: "id", type: "u64" },
       { name: "name", type: "string" }
   ], "id");  // primary_key optional, default "id"

   var n     = spdb_table_count(conn, "player");       // real
   var rows  = spdb_table_iter(conn, "player");        // array of structs
   var row   = spdb_table_find(conn, "player", pk);    // struct, or {} if missing

   Notes (wave-1):
   - Update = delete(old) + insert(new) with same PK → cache upserts; no false remove.
   - EventTable rows are NOT cached.
   - Unsubscribe does NOT purge rows; cache clears on disconnect/destroy only.

   1. SUBSCRIPTION (Initial Snapshot)
   -----------------------------------------------------
   When you call `spdb_subscribe` or `spdb_subscribe_all`,
   the server sends ALL CURRENT DATA matching the query.

   Arguments in on_applied_cb(tables_dict, raw_array):

   - tables_dict: {
         "Player": [ {id: 1, name: "Bob"}, {id: 2, name: "Alice"} ],
         "Item":   [ {id: 10, type: "Sword"} ]
     }
     *Access: var p = tables_dict[$ "Player"];*

   - raw_array: [
         { table: "Player", rows_decoded: [...] },
         { table: "Item",   rows_decoded: [...] }
     ]

   2. UPDATES (Live Updates / Transaction Update)
   -----------------------------------------------------
   Use `spdb_on_table_update(conn, "TableName", on_update_cb)`.
   The wrapper parses the raw JSON from Rust and gives you just the essentials!

   Arguments in on_update_cb(inserts, deletes):

   - inserts: Array of inserted OR modified rows (in SpacetimeDB update = delete + insert).
              Example: [ {id: 1, hp: 90} ]
   - deletes: Array of deleted rows (previous state).
              Example: [ {id: 1, hp: 100} ]

   Prefer reading state via spdb_table_iter/find; use these callbacks for reactions/logs.

   3. AUTOMATION (spdb_bind_table)
   -----------------------------------------------------
   Snapshot + live updates for one table (events only; rows also land in native cache):

   spdb_bind_table(conn, "SELECT * FROM Enemy", "Enemy",
       function(rows) { // initial snapshot array
           show_debug_message("Enemies: " + string(array_length(rows)));
       },
       function(inserts, deletes) { // live diff arrays
           // Prefer spdb_table_iter/find for current rows
       }
   );

   4. CALLING A REDUCER (spdb_call_reducer)
   -----------------------------------------------------
   var req_id = spdb_call_reducer(conn, "move_player", { dir_x: 1, dir_y: 0 }, {
       on_result: function(ev) {
           // Called ALWAYS when the server responds (both Ok and Err)
           show_debug_message("Reducer finished. Request ID: " + string(ev.request_id));
       },
       on_complete: function(ev) {
           // Called ONLY on success (Ok)
           show_debug_message("Move succeeded!");
       },
       on_error: function(ev) {
           // Called ONLY on failure (Err or InternalError)
           // Access decoded error: ev.payload.result.Err_decoded (preferred) or ev.payload.result.Err (raw hex)
           // Access internal error: ev.payload.result.InternalError (string)
           show_debug_message("Move failed: " + string(ev.payload.result.Err_decoded));
       }
   });
========================================================================= */

/// Queue BSATN args buffer (length = written bytes from start) then call reducer FFI.
function __spdb_native_call_reducer(conn_id, name, rid, buf) {
	static __empty_args = buffer_create(1, buffer_fixed, 1);
	var args_len = is_undefined(buf) ? 0 : buffer_tell(buf);
	if (args_len <= 0) {
		__SpacetimeDB_queue_buffer(buffer_get_address(__empty_args), 0);
	} else {
		__SpacetimeDB_queue_buffer(buffer_get_address(buf), args_len);
	}
	var __args_buffer = __ext_core_get_args_buffer();
	buffer_write(__args_buffer, buffer_f64, conn_id);
	buffer_write(__args_buffer, buffer_u32, string_byte_length(name));
	buffer_write(__args_buffer, buffer_string, name);
	buffer_write(__args_buffer, buffer_f64, rid);
	return __stdb_call_reducer(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
}

/// Queue BSATN args buffer then call procedure FFI.
function __spdb_native_call_procedure(conn_id, name, rid, buf) {
	static __empty_args = buffer_create(1, buffer_fixed, 1);
	var args_len = is_undefined(buf) ? 0 : buffer_tell(buf);
	if (args_len <= 0) {
		__SpacetimeDB_queue_buffer(buffer_get_address(__empty_args), 0);
	} else {
		__SpacetimeDB_queue_buffer(buffer_get_address(buf), args_len);
	}
	var __args_buffer = __ext_core_get_args_buffer();
	buffer_write(__args_buffer, buffer_f64, conn_id);
	buffer_write(__args_buffer, buffer_u32, string_byte_length(name));
	buffer_write(__args_buffer, buffer_string, name);
	buffer_write(__args_buffer, buffer_f64, rid);
	return __stdb_call_procedure(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
}

/// @description Lightweight improved wrapper for SpacetimeDB GameMaker SDK (v2)
/// Provides a small, robust API that sits on top of the low-level stdb_* FFI.

/// Create/connect helper: returns a connection struct
function spdb_connect(uri, db_name, token) {
    var conn = {
        id: stdb_create_client(),
        uri: uri,
        db: db_name,
        token: token,
        listeners: ds_map_create(), // event_name -> ds_list of callbacks
        subs: ds_list_create(), // list of subscription maps { query_set_id, sql, persistent? }
        pending: ds_map_create(), // request_id -> callback map
        queue: ds_list_create(), // queued calls while disconnected
		table_listeners: ds_map_create(),
		persistent_tables: ds_list_create(), // table names that survive spdb_unsubscribe_all_events
        next_request_id: 1,
        connected: false,
        last_error: ""
    };

    // ensure listener containers exist for common events
    var evs =["connected","disconnected","error","identity_token","debug","transaction_update","table_update","transaction_query_set","reducer_result","procedure_result","subscribe_applied","unsubscribe_applied","row_change","reconnecting","reconnected","reconnect_failed"];
    for (var i=0; i<array_length(evs); i++) {
        ds_map_add(conn.listeners, evs[i], ds_list_create());
    }

	spdb_set_auto_reconnect(conn, true, 1000, 2000, 15000);

    var rc = stdb_connect_simple(conn.id, uri, db_name, token);
    if (rc != 0) {
        conn.last_error = stdb_get_last_error(conn.id);
    }
    return conn;
}

/// Disconnect + destroy connection
function spdb_disconnect(conn) {
    if (is_undefined(conn) || !variable_struct_exists(conn, "id")) return;
    stdb_disconnect(conn.id);
    stdb_destroy_client(conn.id);

    // cleanup listeners
    if (variable_struct_exists(conn, "listeners") && ds_exists(conn.listeners, ds_type_map)) {
        var keys = ds_map_keys_to_array(conn.listeners);
        for (var i=0; i<array_length(keys); i++) {
            var l = conn.listeners[? keys[i]];
            if (ds_exists(l, ds_type_list)) {
                ds_list_destroy(l);
            }
        }
        ds_map_destroy(conn.listeners);
    }

    // cleanup subs
    if (variable_struct_exists(conn, "subs") && ds_exists(conn.subs, ds_type_list)) {
        for (var i=0; i<ds_list_size(conn.subs); i++) {
            var s = conn.subs[| i];
            if (ds_exists(s, ds_type_map)) {
                ds_map_destroy(s);
            }
        }
        ds_list_destroy(conn.subs);
    }

    // cleanup pending/queue
    if (variable_struct_exists(conn, "pending") && ds_exists(conn.pending, ds_type_map)) {
        ds_map_destroy(conn.pending);
    }
    if (variable_struct_exists(conn, "queue") && ds_exists(conn.queue, ds_type_list)) {
        for (var i=0; i<ds_list_size(conn.queue); i++) {
            var q = conn.queue[| i];
            if (ds_exists(q, ds_type_map)) {
                ds_map_destroy(q);
            }
        }
        ds_list_destroy(conn.queue);
    }

    if (variable_struct_exists(conn, "table_listeners") && ds_exists(conn.table_listeners, ds_type_map)) {
        var t_keys = ds_map_keys_to_array(conn.table_listeners);
        for (var i=0; i<array_length(t_keys); i++) {
            var l = conn.table_listeners[? t_keys[i]];
            if (ds_exists(l, ds_type_list)) {
                ds_list_destroy(l);
            }
        }
        ds_map_destroy(conn.table_listeners);
    }
    if (variable_struct_exists(conn, "persistent_tables") && ds_exists(conn.persistent_tables, ds_type_list)) {
        ds_list_destroy(conn.persistent_tables);
    }
}

/// Register an event listener. callback is a script or function
function spdb_on(conn, event_name, callback) {
	if (is_undefined(conn)) return false;
    if (!ds_map_exists(conn.listeners, event_name)) return false;
    var list = conn.listeners[? event_name];
    ds_list_add(list, callback);
    return true;
}

/// Internal: dispatch event to listeners
function spdb_dispatch(conn, ev) {
	if (is_undefined(conn)) return;
    if (is_undefined(ev) || !variable_struct_exists(ev, "type")) return;
    var t = ev.type;
    if (!ds_map_exists(conn.listeners, t)) return;
    var list = conn.listeners[? t];
    for (var i=0; i<ds_list_size(list); i++) {
        var cb = list[| i];
        if (!is_undefined(cb) && cb != -1) cb(ev);
    }
}

// Safe struct helper: checks if struct has a named field
function spdb_struct_has(_s, _name) {
    return is_struct(_s) && variable_struct_exists(_s, _name);
}

// Normalize incoming rows payload into a flat array of row structs.
// Accepts: array of {table, rows_decoded}, single struct, array of structs, or raw inserts array.
function spdb_normalize_rows(_data, _expected_table) {
    var out = [];
    if (is_undefined(_data)) return out;

    if (is_array(_data)) {
        for (var i = 0; i < array_length(_data); i++) {
            var item = _data[i];
            if (spdb_struct_has(item, "rows_decoded")) {
                if (is_undefined(_expected_table) || _expected_table == "" || (spdb_struct_has(item, "table") && item.table == _expected_table)) {
                    var rr = item.rows_decoded;
                    if (is_array(rr)) {
                        for (var j = 0; j < array_length(rr); j++) array_push(out, rr[j]);
                    }
                }
            }
            else if (is_struct(item)) {
                array_push(out, item);
            }
        }
        return out;
    }

    if (is_struct(_data)) {
        if (spdb_struct_has(_data, "rows_decoded")) {
            if (is_undefined(_expected_table) || _expected_table == "" || (spdb_struct_has(_data, "table") && _data.table == _expected_table)) {
                var r = _data.rows_decoded;
                if (is_array(r)) {
                    for (var k = 0; k < array_length(r); k++) array_push(out, r[k]);
                }
            }
        } else {
            array_push(out, _data);
        }
    }
    return out;
}

/// Register a callback for a specific table's insert/delete events
/// callback takes two arguments: function(inserts_array, deletes_array)
/// NOTE: Replaces any existing callback for the same table (prevents duplicate
/// callbacks on reconnect). If you need multiple listeners, use spdb_on_table_update_append.
function spdb_on_table_update(conn, table_name, callback) {
	if (is_undefined(conn)) return;
    if (!ds_map_exists(conn.table_listeners, table_name)) {
        ds_map_add(conn.table_listeners, table_name, ds_list_create());
    }
    var list = conn.table_listeners[? table_name];
    ds_list_clear(list); // Replace instead of append — prevents duplicate callbacks on reconnect
    ds_list_add(list, callback);
}

/// Append a callback for a specific table's insert/delete events (allows multiple listeners).
/// Prefer spdb_on_table_update for single-listener use cases to avoid duplicates on reconnect.
function spdb_on_table_update_append(conn, table_name, callback) {
	if (is_undefined(conn)) return;
    if (!ds_map_exists(conn.table_listeners, table_name)) {
        ds_map_add(conn.table_listeners, table_name, ds_list_create());
    }
    var list = conn.table_listeners[? table_name];
    ds_list_add(list, callback);
}

/// Subscribe helper (returns qid or -1)
/// Deduplicates: if the same SQL is already subscribed, returns existing qid
/// and updates the on_applied callback.
function spdb_subscribe(conn, sql, on_applied_cb = undefined) {
	if (is_undefined(conn)) return -1;
    if (!conn.connected) {
        var ev = { type: "error", message: "subscribe called before connected", sql: sql };
        spdb_dispatch(conn, ev);
        return -1;
    }

    // Dedup: if same SQL already subscribed, return existing qid and update callback
    for (var i = 0; i < ds_list_size(conn.subs); i++) {
        var existing = conn.subs[| i];
        if (existing[? "sql"] == sql) {
            if (!is_undefined(on_applied_cb)) {
                ds_map_replace(existing, "on_applied", on_applied_cb);
            }
            return existing[? "query_set_id"];
        }
    }

    var qid = stdb_subscribe_sql(conn.id, sql);
    if (qid < 0) return -1;

    var sub = ds_map_create();
    ds_map_add(sub, "query_set_id", qid);
    ds_map_add(sub, "sql", sql);
    ds_map_add(sub, "on_applied", on_applied_cb);
    ds_list_add(conn.subs, sub);

    return qid;
}

/// Persistent subscribe: survives spdb_unsubscribe_all_events.
/// Use for tables that must always be subscribed (e.g., app_config, user_account).
/// Deduplicates: if the same SQL is already subscribed, returns existing qid,
/// updates callback, and marks as persistent.
/// Returns qid or -1.
function spdb_subscribe_persistent(conn, sql, on_applied_cb = undefined) {
	if (is_undefined(conn)) return -1;
    if (!conn.connected) {
        var ev = { type: "error", message: "subscribe_persistent called before connected", sql: sql };
        spdb_dispatch(conn, ev);
        return -1;
    }

    // Dedup: if same SQL already subscribed, update callback and mark persistent
    for (var i = 0; i < ds_list_size(conn.subs); i++) {
        var existing = conn.subs[| i];
        if (existing[? "sql"] == sql) {
            if (!is_undefined(on_applied_cb)) {
                ds_map_replace(existing, "on_applied", on_applied_cb);
            }
            if !ds_map_exists(existing, "persistent") {
                ds_map_add(existing, "persistent", true);
            }
            return existing[? "query_set_id"];
        }
    }

    var qid = stdb_subscribe_sql(conn.id, sql);
    if (qid < 0) return -1;

    var sub = ds_map_create();
    ds_map_add(sub, "query_set_id", qid);
    ds_map_add(sub, "sql", sql);
    ds_map_add(sub, "on_applied", on_applied_cb);
    ds_map_add(sub, "persistent", true);
    ds_list_add(conn.subs, sub);

    return qid;
}

/// Mark a table's listeners as persistent — they survive spdb_unsubscribe_all_events.
/// Call this after spdb_on_table_update for tables that must always be listened to.
function spdb_mark_table_persistent(conn, table_name) {
	if (is_undefined(conn)) return;
    if (ds_list_find_index(conn.persistent_tables, table_name) == -1) {
        ds_list_add(conn.persistent_tables, table_name);
    }
}

/// Subscribe to multiple queries at once (array of SQL strings)
/// on_applied_cb is called ONCE with an array of ALL decoded tables
/// Deduplicates: if the exact same SQL array is already subscribed, returns existing qid
/// and updates the on_applied callback. Supports multiple different batch subscriptions.
function spdb_subscribe_all(conn, sql_array, on_applied_cb = undefined) {
	if (is_undefined(conn)) return -1;
    if (!conn.connected) {
        var ev = { type: "error", message: "subscribe_all called before connected", sql: sql_array };
        spdb_dispatch(conn, ev);
        return -1;
    }

    var json_key = json_stringify(sql_array);

    // Dedup: if the exact same SQL array is already subscribed, update callback and return existing qid
    for (var i = 0; i < ds_list_size(conn.subs); i++) {
        var existing = conn.subs[| i];
        if (existing[? "sql"] == "BATCH_SUBSCRIBE" && ds_map_exists(existing, "sql_array_key")) {
            if (existing[? "sql_array_key"] == json_key) {
                if (!is_undefined(on_applied_cb)) {
                    ds_map_replace(existing, "on_applied", on_applied_cb);
                }
                return existing[? "query_set_id"];
            }
        }
    }

    var __args_buffer = __ext_core_get_args_buffer();
    buffer_write(__args_buffer, buffer_f64, conn.id);
    __ext_core_buffer_marshal_value(__args_buffer, sql_array);
    var qid = __stdb_subscribe_all(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
    if (qid < 0) return -1;

    var sub = ds_map_create();
    ds_map_add(sub, "query_set_id", qid);
    ds_map_add(sub, "sql", "BATCH_SUBSCRIBE");
    ds_map_add(sub, "sql_array_key", json_key);
    ds_map_add(sub, "sql_array", sql_array);
    ds_map_add(sub, "on_applied", on_applied_cb);
    ds_list_add(conn.subs, sub);

    return qid;
}

function spdb_unsubscribe(conn, qid) {
	if (is_undefined(conn)) return false;
    var rc = stdb_unsubscribe_sql(conn.id, qid);
    if (rc != 0) return false;

    // remove locally
    for (var i=0; i<ds_list_size(conn.subs); i++) {
        var s = conn.subs[| i];
        if (s[? "query_set_id"] == qid) {
            ds_map_destroy(s);
            ds_list_delete(conn.subs, i);
            break;
        }
    }
    return true;
}

/// Call reducer with BSATN-encoded args with optional callback struct { on_result, on_complete, on_error }
function spdb_call_reducer(conn, name, args_struct, callback) {
	if (is_undefined(conn)) return -1;
    var rid = conn.next_request_id++;

    if (!conn.connected) {
        var tmp = ds_map_create();
        ds_map_add(tmp, "kind", "reducer");
        ds_map_add(tmp, "name", name);
        ds_map_add(tmp, "args_struct", args_struct);
        ds_map_add(tmp, "request_id", rid);
        ds_map_add(tmp, "callback", callback);
        ds_list_add(conn.queue, tmp);
        return rid;
    }

    var schema = global.spdb_reducer_schemas[? name];
    if (is_undefined(schema)) {
        show_debug_message("ERROR: Schema for reducer " + name + " is not registered!");
        return -1;
    }

    var buf = undefined;
    if (array_length(schema) > 0) {
        buf = buffer_create(512, buffer_grow, 1);
        spdb_encode_bsatn_to_buffer(schema, args_struct, buf);
    }
    var rc = __spdb_native_call_reducer(conn.id, name, rid, buf);
    if (!is_undefined(buf)) buffer_delete(buf);

    if (rc != 0) return undefined;
    if (!is_undefined(callback)) ds_map_add(conn.pending, string(rid), callback);
    return rid;
}

/// Call procedure with BSATN-encoded args with optional callback struct { on_result, on_complete, on_error }
function spdb_call_procedure(conn, name, args_struct, callback) {
	if (is_undefined(conn)) return -1;
    var rid = conn.next_request_id++;

    if (!conn.connected) {
        var tmp = ds_map_create();
        ds_map_add(tmp, "kind", "procedure");
        ds_map_add(tmp, "name", name);
        ds_map_add(tmp, "args_struct", args_struct);
        ds_map_add(tmp, "request_id", rid);
        ds_map_add(tmp, "callback", callback);
        ds_list_add(conn.queue, tmp);
        return rid;
    }

    var schema = global.spdb_procedure_schemas[? name];
    if (is_undefined(schema)) {
        show_debug_message("ERROR: Schema for procedure " + name + " is not registered!");
        return -1;
    }

    var buf = undefined;
    if (array_length(schema) > 0) {
        buf = buffer_create(512, buffer_grow, 1);
        spdb_encode_bsatn_to_buffer(schema, args_struct, buf);
    }
    var rc = __spdb_native_call_procedure(conn.id, name, rid, buf);
    if (!is_undefined(buf)) buffer_delete(buf);

    if (rc != 0) return undefined;
    if (!is_undefined(callback)) ds_map_add(conn.pending, string(rid), callback);
    return rid;
}

/// Register schema helper
/// Register schema helper. Optional `primary_key` defaults to `"id"`.
function spdb_register_schema(conn, table_name, schema_fields, primary_key = "id") {
	if (is_undefined(conn)) return -1;
    if (is_undefined(primary_key) || primary_key == "") primary_key = "id";
    var schema = { primary_key: primary_key, fields: schema_fields };
    var __args_buffer = __ext_core_get_args_buffer();
    buffer_write(__args_buffer, buffer_f64, conn.id);
    buffer_write(__args_buffer, buffer_u32, string_byte_length(table_name));
    buffer_write(__args_buffer, buffer_string, table_name);
    __ext_core_buffer_marshal_value(__args_buffer, schema);
    return __stdb_register_schema(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
}

function spdb_clear_table_listeners(conn, table_name) {
	if (is_undefined(conn)) return;
    if (ds_map_exists(conn.table_listeners, table_name)) {
        ds_list_clear(conn.table_listeners[? table_name]);
    }
}

/// Poll events and dispatch
function spdb_poll(conn) {
	if (is_undefined(conn)) return;
    // One FFI call drains the whole queue as a typed GMValue array (no json_parse).
    var __ret_buffer = __ext_core_get_ret_buffer(1024 * 1024);
    var rc = __stdb_poll_events_batch(conn.id, buffer_get_address(__ret_buffer), buffer_get_size(__ret_buffer));
    var events = [];
    if (rc == 0) {
        buffer_seek(__ret_buffer, buffer_seek_start, 0);
        var parsed = __ext_core_buffer_unmarshal_value(__ret_buffer, []);
        if (is_array(parsed)) events = parsed;
    }

    for (var ei = 0; ei < array_length(events); ei++) {
        var ev = events[ei];
        if (!is_struct(ev)) continue;

		if (DEBUG_SPACETIMEDB) {
			show_debug_message($"DEBUG ev -> {json_stringify(ev)}");
		}

        // Update connection booleans for common events
        switch (ev.type) {
            case "connected": conn.connected = true; break;
            case "disconnected": conn.connected = false; break;
            case "reconnecting": conn.connected = false; break;
            case "reconnected": conn.connected = true; break;
            case "reconnect_failed": conn.connected = false; break;
            case "token_swap_requested": break;
        }

        // --- UNIFIED HANDLER FOR REDUCER & PROCEDURE RESULTS ---
        if (ev.type == "reducer_result" || ev.type == "procedure_result") {
            var payload = ev[$ "payload"];
            var result_obj = payload[$ "result"] ?? {};
            var is_ok = variable_struct_exists(result_obj, "Ok");
            var is_ok_empty = variable_struct_exists(result_obj, "OkEmpty");
            var is_err = variable_struct_exists(result_obj, "Err");
            var is_internal_error = variable_struct_exists(result_obj, "InternalError");

            // 1. Resolve Request ID
            var rid = ev[$ "request_id"];
            if (is_undefined(rid) && is_struct(payload)) {
                rid = payload[$ "request_id"];
            }

            var cb = undefined;
            var rid_str = "";

            // 2. Find and execute pending callbacks
            if (!is_undefined(rid) && rid != -1) {
                rid_str = string(int64(rid));
                cb = conn.pending[? rid_str];

                if (!is_undefined(cb)) {
                    if (is_struct(cb)) {
                        // A. Universal callback (fires always)
                        if (!is_undefined(cb[$ "on_result"])) cb.on_result(ev);

                        // B. Specific callbacks (fire conditionally)
                        if ((is_ok || is_ok_empty) && !is_undefined(cb[$ "on_complete"])) {
                            cb.on_complete(ev);
                        } else if ((is_err || is_internal_error) && !is_undefined(cb[$ "on_error"])) {
                            cb.on_error(ev);
                        }
                    }
                    ds_map_delete(conn.pending, rid_str);
                }
            } else {
                show_debug_message("WARNING: Result received, but request_id not found! type=" + string(ev.type));
            }

            // 3. Handle Event Transformation & Default Error Logging
            if (ev.type == "reducer_result") {
                if (is_ok) {
                    // Transform to transaction_update for table bindings
                    var ok_data = result_obj[$ "Ok"] ?? {};
                    if (variable_struct_exists(ok_data, "transaction_update")) {
                        ev.type = "transaction_update";
                        ev.payload = ok_data.transaction_update;
                    }
                } else if (is_ok_empty) {
                    // Success with no data — nothing to transform
                } else if (is_err) {
                    // Use Err_decoded if available (decoded BSATN error), otherwise raw hex
                    var err_decoded = variable_struct_exists(result_obj, "Err_decoded") ? result_obj[$ "Err_decoded"] : result_obj[$ "Err"];
                    if (is_undefined(cb) || !is_struct(cb) || is_undefined(cb[$ "on_error"])) {
                        show_debug_message("Reducer Error (unhandled): " + string(err_decoded));
                    }
                } else if (is_internal_error) {
                    // InternalError is a plain string (diagnostic, not structured)
                    if (is_undefined(cb) || !is_struct(cb) || is_undefined(cb[$ "on_error"])) {
                        show_debug_message("Reducer InternalError (unhandled): " + string(result_obj.InternalError));
                    }
                }
            }
        }

		if (ev.type == "subscribe_applied" || ev.type == "subscribe_applied_fallback") {
            var qid = ev.query_set_id;
            for(var i = 0; i < ds_list_size(conn.subs); i++) {
                var sub = conn.subs[| i];
                if (sub[? "query_set_id"] == qid) {
                    var cb = sub[? "on_applied"];
                    if (!is_undefined(cb) && cb != -1) {
						var payload = ev[$ "payload"];
                        if (!is_undefined(payload)) {
                            var _tables = payload[$ "tables"];
							if (is_array(_tables)) {
							    var tables_dict = {};
							    for (var k = 0; k < array_length(_tables); k++) {
							        var t_obj = _tables[k];
							        var t_name = t_obj[$ "table"];
							        if (!is_undefined(t_name)) {
							            tables_dict[$ t_name] = t_obj[$ "rows_decoded"];
							        }
							    }
							    cb(tables_dict, _tables);
							}
                        }
                    }
                    break;
                }
            }
        }

		if (ev.type == "transaction_update") {
            var payload = ev[$ "payload"];
            if (!is_undefined(payload) && is_array(payload[$ "query_sets"])) {
                var query_sets = payload.query_sets;

                for (var qs_idx = 0; qs_idx < array_length(query_sets); qs_idx++) {
                    var qs = query_sets[qs_idx];

                    if (is_array(qs[$ "tables"])) {
                        var tables = qs.tables;

                        for (var t_idx = 0; t_idx < array_length(tables); t_idx++) {
                            var t = tables[t_idx];
                            var t_name = t[$ "table_name"];
                            if (is_undefined(t_name)) continue;

                            var all_inserts = [];
                            var all_deletes = [];
                            var all_events = [];

                            if (is_array(t[$ "rows"])) {
                                var rows_arr = t.rows;
                                for (var r_idx = 0; r_idx < array_length(rows_arr); r_idx++) {
                                    var row_chunk = rows_arr[r_idx];

                                    // PersistentTable (regular tables)
                                    var persistentTable = row_chunk[$ "PersistentTable"];
                                    if (is_struct(persistentTable)) {
                                        var ins = persistentTable[$ "inserts_decoded"];
                                        if (is_array(ins)) {
                                            for(var _i = 0; _i < array_length(ins); _i++) array_push(all_inserts, ins[_i]);
                                        }

                                        var dels = persistentTable[$ "deletes_decoded"];
                                        if (is_array(dels)) {
                                            for(var _d = 0; _d < array_length(dels); _d++) array_push(all_deletes, dels[_d]);
                                        }
                                    }

                                    // EventTable (procedural views)
                                    var eventTable = row_chunk[$ "EventTable"];
                                    if (is_struct(eventTable)) {
                                        var evts = eventTable[$ "events_decoded"];
                                        if (is_array(evts)) {
                                            for(var _e = 0; _e < array_length(evts); _e++) array_push(all_events, evts[_e]);
                                        }
                                    }
                                }
                            }

                            if (array_length(all_inserts) > 0 || array_length(all_deletes) > 0) {
                                if (ds_map_exists(conn.table_listeners, t_name)) {
                                    var list = conn.table_listeners[? t_name];
                                    for (var j = 0; j < ds_list_size(list); j++) {
                                        var cb = list[| j];
                                        if (!is_undefined(cb) && cb != -1) {
                                            cb(all_inserts, all_deletes);
                                        }
                                    }
                                }
                            }

                            // Event tables use a different callback pattern: (events_array)
                            if (array_length(all_events) > 0) {
                                if (ds_map_exists(conn.table_listeners, t_name)) {
                                    var list = conn.table_listeners[? t_name];
                                    for (var j = 0; j < ds_list_size(list); j++) {
                                        var cb = list[| j];
                                        if (!is_undefined(cb) && cb != -1) {
                                            cb(all_events, []);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Dispatch to registered listeners (for generic event hooks via spdb_on)
        spdb_dispatch(conn, ev);
    }

    // flush queue if connected
    if (conn.connected) {
        var i = 0;
        while (i < ds_list_size(conn.queue)) {
            var q = conn.queue[| i];
            var kind = q[? "kind"];
            var name = q[? "name"];
            var rid = q[? "request_id"];
            var cb = q[? "callback"];
            var rc = -1;

            if (kind == "reducer") {
             var schema = global.spdb_reducer_schemas[? name];
             if (!is_undefined(schema)) {
     var buf = undefined;
     if (array_length(schema) > 0) {
      buf = buffer_create(512, buffer_grow, 1);
      var args_struct = q[? "args_struct"];
      spdb_encode_bsatn_to_buffer(schema, args_struct, buf);
     }
     rc = __spdb_native_call_reducer(conn.id, name, rid, buf);
     if (!is_undefined(buf)) buffer_delete(buf);
             }
         } else if (kind == "procedure") {
             var schema = global.spdb_procedure_schemas[? name];
             if (!is_undefined(schema)) {
     var buf = undefined;
     if (array_length(schema) > 0) {
      buf = buffer_create(512, buffer_grow, 1);
      var args_struct = q[? "args_struct"];
      spdb_encode_bsatn_to_buffer(schema, args_struct, buf);
     }
     rc = __spdb_native_call_procedure(conn.id, name, rid, buf);
     if (!is_undefined(buf)) buffer_delete(buf);
             }
            }

            if (rc == 0) {
                if (!is_undefined(cb)) ds_map_add(conn.pending, string(rid), cb);
                ds_map_destroy(q);
                ds_list_delete(conn.queue, i);
            } else {
                conn.last_error = stdb_get_last_error(conn.id);
                break;
            }
        }
    }
}


/// @param {Struct} conn
/// @param {String} sql - SQL query
/// @param {String} table_name - Table name (Case Sensitive)
/// @param {Function} on_snapshot - Fires once on load. Argument: (rows_array)
/// @param {Function} on_transaction - Fires on changes. Arguments: (inserts_array, deletes_array)
function spdb_bind_table(conn, sql, table_name, on_snapshot, on_transaction) {
	if (is_undefined(conn)) return -1;
    var ctx = {
        t_name: table_name,
        cb_snapshot: on_snapshot,
        cb_trans: on_transaction
    };

    var start_cb = method(ctx, function(tables_dict) {
        if (!is_struct(tables_dict)) return;

        var rows = tables_dict[$ self.t_name];
        if (is_array(rows) && !is_undefined(self.cb_snapshot)) {
            self.cb_snapshot(rows);
        }
    });

    var sub_id = spdb_subscribe(conn, sql, start_cb);

    var update_cb = method(ctx, function(inserts, deletes) {
        if (!is_undefined(self.cb_trans)) {
            self.cb_trans(inserts, deletes);
        }
    });

    spdb_on_table_update(conn, table_name, update_cb);

    return sub_id;
}

function spdb_unsubscribe_all_events(conn) {
    if (!is_struct(conn)) return;

    var new_subs = ds_list_create();
    for (var i = 0; i < ds_list_size(conn.subs); i++) {
        var sub = conn.subs[| i];
        var is_persistent = sub[? "persistent"];
        if (is_persistent) {
            ds_list_add(new_subs, sub);
        } else {
            var qid = sub[? "query_set_id"];
            if (qid >= 0) {
                stdb_unsubscribe_sql(conn.id, qid);
            }
            ds_map_destroy(sub);
        }
    }
    ds_list_destroy(conn.subs);
    conn.subs = new_subs;

    var t_keys = ds_map_keys_to_array(conn.table_listeners);
    for (var i = 0; i < array_length(t_keys); i++) {
        var t_name = t_keys[i];
        if (ds_list_find_index(conn.persistent_tables, t_name) == -1) {
            ds_list_clear(conn.table_listeners[? t_name]);
        }
    }
}

function spdb_register_reducer(reducer_name, schema_array) {
    if (!variable_global_exists("spdb_reducer_schemas")) {
        global.spdb_reducer_schemas = ds_map_create();
    }
    global.spdb_reducer_schemas[? reducer_name] = schema_array;
}

/// Register a reducer error schema for BSATN decoding of Err payloads.
/// This enables structured error messages instead of raw hex strings.
///
/// Usage examples:
///   // String error type:
///   spdb_register_reducer_error_schema(conn, "place_bet", "\"string\"");
///
///   // Enum error type with unit and string variants:
///   spdb_register_reducer_error_schema(conn, "move_player",
///       "[{\"name\":\"InvalidOperation_NoValue\",\"type\":\"Unit\"},{\"name\":\"InsufficientFunds\",\"type\":\"String\"}]");
///
///   // Struct error type (must also register the struct via spdb_register_struct):
///   spdb_register_reducer_error_schema(conn, "trade", "{\"type\":\"TradeError\"}");
///
function spdb_register_reducer_error_schema(conn, reducer_name, schema_json) {
    if (is_undefined(conn) || !variable_struct_exists(conn, "id")) return -1;
    var __args_buffer = __ext_core_get_args_buffer();
    buffer_write(__args_buffer, buffer_f64, conn.id);
    buffer_write(__args_buffer, buffer_u32, string_byte_length(reducer_name));
    buffer_write(__args_buffer, buffer_string, reducer_name);
    __ext_core_buffer_marshal_value(__args_buffer, schema_json);
    return __stdb_register_reducer_error_schema(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
}

function spdb_register_procedure(procedure_name, schema_array) {
    if (!variable_global_exists("spdb_procedure_schemas")) {
        global.spdb_procedure_schemas = ds_map_create();
    }
    global.spdb_procedure_schemas[? procedure_name] = schema_array;
}

/// Enable auto-reconnect with exponential backoff.
/// First attempt is immediate (0ms), then base_delay, 2*base_delay, 4*base_delay...
/// capped at max_delay_ms.
function spdb_set_auto_reconnect(conn, enabled, max_attempts, base_delay_ms, max_delay_ms) {
	if (is_undefined(conn)) return;
    if (is_undefined(max_attempts)) max_attempts = 10;
    if (is_undefined(base_delay_ms)) base_delay_ms = 1000;
    if (is_undefined(max_delay_ms)) max_delay_ms = 30000;
    return stdb_set_auto_reconnect(conn.id, enabled ? 1.0 : 0.0, max_attempts, base_delay_ms, max_delay_ms);
}

/// Swap auth token and reconnect, restoring subscriptions automatically.
/// Use after anonymous connect → login/register when the server issues a new token.
/// Pass "" to clear the token (anonymous reconnect).
function spdb_reconnect_with_token(conn, new_token) {
	if (is_undefined(conn)) return -1;
    if (is_undefined(new_token)) new_token = "";
    return stdb_reconnect_with_token(conn.id, new_token);
}

function spdb_register_struct(conn, struct_name, schema_array) {
    if (is_undefined(conn) || !variable_struct_exists(conn, "id")) return -1;

    if (!variable_global_exists("spdb_struct_schemas")) {
        global.spdb_struct_schemas = ds_map_create();
    }

    global.spdb_struct_schemas[? struct_name] = schema_array;

    var __args_buffer = __ext_core_get_args_buffer();
    buffer_write(__args_buffer, buffer_f64, conn.id);
    buffer_write(__args_buffer, buffer_u32, string_byte_length(struct_name));
    buffer_write(__args_buffer, buffer_string, struct_name);
    __ext_core_buffer_marshal_value(__args_buffer, schema_array);
    return __stdb_register_struct_schema(buffer_get_address(__args_buffer), buffer_tell(__args_buffer));
}

/// Debug: print all current subscriptions, table listeners, and connection state to the log.
/// Call this from any room/object to diagnose subscription or polling issues.
/// Usage: spdb_debug_print_state(global.spdb_connection);
function spdb_debug_print_state(conn) {
    if (is_undefined(conn) || !variable_struct_exists(conn, "id")) {
        show_debug_message("[SPDB DEBUG] conn is invalid or missing 'id'");
        return;
    }

    show_debug_message("========== SPDB DEBUG STATE ==========");
    show_debug_message($"[SPDB DEBUG] conn.id = {conn.id}");
    show_debug_message($"[SPDB DEBUG] conn.connected = {conn.connected}");
    show_debug_message($"[SPDB DEBUG] conn.uri = {conn.uri}");
    show_debug_message($"[SPDB DEBUG] conn.db = {conn.db}");
    show_debug_message($"[SPDB DEBUG] conn.last_error = {conn.last_error}");
    show_debug_message($"[SPDB DEBUG] conn.next_request_id = {conn.next_request_id}");

    // Subscriptions
    var sub_count = ds_list_size(conn.subs);
    show_debug_message($"[SPDB DEBUG] Subscriptions count: {sub_count}");
    for (var i = 0; i < sub_count; i++) {
        var sub = conn.subs[| i];
        var qid = sub[? "query_set_id"];
        var sql = sub[? "sql"];
        var is_persistent = ds_map_exists(sub, "persistent") ? sub[? "persistent"] : false;
        var has_cb = ds_map_exists(sub, "on_applied") && !is_undefined(sub[? "on_applied"]) && sub[? "on_applied"] != -1;
        if (sql == "BATCH_SUBSCRIBE" && ds_map_exists(sub, "sql_array")) {
            var arr = sub[? "sql_array"];
            var arr_len = is_array(arr) ? array_length(arr) : 0;
            show_debug_message($"[SPDB DEBUG]   sub[{i}]: qid={qid}, BATCH_SUBSCRIBE ({arr_len} queries), persistent={is_persistent}, has_callback={has_cb}");
            for (var j = 0; j < arr_len; j++) {
                show_debug_message($"[SPDB DEBUG]     [{j}]: {arr[j]}");
            }
        } else {
            show_debug_message($"[SPDB DEBUG]   sub[{i}]: qid={qid}, sql=\"{sql}\", persistent={is_persistent}, has_callback={has_cb}");
        }
    }

    // Table listeners
    var t_keys = ds_map_keys_to_array(conn.table_listeners);
    show_debug_message($"[SPDB DEBUG] Table listeners count: {array_length(t_keys)}");
    for (var i = 0; i < array_length(t_keys); i++) {
        var t_name = t_keys[i];
        var list = conn.table_listeners[? t_name];
        var cb_count = ds_list_size(list);
        show_debug_message($"[SPDB DEBUG]   table \"{t_name}\": {cb_count} callback(s)");
    }

    // Persistent tables
    var pt_count = ds_list_size(conn.persistent_tables);
    show_debug_message($"[SPDB DEBUG] Persistent tables count: {pt_count}");
    for (var i = 0; i < pt_count; i++) {
        show_debug_message($"[SPDB DEBUG]   persistent[{i}]: \"{ds_list_find_value(conn.persistent_tables, i)}\"");
    }

    // Pending requests
    var p_keys = ds_map_keys_to_array(conn.pending);
    show_debug_message($"[SPDB DEBUG] Pending requests count: {array_length(p_keys)}");
    for (var i = 0; i < array_length(p_keys); i++) {
        show_debug_message($"[SPDB DEBUG]   pending rid={p_keys[i]}");
    }

    // Queued calls
    var q_count = ds_list_size(conn.queue);
    show_debug_message($"[SPDB DEBUG] Queued calls count: {q_count}");
    for (var i = 0; i < q_count; i++) {
        var q = conn.queue[| i];
        show_debug_message($"[SPDB DEBUG]   queue[{i}]: kind={q[? "kind"]}, name={q[? "name"]}, rid={q[? "request_id"]}");
    }

    // Event listeners
    var ev_keys = ds_map_keys_to_array(conn.listeners);
    show_debug_message($"[SPDB DEBUG] Event listener categories count: {array_length(ev_keys)}");
    for (var i = 0; i < array_length(ev_keys); i++) {
        var ev_name = ev_keys[i];
        var list = conn.listeners[? ev_name];
        var cb_count = ds_list_size(list);
        if (cb_count > 0) {
            show_debug_message($"[SPDB DEBUG]   event \"{ev_name}\": {cb_count} listener(s)");
        }
    }

    // Check Rust-side state
    var last_err = stdb_get_last_error(conn.id);
    show_debug_message($"[SPDB DEBUG] Rust last_error: {last_err}");

    // Rust-side internal state (subscription maps, connection flags, etc.)
    var __ret_buffer = __ext_core_get_ret_buffer(65536);
    if (__stdb_debug_state(conn.id, buffer_get_address(__ret_buffer), buffer_get_size(__ret_buffer)) == 0) {
        buffer_seek(__ret_buffer, buffer_seek_start, 0);
        var rust_state = __ext_core_buffer_unmarshal_value(__ret_buffer, []);
        if (is_struct(rust_state)) {
            show_debug_message($"[SPDB DEBUG] Rust internal state: {json_stringify(rust_state)}");
        }
    }

    show_debug_message("========== SPDB DEBUG STATE END ==========");
}

/// Native cache: number of rows for a subscribed table.
function spdb_table_count(conn, table_name) {
	if (is_undefined(conn)) return -1;
    return stdb_table_count(conn.id, table_name);
}

/// Native cache: array of all decoded rows for a table.
function spdb_table_iter(conn, table_name) {
	if (is_undefined(conn)) return [];
    var __ret_buffer = __ext_core_get_ret_buffer(1024 * 1024);
    var rc = __stdb_table_iter(
        conn.id,
        table_name,
        buffer_get_address(__ret_buffer),
        buffer_get_size(__ret_buffer)
    );
    if (rc != 0) return [];
    buffer_seek(__ret_buffer, buffer_seek_start, 0);
    var rows = __ext_core_buffer_unmarshal_value(__ret_buffer, []);
    return is_array(rows) ? rows : [];
}

/// Native cache: find one row by primary key (real or string). Empty struct if missing.
function spdb_table_find(conn, table_name, pk) {
	if (is_undefined(conn)) return {};
    var __args_buffer = __ext_core_get_args_buffer();
    buffer_write(__args_buffer, buffer_f64, conn.id);
    buffer_write(__args_buffer, buffer_u32, string_byte_length(table_name));
    buffer_write(__args_buffer, buffer_string, table_name);
    __ext_core_buffer_marshal_value(__args_buffer, pk);
    var __ret_buffer = __ext_core_get_ret_buffer(65536);
    var rc = __stdb_table_find(
        buffer_get_address(__args_buffer), buffer_tell(__args_buffer),
        buffer_get_address(__ret_buffer), buffer_get_size(__ret_buffer)
    );
    if (rc != 0) return {};
    buffer_seek(__ret_buffer, buffer_seek_start, 0);
    var row = __ext_core_buffer_unmarshal_value(__ret_buffer, []);
    return is_struct(row) ? row : {};
}
