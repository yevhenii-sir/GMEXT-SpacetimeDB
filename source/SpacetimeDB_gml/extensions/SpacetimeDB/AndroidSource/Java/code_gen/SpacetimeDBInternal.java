package ${YYAndroidPackageName};
import static com.gamemaker.ExtensionCore.ExtBridge.SpacetimeDBBridge.*;
import java.lang.String;
import java.nio.ByteBuffer;

public class SpacetimeDBInternal extends RunnerSocial {
    public double __EXT_NATIVE__SpacetimeDB_queue_buffer(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__SpacetimeDB_queue_buffer(__arg_buffer, __arg_buffer_length);
    }
    public String __EXT_NATIVE__stdb_ping()
    {
        return __EXT_JNI__stdb_ping();
    }
    public double __EXT_NATIVE__stdb_create_client()
    {
        return __EXT_JNI__stdb_create_client();
    }
    public double __EXT_NATIVE__stdb_destroy_client(double handle)
    {
        return __EXT_JNI__stdb_destroy_client(handle);
    }
    public double __EXT_NATIVE__stdb_connect_simple(double handle, String uri, String db_name_or_address, String auth_token_or_null)
    {
        return __EXT_JNI__stdb_connect_simple(handle, uri, db_name_or_address, auth_token_or_null);
    }
    public double __EXT_NATIVE__stdb_disconnect(double handle)
    {
        return __EXT_JNI__stdb_disconnect(handle);
    }
    public double __EXT_NATIVE__stdb_reconnect_with_token(double handle, String new_token)
    {
        return __EXT_JNI__stdb_reconnect_with_token(handle, new_token);
    }
    public double __EXT_NATIVE__stdb_poll_event(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length)
    {
        return __EXT_JNI__stdb_poll_event(handle, __ret_buffer, __ret_buffer_length);
    }
    public double __EXT_NATIVE__stdb_poll_events_batch(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length)
    {
        return __EXT_JNI__stdb_poll_events_batch(handle, __ret_buffer, __ret_buffer_length);
    }
    public String __EXT_NATIVE__stdb_get_last_error(double handle)
    {
        return __EXT_JNI__stdb_get_last_error(handle);
    }
    public double __EXT_NATIVE__stdb_debug_state(double handle, ByteBuffer __ret_buffer, double __ret_buffer_length)
    {
        return __EXT_JNI__stdb_debug_state(handle, __ret_buffer, __ret_buffer_length);
    }
    public double __EXT_NATIVE__stdb_set_auto_reconnect(double handle, double enabled, double max_attempts, double base_delay_ms, double max_delay_ms)
    {
        return __EXT_JNI__stdb_set_auto_reconnect(handle, enabled, max_attempts, base_delay_ms, max_delay_ms);
    }
    public double __EXT_NATIVE__stdb_set_compression_mode(double handle, String mode)
    {
        return __EXT_JNI__stdb_set_compression_mode(handle, mode);
    }
    public double __EXT_NATIVE__stdb_set_log_level(double level)
    {
        return __EXT_JNI__stdb_set_log_level(level);
    }
    public double __EXT_NATIVE__stdb_set_default_request_timeout_ms(double handle, double timeout_ms)
    {
        return __EXT_JNI__stdb_set_default_request_timeout_ms(handle, timeout_ms);
    }
    public double __EXT_NATIVE__stdb_set_meta_events(double handle, double enabled)
    {
        return __EXT_JNI__stdb_set_meta_events(handle, enabled);
    }
    public double __EXT_NATIVE__stdb_cancel_request(double handle, double request_id)
    {
        return __EXT_JNI__stdb_cancel_request(handle, request_id);
    }
    public double __EXT_NATIVE__stdb_register_schema(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_register_schema(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_register_schemas(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_register_schemas(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_register_struct_schema(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_register_struct_schema(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_register_reducer_error_schema(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_register_reducer_error_schema(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_subscribe_sql(double handle, String sql)
    {
        return __EXT_JNI__stdb_subscribe_sql(handle, sql);
    }
    public double __EXT_NATIVE__stdb_subscribe_all(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_subscribe_all(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_unsubscribe_sql(double handle, double query_set_id)
    {
        return __EXT_JNI__stdb_unsubscribe_sql(handle, query_set_id);
    }
    public double __EXT_NATIVE__stdb_call_reducer(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_call_reducer(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_call_procedure(ByteBuffer __arg_buffer, double __arg_buffer_length)
    {
        return __EXT_JNI__stdb_call_procedure(__arg_buffer, __arg_buffer_length);
    }
    public double __EXT_NATIVE__stdb_table_count(double handle, String table_name)
    {
        return __EXT_JNI__stdb_table_count(handle, table_name);
    }
    public double __EXT_NATIVE__stdb_table_iter(double handle, String table_name, ByteBuffer __ret_buffer, double __ret_buffer_length)
    {
        return __EXT_JNI__stdb_table_iter(handle, table_name, __ret_buffer, __ret_buffer_length);
    }
    public double __EXT_NATIVE__stdb_table_find(ByteBuffer __arg_buffer, double __arg_buffer_length, ByteBuffer __ret_buffer, double __ret_buffer_length)
    {
        return __EXT_JNI__stdb_table_find(__arg_buffer, __arg_buffer_length, __ret_buffer, __ret_buffer_length);
    }
}