// ##### extgen :: Auto-generated file do not edit!! #####

#import "SpacetimeDBInternal_ios.h"
#import <objc/runtime.h>

extern "C" {

char* __EXT_NATIVE__stdb_ping(void);
double __EXT_NATIVE__stdb_create_client(void);
double __EXT_NATIVE__stdb_destroy_client(double handle);
double __EXT_NATIVE__stdb_connect_simple(double handle, char* uri, char* db_name_or_address, char* auth_token_or_null);
double __EXT_NATIVE__stdb_disconnect(double handle);
double __EXT_NATIVE__stdb_reconnect_with_token(double handle, char* new_token);
double __EXT_NATIVE__stdb_poll_event(double handle, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__stdb_poll_events_batch(double handle, char* __ret_buffer, double __ret_buffer_length);
char* __EXT_NATIVE__stdb_get_last_error(double handle);
double __EXT_NATIVE__stdb_debug_state(double handle, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__stdb_set_auto_reconnect(double handle, double enabled, double max_attempts, double base_delay_ms, double max_delay_ms);
double __EXT_NATIVE__stdb_set_compression_mode(double handle, char* mode);
double __EXT_NATIVE__stdb_set_log_level(double level);
double __EXT_NATIVE__stdb_set_default_request_timeout_ms(double handle, double timeout_ms);
double __EXT_NATIVE__stdb_set_meta_events(double handle, double enabled);
double __EXT_NATIVE__stdb_cancel_request(double handle, double request_id);
double __EXT_NATIVE__stdb_register_schema(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_register_schemas(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_register_struct_schema(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_register_reducer_error_schema(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_subscribe_sql(double handle, char* sql);
double __EXT_NATIVE__stdb_subscribe_all(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_unsubscribe_sql(double handle, double query_set_id);
double __EXT_NATIVE__stdb_call_reducer(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_call_procedure(char* __arg_buffer, double __arg_buffer_length);
double __EXT_NATIVE__stdb_table_count(double handle, char* table_name);
double __EXT_NATIVE__stdb_table_iter(double handle, char* table_name, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__stdb_table_find(char* __arg_buffer, double __arg_buffer_length, char* __ret_buffer, double __ret_buffer_length);
double __EXT_NATIVE__SpacetimeDB_queue_buffer(char* __arg_buffer, double __arg_buffer_length);
const char* __EXT_NATIVE__SpacetimeDB_get_last_error(void);
}


static BOOL GMIsSubclassOf(Class cls, Class base)
{
    for (Class c = cls; c != Nil; c = class_getSuperclass(c)) {
        if (c == base) return YES;
    }
    return NO;
}

static void GMInjectSelectorsIntoSubclass(Class subclass, Class base)
{
    // Build set of methods already defined on subclass
    unsigned subCount = 0;
    Method *subList = class_copyMethodList(subclass, &subCount);

    CFMutableSetRef owned = CFSetCreateMutable(kCFAllocatorDefault, 0, NULL);
    for (unsigned i = 0; i < subCount; ++i) {
        CFSetAddValue(owned, method_getName(subList[i]));
    }

    // Walk base class methods
    unsigned baseCount = 0;
    Method *baseList = class_copyMethodList(base, &baseCount);

    for (unsigned i = 0; i < baseCount; ++i) {
        SEL sel = method_getName(baseList[i]);
        const char *name = sel_getName(sel);

        // Only inject extension selectors (methods prefixed with __EXT_NATIVE__)
        if (!name || strncmp(name, "__EXT_NATIVE__", 13) != 0) continue;

        // Add only if subclass doesn't already have it
        if (!CFSetContainsValue(owned, sel)) {
            IMP imp = method_getImplementation(baseList[i]);
            const char *types = method_getTypeEncoding(baseList[i]);
            if (class_addMethod(subclass, sel, imp, types)) {
                CFSetAddValue(owned, sel);
            }
        }
    }

    if (subList) free(subList);
    if (baseList) free(baseList);
    if (owned) CFRelease(owned);
}

@implementation SpacetimeDBInternal

+ (void)load
{
    // Find all loaded classes
    int num = objc_getClassList(NULL, 0);
    if (num <= 0) return;

    Class *classes = (Class *)malloc(sizeof(Class) * (unsigned)num);
    num = objc_getClassList(classes, num);

    Class base = [SpacetimeDBInternal class];

    for (int i = 0; i < num; ++i) {
        Class cls = classes[i];
        if (cls == base) continue;

        // We only care about direct or indirect subclasses
        if (GMIsSubclassOf(cls, base)) {
            GMInjectSelectorsIntoSubclass(cls, base);
        }
    }

    free(classes);
}

- (char*)__EXT_NATIVE__stdb_ping
{
    return __EXT_NATIVE__stdb_ping();
}
- (double)__EXT_NATIVE__stdb_create_client
{
    return __EXT_NATIVE__stdb_create_client();
}
- (double)__EXT_NATIVE__stdb_destroy_client:(double)handle
{
    return __EXT_NATIVE__stdb_destroy_client(handle);
}
- (double)__EXT_NATIVE__stdb_connect_simple:(double)handle arg1:(char*)uri arg2:(char*)db_name_or_address arg3:(char*)auth_token_or_null
{
    return __EXT_NATIVE__stdb_connect_simple(handle, uri, db_name_or_address, auth_token_or_null);
}
- (double)__EXT_NATIVE__stdb_disconnect:(double)handle
{
    return __EXT_NATIVE__stdb_disconnect(handle);
}
- (double)__EXT_NATIVE__stdb_reconnect_with_token:(double)handle arg1:(char*)new_token
{
    return __EXT_NATIVE__stdb_reconnect_with_token(handle, new_token);
}
- (double)__EXT_NATIVE__stdb_poll_event:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length
{
    return __EXT_NATIVE__stdb_poll_event(handle, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__stdb_poll_events_batch:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length
{
    return __EXT_NATIVE__stdb_poll_events_batch(handle, __ret_buffer, __ret_buffer_length);
}
- (char*)__EXT_NATIVE__stdb_get_last_error:(double)handle
{
    return __EXT_NATIVE__stdb_get_last_error(handle);
}
- (double)__EXT_NATIVE__stdb_debug_state:(double)handle arg1:(char*)__ret_buffer arg2:(double)__ret_buffer_length
{
    return __EXT_NATIVE__stdb_debug_state(handle, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__stdb_set_auto_reconnect:(double)handle arg1:(double)enabled arg2:(double)max_attempts arg3:(double)base_delay_ms arg4:(double)max_delay_ms
{
    return __EXT_NATIVE__stdb_set_auto_reconnect(handle, enabled, max_attempts, base_delay_ms, max_delay_ms);
}
- (double)__EXT_NATIVE__stdb_set_compression_mode:(double)handle arg1:(char*)mode
{
    return __EXT_NATIVE__stdb_set_compression_mode(handle, mode);
}
- (double)__EXT_NATIVE__stdb_set_log_level:(double)level
{
    return __EXT_NATIVE__stdb_set_log_level(level);
}
- (double)__EXT_NATIVE__stdb_set_default_request_timeout_ms:(double)handle arg1:(double)timeout_ms
{
    return __EXT_NATIVE__stdb_set_default_request_timeout_ms(handle, timeout_ms);
}
- (double)__EXT_NATIVE__stdb_set_meta_events:(double)handle arg1:(double)enabled
{
    return __EXT_NATIVE__stdb_set_meta_events(handle, enabled);
}
- (double)__EXT_NATIVE__stdb_cancel_request:(double)handle arg1:(double)request_id
{
    return __EXT_NATIVE__stdb_cancel_request(handle, request_id);
}
- (double)__EXT_NATIVE__stdb_register_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_register_schema(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_register_schemas:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_register_schemas(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_register_struct_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_register_struct_schema(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_register_reducer_error_schema:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_register_reducer_error_schema(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_subscribe_sql:(double)handle arg1:(char*)sql
{
    return __EXT_NATIVE__stdb_subscribe_sql(handle, sql);
}
- (double)__EXT_NATIVE__stdb_subscribe_all:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_subscribe_all(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_unsubscribe_sql:(double)handle arg1:(double)query_set_id
{
    return __EXT_NATIVE__stdb_unsubscribe_sql(handle, query_set_id);
}
- (double)__EXT_NATIVE__stdb_call_reducer:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_call_reducer(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_call_procedure:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__stdb_call_procedure(__arg_buffer, __arg_buffer_length);
}
- (double)__EXT_NATIVE__stdb_table_count:(double)handle arg1:(char*)table_name
{
    return __EXT_NATIVE__stdb_table_count(handle, table_name);
}
- (double)__EXT_NATIVE__stdb_table_iter:(double)handle arg1:(char*)table_name arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length
{
    return __EXT_NATIVE__stdb_table_iter(handle, table_name, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__stdb_table_find:(char*)__arg_buffer arg1:(double)__arg_buffer_length arg2:(char*)__ret_buffer arg3:(double)__ret_buffer_length
{
    return __EXT_NATIVE__stdb_table_find(__arg_buffer, __arg_buffer_length, __ret_buffer, __ret_buffer_length);
}
- (double)__EXT_NATIVE__SpacetimeDB_queue_buffer:(char*)__arg_buffer arg1:(double)__arg_buffer_length
{
    return __EXT_NATIVE__SpacetimeDB_queue_buffer(__arg_buffer, __arg_buffer_length);
}
@end

