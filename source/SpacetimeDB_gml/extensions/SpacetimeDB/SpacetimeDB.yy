{
  "$GMExtension": "",
  "%Name": "SpacetimeDB",
  "androidactivityinject": null,
  "androidclassname": "SpacetimeDB",
  "androidcodeinjection": "",
  "androidinject": null,
  "androidmanifestinject": null,
  "androidPermissions": [
    "android.permission.INTERNET"
  ],
  "androidProps": true,
  "androidsourcedir": "",
  "author": "",
  "classname": "SpacetimeDB",
  "copyToTargets": 9007199254741198,
  "description": "SpacetimeDB WebSocket v2 client for GameMaker (Rust / extgen)",
  "exportToGame": true,
  "extensionVersion": "1.3.0",
  "files": [
    {
      "$GMExtensionFile": "v1",
      "%Name": "",
      "constants": [],
      "copyToTargets": 9007199254741198,
      "filename": "SpacetimeDB.ext",
      "final": "",
      "functions": [
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_ping",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {String}",
          "externalName": "__EXT_NATIVE__stdb_ping",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_ping",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_create_client",
          "argCount": 0,
          "args": [],
          "documentation": "@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_create_client",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_create_client",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_destroy_client",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_destroy_client",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_destroy_client",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_connect_simple",
          "argCount": 4,
          "args": [
            2,
            1,
            1,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} uri\r\n@param {String} db_name_or_address\r\n@param {String} auth_token_or_null\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_connect_simple",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_connect_simple",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_disconnect",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_disconnect",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_disconnect",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_reconnect_with_token",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} new_token\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_reconnect_with_token",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_reconnect_with_token",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_poll_event",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_poll_event",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_poll_event",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_poll_events_batch",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_poll_events_batch",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_poll_events_batch",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_get_last_error",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} handle\r\n@returns {String}",
          "externalName": "__EXT_NATIVE__stdb_get_last_error",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_get_last_error",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 1
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_debug_state",
          "argCount": 3,
          "args": [
            2,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_debug_state",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_debug_state",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_set_auto_reconnect",
          "argCount": 5,
          "args": [
            2,
            2,
            2,
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} enabled\r\n@param {Real} max_attempts\r\n@param {Real} base_delay_ms\r\n@param {Real} max_delay_ms\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_set_auto_reconnect",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_set_auto_reconnect",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_set_compression_mode",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} mode\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_set_compression_mode",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_set_compression_mode",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_set_log_level",
          "argCount": 1,
          "args": [
            2
          ],
          "documentation": "@param {Real} level\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_set_log_level",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_set_log_level",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_set_default_request_timeout_ms",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} timeout_ms\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_set_default_request_timeout_ms",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_set_default_request_timeout_ms",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_set_meta_events",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} enabled\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_set_meta_events",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_set_meta_events",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_cancel_request",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} request_id\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_cancel_request",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_cancel_request",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_register_schema",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_register_schema",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_register_schema",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_register_schemas",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_register_schemas",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_register_schemas",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_register_struct_schema",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_register_struct_schema",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_register_struct_schema",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_register_reducer_error_schema",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_register_reducer_error_schema",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_register_reducer_error_schema",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_subscribe_sql",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} sql\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_subscribe_sql",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_subscribe_sql",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_subscribe_all",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_subscribe_all",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_subscribe_all",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_unsubscribe_sql",
          "argCount": 2,
          "args": [
            2,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {Real} query_set_id\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_unsubscribe_sql",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_unsubscribe_sql",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_call_reducer",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_call_reducer",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_call_reducer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_call_procedure",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_call_procedure",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_call_procedure",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "stdb_table_count",
          "argCount": 2,
          "args": [
            2,
            1
          ],
          "documentation": "@param {Real} handle\r\n@param {String} table_name\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_table_count",
          "help": "",
          "hidden": false,
          "kind": 4,
          "name": "stdb_table_count",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_table_iter",
          "argCount": 4,
          "args": [
            2,
            1,
            1,
            2
          ],
          "documentation": "@param {Real} handle\r\n@param {String} table_name\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_table_iter",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_table_iter",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__stdb_table_find",
          "argCount": 4,
          "args": [
            1,
            2,
            1,
            2
          ],
          "documentation": "@param {Pointer} _arg_buffer\r\n@param {Real} _arg_buffer_length\r\n@param {Pointer} _ret_buffer\r\n@param {Real} _ret_buffer_length\r\n@returns {Real}",
          "externalName": "__EXT_NATIVE__stdb_table_find",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__stdb_table_find",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        },
        {
          "$GMExtensionFunction": "",
          "%Name": "__SpacetimeDB_queue_buffer",
          "argCount": 2,
          "args": [
            1,
            2
          ],
          "documentation": "@param {Pointer} _buffer_ptr\r\n@param {Real} _buffer_size",
          "externalName": "__EXT_NATIVE__SpacetimeDB_queue_buffer",
          "help": "",
          "hidden": true,
          "kind": 4,
          "name": "__SpacetimeDB_queue_buffer",
          "resourceType": "GMExtensionFunction",
          "resourceVersion": "2.0",
          "returnType": 2
        }
      ],
      "init": "",
      "kind": 4,
      "name": "",
      "origname": "",
      "ProxyFiles": [
        {
          "$GMProxyFile": "",
          "%Name": "SpacetimeDB.dll",
          "name": "SpacetimeDB.dll",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 6
        },
        {
          "$GMProxyFile": "",
          "%Name": "libSpacetimeDB.dylib",
          "name": "libSpacetimeDB.dylib",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 1
        },
        {
          "$GMProxyFile": "",
          "%Name": "libSpacetimeDB.so",
          "name": "libSpacetimeDB.so",
          "resourceType": "GMProxyFile",
          "resourceVersion": "2.0",
          "TargetMask": 7
        }
      ],
      "resourceType": "GMExtensionFile",
      "resourceVersion": "2.0",
      "uncompress": false,
      "usesRunnerInterface": false
    }
  ],
  "gradleinject": null,
  "hasConvertedCodeInjection": true,
  "helpfile": "",
  "HTML5CodeInjection": "",
  "html5Props": false,
  "IncludedResources": [],
  "installdir": "",
  "iosCocoaPodDependencies": "",
  "iosCocoaPods": "",
  "ioscodeinjection": "\r\n\u003CYYIosBuildSettingsInjection\u003E\r\nLD_RUNPATH_SEARCH_PATHS = \u0022$(inherited) @executable_path/Frameworks\u0022;\r\n\u003C/YYIosBuildSettingsInjection\u003E\r\n",
  "iosdelegatename": "",
  "iosplistinject": null,
  "iosProps": true,
  "iosSystemFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Security.framework",
      "embed": 0,
      "name": "Security.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    },
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Foundation.framework",
      "embed": 0,
      "name": "Foundation.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    },
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Network.framework",
      "embed": 0,
      "name": "Network.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ],
  "iosThirdPartyFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "SpacetimeDB_Rust.xcframework",
      "embed": 1,
      "name": "SpacetimeDB_Rust.xcframework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ],
  "license": "",
  "maccompilerflags": "",
  "maclinkerflags": "-ObjC",
  "macsourcedir": "",
  "name": "SpacetimeDB",
  "options": [],
  "optionsFile": "options.json",
  "packageId": "",
  "parent": {
    "name": "SpacetimeDB",
    "path": "folders/Extensions/SpacetimeDB.yy"
  },
  "productId": "",
  "resourceType": "GMExtension",
  "resourceVersion": "2.0",
  "sourcedir": "",
  "supportedTargets": -1,
  "tvosclassname": "SpacetimeDB",
  "tvosCocoaPodDependencies": "",
  "tvosCocoaPods": "",
  "tvoscodeinjection": "",
  "tvosdelegatename": null,
  "tvosmaccompilerflags": "",
  "tvosmaclinkerflags": "-ObjC",
  "tvosplistinject": null,
  "tvosProps": true,
  "tvosSystemFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Security.framework",
      "embed": 0,
      "name": "Security.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    },
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Foundation.framework",
      "embed": 0,
      "name": "Foundation.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    },
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "Network.framework",
      "embed": 0,
      "name": "Network.framework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ],
  "tvosThirdPartyFrameworkEntries": [
    {
      "$GMExtensionFrameworkEntry": "",
      "%Name": "SpacetimeDB_Rust.xcframework",
      "embed": 1,
      "name": "SpacetimeDB_Rust.xcframework",
      "resourceType": "GMExtensionFrameworkEntry",
      "resourceVersion": "2.0",
      "weakReference": false
    }
  ]
}