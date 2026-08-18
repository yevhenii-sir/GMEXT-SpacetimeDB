// ##### extgen :: Auto-generated file do not edit!! #####

#import <Foundation/Foundation.h>

@interface SpacetimeDBInternal : NSObject
- (char*)__EXT_NATIVE__stdb_ping;
- (double)__EXT_NATIVE__stdb_create_client;
- (double)__EXT_NATIVE__stdb_destroy_client:(double)handle;
- (double)__EXT_NATIVE__stdb_connect_simple:(double)handle arg1:(char*)uri arg2:(char*)db_name_or_address arg3:(char*)auth_token_or_null;
- (double)__EXT_NATIVE__stdb_disconnect:(double)handle;
- (double)__EXT_NATIVE__stdb_reconnect_with_token:(double)handle arg1:(char*)new_token;
- (char*)__EXT_NATIVE__stdb_poll_event_json:(double)handle;
- (char*)__EXT_NATIVE__stdb_poll_events_batch_json:(double)handle;
- (char*)__EXT_NATIVE__stdb_get_last_error:(double)handle;
- (char*)__EXT_NATIVE__stdb_debug_state_json:(double)handle;
- (double)__EXT_NATIVE__stdb_set_auto_reconnect:(double)handle arg1:(double)enabled arg2:(double)max_attempts arg3:(double)base_delay_ms arg4:(double)max_delay_ms;
- (double)__EXT_NATIVE__stdb_set_compression_mode:(double)handle arg1:(char*)mode;
- (double)__EXT_NATIVE__stdb_set_log_level:(double)level;
- (double)__EXT_NATIVE__stdb_set_default_request_timeout_ms:(double)handle arg1:(double)timeout_ms;
- (double)__EXT_NATIVE__stdb_cancel_request:(double)handle arg1:(double)request_id;
- (double)__EXT_NATIVE__stdb_register_schema_json:(double)handle arg1:(char*)table_name arg2:(char*)schema_json;
- (double)__EXT_NATIVE__stdb_register_schemas_json:(double)handle arg1:(char*)all_schemas_json;
- (double)__EXT_NATIVE__stdb_register_struct_schema_json:(double)handle arg1:(char*)struct_name arg2:(char*)schema_json;
- (double)__EXT_NATIVE__stdb_register_reducer_error_schema:(double)handle arg1:(char*)reducer_name arg2:(char*)schema_json;
- (double)__EXT_NATIVE__stdb_subscribe_sql:(double)handle arg1:(char*)sql;
- (double)__EXT_NATIVE__stdb_subscribe_all_json:(double)handle arg1:(char*)json_payload;
- (double)__EXT_NATIVE__stdb_unsubscribe_sql:(double)handle arg1:(double)query_set_id;
- (double)__EXT_NATIVE__stdb_call_reducer_bsatn:(double)handle arg1:(char*)name_and_id arg2:(char*)b64_args;
- (double)__EXT_NATIVE__stdb_call_procedure_bsatn:(double)handle arg1:(char*)name_and_id arg2:(char*)b64_args;
@end

