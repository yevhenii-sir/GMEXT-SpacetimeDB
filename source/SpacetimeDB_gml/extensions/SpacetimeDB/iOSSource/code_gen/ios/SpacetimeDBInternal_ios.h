// ##### extgen :: Auto-generated file do not edit!! #####

#import <Foundation/Foundation.h>

@interface SpacetimeDBInternal : NSObject
- (char*)__EXT_NATIVE__stdb_ping;
- (double)__EXT_NATIVE__stdb_create_client;
- (double)__EXT_NATIVE__stdb_destroy_client:(double)handle;
- (double)__EXT_NATIVE__stdb_connect_simple:(double)handle arg1:(char*)uri arg2:(char*)db_name_or_address arg3:(char*)auth_token_or_null;
- (double)__EXT_NATIVE__stdb_disconnect:(double)handle;
- (double)__EXT_NATIVE__stdb_reconnect_with_token:(double)handle arg1:(char*)new_token;
- (double)__EXT_NATIVE__stdb_poll_event:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__stdb_poll_events_batch:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length;
- (char*)__EXT_NATIVE__stdb_get_last_error:(double)handle;
- (double)__EXT_NATIVE__stdb_debug_state:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__stdb_set_auto_reconnect:(double)handle arg1:(double)enabled arg2:(double)max_attempts arg3:(double)base_delay_ms arg4:(double)max_delay_ms;
- (double)__EXT_NATIVE__stdb_set_compression_mode:(double)handle arg1:(char*)mode;
- (double)__EXT_NATIVE__stdb_set_log_level:(double)level;
- (double)__EXT_NATIVE__stdb_set_default_request_timeout_ms:(double)handle arg1:(double)timeout_ms;
- (double)__EXT_NATIVE__stdb_set_meta_events:(double)handle arg1:(double)enabled;
- (double)__EXT_NATIVE__stdb_cancel_request:(double)handle arg1:(double)request_id;
- (double)__EXT_NATIVE__stdb_register_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_register_schemas:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_register_struct_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_register_reducer_error_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_subscribe_sql:(double)handle arg1:(char*)sql;
- (double)__EXT_NATIVE__stdb_subscribe_all:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_unsubscribe_sql:(double)handle arg1:(double)query_set_id;
- (double)__EXT_NATIVE__stdb_call_reducer:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_call_procedure:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
- (double)__EXT_NATIVE__stdb_table_count:(double)handle arg1:(char*)table_name;
- (double)__EXT_NATIVE__stdb_table_iter:(double)handle arg1:(char*)table_name arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__stdb_table_find:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length;
- (double)__EXT_NATIVE__SpacetimeDB_queue_buffer:(char*)__arg_buffer arg1:(double)__arg_buffer_length;
@end

