// ##### extgen :: Auto-generated file do not edit!! #####

#import "SpacetimeDBInternal_tvos.h"
#import "native/SpacetimeDBInternal_exports.h"
#import <objc/runtime.h>


extern "C" const char* extOptGetString(char* _ext, char* _opt);

// Adapter: matches const signature expected by the C++ API
static const char* ExtOptGetString(const char* ext, const char* opt)
{
    return extOptGetString(const_cast<char*>(ext), const_cast<char*>(opt));
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

    gm::details::GMRTRunnerInterface ri{};
    ri.ExtOptGetString = &ExtOptGetString;
    GMExtensionInitialise(&ri, sizeof(ri));
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
- (char*)__EXT_NATIVE__stdb_poll_event_json:(double)handle
{
    return __EXT_NATIVE__stdb_poll_event_json(handle);
}
- (char*)__EXT_NATIVE__stdb_poll_events_batch_json:(double)handle
{
    return __EXT_NATIVE__stdb_poll_events_batch_json(handle);
}
- (char*)__EXT_NATIVE__stdb_get_last_error:(double)handle
{
    return __EXT_NATIVE__stdb_get_last_error(handle);
}
- (char*)__EXT_NATIVE__stdb_debug_state_json:(double)handle
{
    return __EXT_NATIVE__stdb_debug_state_json(handle);
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
- (double)__EXT_NATIVE__stdb_cancel_request:(double)handle arg1:(double)request_id
{
    return __EXT_NATIVE__stdb_cancel_request(handle, request_id);
}
- (double)__EXT_NATIVE__stdb_register_schema_json:(double)handle arg1:(char*)table_name arg2:(char*)schema_json
{
    return __EXT_NATIVE__stdb_register_schema_json(handle, table_name, schema_json);
}
- (double)__EXT_NATIVE__stdb_register_schemas_json:(double)handle arg1:(char*)all_schemas_json
{
    return __EXT_NATIVE__stdb_register_schemas_json(handle, all_schemas_json);
}
- (double)__EXT_NATIVE__stdb_register_struct_schema_json:(double)handle arg1:(char*)struct_name arg2:(char*)schema_json
{
    return __EXT_NATIVE__stdb_register_struct_schema_json(handle, struct_name, schema_json);
}
- (double)__EXT_NATIVE__stdb_register_reducer_error_schema:(double)handle arg1:(char*)reducer_name arg2:(char*)schema_json
{
    return __EXT_NATIVE__stdb_register_reducer_error_schema(handle, reducer_name, schema_json);
}
- (double)__EXT_NATIVE__stdb_subscribe_sql:(double)handle arg1:(char*)sql
{
    return __EXT_NATIVE__stdb_subscribe_sql(handle, sql);
}
- (double)__EXT_NATIVE__stdb_subscribe_all_json:(double)handle arg1:(char*)json_payload
{
    return __EXT_NATIVE__stdb_subscribe_all_json(handle, json_payload);
}
- (double)__EXT_NATIVE__stdb_unsubscribe_sql:(double)handle arg1:(double)query_set_id
{
    return __EXT_NATIVE__stdb_unsubscribe_sql(handle, query_set_id);
}
- (double)__EXT_NATIVE__stdb_call_reducer_bsatn:(double)handle arg1:(char*)name_and_id arg2:(char*)b64_args
{
    return __EXT_NATIVE__stdb_call_reducer_bsatn(handle, name_and_id, b64_args);
}
- (double)__EXT_NATIVE__stdb_call_procedure_bsatn:(double)handle arg1:(char*)name_and_id arg2:(char*)b64_args
{
    return __EXT_NATIVE__stdb_call_procedure_bsatn(handle, name_and_id, b64_args);
}
@end

