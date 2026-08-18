// ##### extgen :: Auto-generated Android JNI bridge (Rust) #####
#![allow(non_snake_case)]

use jni::objects::{JByteBuffer, JClass, JObject, JString, JValue};
use jni::sys::{jdouble, jint, jstring, JNI_VERSION_1_6};
use jni::{JNIEnv, JavaVM, NativeMethod};
use std::ffi::c_void;
use std::os::raw::c_char;

use crate::generated::ffi;

fn direct_buf_ptr(env: &mut JNIEnv<'_>, buf: JObject<'_>) -> Option<*mut c_char> {
    let bb = unsafe { JByteBuffer::from_raw(buf.as_raw()) };
    env.get_direct_buffer_address(&bb).ok().map(|p| p as *mut c_char)
}

extern "system" fn jni_wrap_stdb_ping(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_ping() };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

extern "system" fn jni_wrap_stdb_create_client(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,

) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_create_client() };
    result
}

extern "system" fn jni_wrap_stdb_destroy_client(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_destroy_client(handle) };
    result
}

extern "system" fn jni_wrap_stdb_connect_simple(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    uri: JString<'_>,
    db_name_or_address: JString<'_>,
    auth_token_or_null: JString<'_>
) -> jdouble {
    let uri_c = env.get_string(&uri).ok();
    let uri_ptr = uri_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let db_name_or_address_c = env.get_string(&db_name_or_address).ok();
    let db_name_or_address_ptr = db_name_or_address_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let auth_token_or_null_c = env.get_string(&auth_token_or_null).ok();
    let auth_token_or_null_ptr = auth_token_or_null_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__stdb_connect_simple(handle, uri_ptr, db_name_or_address_ptr, auth_token_or_null_ptr) };
    result
}

extern "system" fn jni_wrap_stdb_disconnect(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_disconnect(handle) };
    result
}

extern "system" fn jni_wrap_stdb_reconnect_with_token(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    new_token: JString<'_>
) -> jdouble {
    let new_token_c = env.get_string(&new_token).ok();
    let new_token_ptr = new_token_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__stdb_reconnect_with_token(handle, new_token_ptr) };
    result
}

extern "system" fn jni_wrap_stdb_poll_event(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_poll_event(handle, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_poll_events_batch(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_poll_events_batch(handle, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_get_last_error(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble
) -> jstring {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_get_last_error(handle) };
    if result.is_null() { return std::ptr::null_mut(); }
    let cstr = unsafe { std::ffi::CStr::from_ptr(result) };
    match cstr.to_str() {
        Ok(s) => env.new_string(s).map(|js| js.into_raw()).unwrap_or(std::ptr::null_mut()),
        Err(_) => std::ptr::null_mut(),
    }
}

extern "system" fn jni_wrap_stdb_debug_state(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_debug_state(handle, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_set_auto_reconnect(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    enabled: jdouble,
    max_attempts: jdouble,
    base_delay_ms: jdouble,
    max_delay_ms: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_set_auto_reconnect(handle, enabled, max_attempts, base_delay_ms, max_delay_ms) };
    result
}

extern "system" fn jni_wrap_stdb_set_compression_mode(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    mode: JString<'_>
) -> jdouble {
    let mode_c = env.get_string(&mode).ok();
    let mode_ptr = mode_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__stdb_set_compression_mode(handle, mode_ptr) };
    result
}

extern "system" fn jni_wrap_stdb_set_log_level(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    level: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_set_log_level(level) };
    result
}

extern "system" fn jni_wrap_stdb_set_default_request_timeout_ms(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    timeout_ms: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_set_default_request_timeout_ms(handle, timeout_ms) };
    result
}

extern "system" fn jni_wrap_stdb_set_meta_events(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    enabled: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_set_meta_events(handle, enabled) };
    result
}

extern "system" fn jni_wrap_stdb_cancel_request(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    request_id: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_cancel_request(handle, request_id) };
    result
}

extern "system" fn jni_wrap_stdb_register_schema(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_register_schema(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_register_schemas(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_register_schemas(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_register_struct_schema(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_register_struct_schema(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_register_reducer_error_schema(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_register_reducer_error_schema(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_subscribe_sql(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    sql: JString<'_>
) -> jdouble {
    let sql_c = env.get_string(&sql).ok();
    let sql_ptr = sql_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__stdb_subscribe_sql(handle, sql_ptr) };
    result
}

extern "system" fn jni_wrap_stdb_subscribe_all(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_subscribe_all(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_unsubscribe_sql(
    _env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    query_set_id: jdouble
) -> jdouble {
    let result = unsafe { ffi::__EXT_NATIVE__stdb_unsubscribe_sql(handle, query_set_id) };
    result
}

extern "system" fn jni_wrap_stdb_call_reducer(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_call_reducer(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_call_procedure(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_call_procedure(__arg_buffer_ptr, __arg_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_table_count(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    table_name: JString<'_>
) -> jdouble {
    let table_name_c = env.get_string(&table_name).ok();
    let table_name_ptr = table_name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let result = unsafe { ffi::__EXT_NATIVE__stdb_table_count(handle, table_name_ptr) };
    result
}

extern "system" fn jni_wrap_stdb_table_iter(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    handle: jdouble,
    table_name: JString<'_>,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let table_name_c = env.get_string(&table_name).ok();
    let table_name_ptr = table_name_c.as_ref().map(|s| s.as_ptr()).unwrap_or(std::ptr::null());
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_table_iter(handle, table_name_ptr, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

extern "system" fn jni_wrap_stdb_table_find(
    mut env: JNIEnv<'_>,
    _class: JClass<'_>,
    __arg_buffer: JObject<'_>,
    __arg_buffer_length: jdouble,
    __ret_buffer: JObject<'_>,
    __ret_buffer_length: jdouble
) -> jdouble {
    let __arg_buffer_ptr = match direct_buf_ptr(&mut env, __arg_buffer) { Some(p) => p, None => return -1.0 };
    let __ret_buffer_ptr = match direct_buf_ptr(&mut env, __ret_buffer) { Some(p) => p, None => return -1.0 };
    let result = unsafe { ffi::__EXT_NATIVE__stdb_table_find(__arg_buffer_ptr, __arg_buffer_length, __ret_buffer_ptr, __ret_buffer_length) };
    result
}

#[no_mangle]
pub extern "system" fn Java_com_gamemaker_ExtensionCore_ExtBridge_SpacetimeDBBridge_nativeRegister(mut env: JNIEnv<'_>, class: JClass<'_>) {
    let methods = [
        NativeMethod { name: "__EXT_JNI__stdb_ping".into(), sig: "()Ljava/lang/String;".into(), fn_ptr: jni_wrap_stdb_ping as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_create_client".into(), sig: "()D".into(), fn_ptr: jni_wrap_stdb_create_client as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_destroy_client".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_stdb_destroy_client as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_connect_simple".into(), sig: "(DLjava/lang/String;Ljava/lang/String;Ljava/lang/String;)D".into(), fn_ptr: jni_wrap_stdb_connect_simple as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_disconnect".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_stdb_disconnect as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_reconnect_with_token".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_stdb_reconnect_with_token as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_poll_event".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_poll_event as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_poll_events_batch".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_poll_events_batch as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_get_last_error".into(), sig: "(D)Ljava/lang/String;".into(), fn_ptr: jni_wrap_stdb_get_last_error as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_debug_state".into(), sig: "(DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_debug_state as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_set_auto_reconnect".into(), sig: "(DDDDD)D".into(), fn_ptr: jni_wrap_stdb_set_auto_reconnect as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_set_compression_mode".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_stdb_set_compression_mode as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_set_log_level".into(), sig: "(D)D".into(), fn_ptr: jni_wrap_stdb_set_log_level as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_set_default_request_timeout_ms".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_stdb_set_default_request_timeout_ms as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_set_meta_events".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_stdb_set_meta_events as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_cancel_request".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_stdb_cancel_request as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_register_schema".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_register_schema as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_register_schemas".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_register_schemas as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_register_struct_schema".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_register_struct_schema as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_register_reducer_error_schema".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_register_reducer_error_schema as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_subscribe_sql".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_stdb_subscribe_sql as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_subscribe_all".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_subscribe_all as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_unsubscribe_sql".into(), sig: "(DD)D".into(), fn_ptr: jni_wrap_stdb_unsubscribe_sql as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_call_reducer".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_call_reducer as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_call_procedure".into(), sig: "(Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_call_procedure as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_table_count".into(), sig: "(DLjava/lang/String;)D".into(), fn_ptr: jni_wrap_stdb_table_count as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_table_iter".into(), sig: "(DLjava/lang/String;Ljava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_table_iter as *mut c_void },
        NativeMethod { name: "__EXT_JNI__stdb_table_find".into(), sig: "(Ljava/nio/ByteBuffer;DLjava/nio/ByteBuffer;D)D".into(), fn_ptr: jni_wrap_stdb_table_find as *mut c_void },
    ];
    let _ = env.register_native_methods(class, &methods);
}

#[no_mangle]
pub extern "system" fn JNI_OnLoad(_vm: JavaVM, _reserved: *mut c_void) -> jint {
    JNI_VERSION_1_6
}

