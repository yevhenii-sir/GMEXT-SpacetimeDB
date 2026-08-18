package com.gamemaker.ExtensionCore.ExtBridge;
import java.lang.String;
import java.nio.ByteBuffer;
import ${YYAndroidPackageName}.GMExtUtils;

public final class SpacetimeDBBridge {
    static {
        // this is the extension lib name
        System.loadLibrary("SpacetimeDB");
        nativeRegister();
    }
    // this registers the native functions on the C++ layer
    private static native void nativeRegister();

    public static String __EXT_JAVA__GetExtensionOption(String extName, String optName)
    {
        return GMExtUtils.GetExtensionOption(extName, optName);
    }

    public static native double __EXT_JNI__SpacetimeDB_queue_buffer(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native String __EXT_JNI__stdb_ping();
    public static native double __EXT_JNI__stdb_create_client();
    public static native double __EXT_JNI__stdb_destroy_client(double handle);
    public static native double __EXT_JNI__stdb_connect_simple(double handle, String uri, String db_name_or_address, String auth_token_or_null);
    public static native double __EXT_JNI__stdb_disconnect(double handle);
    public static native double __EXT_JNI__stdb_reconnect_with_token(double handle, String new_token);
    public static native double __EXT_JNI__stdb_poll_event(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__stdb_poll_events_batch(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native String __EXT_JNI__stdb_get_last_error(double handle);
    public static native double __EXT_JNI__stdb_debug_state(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__stdb_set_auto_reconnect(double handle, double enabled, double max_attempts, double base_delay_ms, double max_delay_ms);
    public static native double __EXT_JNI__stdb_set_compression_mode(double handle, String mode);
    public static native double __EXT_JNI__stdb_set_log_level(double level);
    public static native double __EXT_JNI__stdb_set_default_request_timeout_ms(double handle, double timeout_ms);
    public static native double __EXT_JNI__stdb_set_meta_events(double handle, double enabled);
    public static native double __EXT_JNI__stdb_cancel_request(double handle, double request_id);
    public static native double __EXT_JNI__stdb_register_schema(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_register_schemas(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_register_struct_schema(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_register_reducer_error_schema(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_subscribe_sql(double handle, String sql);
    public static native double __EXT_JNI__stdb_subscribe_all(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_unsubscribe_sql(double handle, double query_set_id);
    public static native double __EXT_JNI__stdb_call_reducer(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_call_procedure(ByteBuffer __arg_buffer, double __arg_buffer_length);
    public static native double __EXT_JNI__stdb_table_count(double handle, String table_name);
    public static native double __EXT_JNI__stdb_table_iter(double handle, String table_name, ByteBuffer __ret_buffer, double __ret_buffer_length);
    public static native double __EXT_JNI__stdb_table_find(ByteBuffer __arg_buffer, double __arg_buffer_length, ByteBuffer __ret_buffer, double __ret_buffer_length);
}