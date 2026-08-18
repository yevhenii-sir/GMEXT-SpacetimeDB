// ##### extgen :: Auto-generated file do not edit!! #####

#![allow(non_upper_case_globals)]

use std::ffi::c_char;
use std::panic::catch_unwind;
use gm_ext_wire::{clear_last_error, get_last_error_ptr, set_last_error};
use gm_ext_wire::store_tls_string;
use gm_ext_wire::{GMBufferReader, GMSliceWriter, BufferQueue, GMBuffer};
use crate::user;

static __buffer_queue: BufferQueue = BufferQueue::new();

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__SpacetimeDB_queue_buffer(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __buff = GMBuffer::new(__arg_buffer as *mut u8, __arg_buffer_length as u64);
        __buffer_queue.push(__buff);
        1.0
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__SpacetimeDB_queue_buffer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__SpacetimeDB_get_last_error() -> *const c_char {
    get_last_error_ptr()
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_ping() -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::stdb_ping();
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_ping");
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_create_client() -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_create_client()
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_create_client");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_destroy_client(handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_destroy_client(handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_destroy_client");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_connect_simple(handle: f64, uri: *const c_char, db_name_or_address: *const c_char, auth_token_or_null: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let uri_str = if uri.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(uri) }.to_str().unwrap_or("") };
        let db_name_or_address_str = if db_name_or_address.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(db_name_or_address) }.to_str().unwrap_or("") };
        let auth_token_or_null_str = if auth_token_or_null.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(auth_token_or_null) }.to_str().unwrap_or("") };
        user::stdb_connect_simple(handle, uri_str, db_name_or_address_str, auth_token_or_null_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_connect_simple");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_disconnect(handle: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_disconnect(handle)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_disconnect");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_reconnect_with_token(handle: f64, new_token: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let new_token_str = if new_token.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(new_token) }.to_str().unwrap_or("") };
        user::stdb_reconnect_with_token(handle, new_token_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_reconnect_with_token");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_poll_event(handle: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let __result = user::stdb_poll_event(handle);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_poll_event");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_poll_events_batch(handle: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let __result = user::stdb_poll_events_batch(handle);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_poll_events_batch");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_get_last_error(handle: f64) -> *const c_char {
    match catch_unwind(|| {
        clear_last_error();
        let s = user::stdb_get_last_error(handle);
        store_tls_string(s)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_get_last_error");
            std::ptr::null()
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_debug_state(handle: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let __result = user::stdb_debug_state(handle);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_debug_state");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_set_auto_reconnect(handle: f64, enabled: f64, max_attempts: f64, base_delay_ms: f64, max_delay_ms: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_set_auto_reconnect(handle, enabled, max_attempts, base_delay_ms, max_delay_ms)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_set_auto_reconnect");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_set_compression_mode(handle: f64, mode: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let mode_str = if mode.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(mode) }.to_str().unwrap_or("") };
        user::stdb_set_compression_mode(handle, mode_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_set_compression_mode");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_set_log_level(level: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_set_log_level(level)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_set_log_level");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_set_default_request_timeout_ms(handle: f64, timeout_ms: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_set_default_request_timeout_ms(handle, timeout_ms)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_set_default_request_timeout_ms");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_set_meta_events(handle: f64, enabled: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_set_meta_events(handle, enabled)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_set_meta_events");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_cancel_request(handle: f64, request_id: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_cancel_request(handle, request_id)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_cancel_request");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_register_schema(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let table_name = __br.read_idl_string()?.to_string();
            let schema = { match __br.unpack_value()? { gm_ext_wire::GMValue::Struct(__m) => __m.into_iter().map(|(k, v)| (k.to_string(), v.into_owned())).collect(), _ => return None } };
            Some(user::stdb_register_schema(handle, table_name, schema) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_register_schema");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_register_schemas(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let all_schemas = { match __br.unpack_value()? { gm_ext_wire::GMValue::Struct(__m) => __m.into_iter().map(|(k, v)| (k.to_string(), v.into_owned())).collect(), _ => return None } };
            Some(user::stdb_register_schemas(handle, all_schemas) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_register_schemas");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_register_struct_schema(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let struct_name = __br.read_idl_string()?.to_string();
            let schema = { match __br.unpack_value()? { gm_ext_wire::GMValue::Array(__a) => __a.into_iter().map(|__v| __v.into_owned()).collect(), _ => return None } };
            Some(user::stdb_register_struct_schema(handle, struct_name, schema) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_register_struct_schema");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_register_reducer_error_schema(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let reducer_name = __br.read_idl_string()?.to_string();
            let schema = __br.unpack_value()?.into_owned();
            Some(user::stdb_register_reducer_error_schema(handle, reducer_name, schema) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_register_reducer_error_schema");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_subscribe_sql(handle: f64, sql: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let sql_str = if sql.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(sql) }.to_str().unwrap_or("") };
        user::stdb_subscribe_sql(handle, sql_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_subscribe_sql");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_subscribe_all(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let sqls = { match __br.unpack_value()? { gm_ext_wire::GMValue::Array(__a) => __a.into_iter().map(|__v| __v.into_owned()).collect(), _ => return None } };
            Some(user::stdb_subscribe_all(handle, sqls) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_subscribe_all");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_unsubscribe_sql(handle: f64, query_set_id: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        user::stdb_unsubscribe_sql(handle, query_set_id)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_unsubscribe_sql");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_call_reducer(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let name = __br.read_idl_string()?.to_string();
            let request_id = __br.read_f64()?;
            let args = __buffer_queue.pop_front()?;
            Some(user::stdb_call_reducer(handle, name, request_id, args) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_call_reducer");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_call_procedure(__arg_buffer: *mut c_char, __arg_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let name = __br.read_idl_string()?.to_string();
            let request_id = __br.read_f64()?;
            let args = __buffer_queue.pop_front()?;
            Some(user::stdb_call_procedure(handle, name, request_id, args) as f64)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_call_procedure");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_table_count(handle: f64, table_name: *const c_char) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let table_name_str = if table_name.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(table_name) }.to_str().unwrap_or("") };
        user::stdb_table_count(handle, table_name_str)
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_table_count");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_table_iter(handle: f64, table_name: *const c_char, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let table_name_str = if table_name.is_null() { "" } else { unsafe { std::ffi::CStr::from_ptr(table_name) }.to_str().unwrap_or("") };
            let __result = user::stdb_table_iter(handle, table_name_str);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_table_iter");
            -1.0
        }
    }
}

#[no_mangle]
pub extern "C" fn __EXT_NATIVE__stdb_table_find(__arg_buffer: *mut c_char, __arg_buffer_length: f64, __ret_buffer: *mut c_char, __ret_buffer_length: f64) -> f64 {
    match catch_unwind(|| {
        clear_last_error();
        let __wire: Option<f64> = (|| {
            let mut __br = unsafe { GMBufferReader::from_raw_parts(__arg_buffer as *const u8, __arg_buffer_length as usize) };
            let handle = __br.read_f64()?;
            let table_name = __br.read_idl_string()?.to_string();
            let pk = __br.unpack_value()?.into_owned();
            let __result = user::stdb_table_find(handle, table_name, pk);
            let mut __bw = unsafe { GMSliceWriter::from_raw_parts(__ret_buffer as *mut u8, __ret_buffer_length as usize) };
            __result.write_to(&mut __bw)?;
            Some(0.0)
        })();
        match __wire {
            Some(v) => v,
            None => { set_last_error("wire decode/encode failed"); -1.0 }
        }
    }) {
        Ok(v) => v,
        Err(_) => {
            set_last_error("panic in __EXT_NATIVE__stdb_table_find");
            -1.0
        }
    }
}

