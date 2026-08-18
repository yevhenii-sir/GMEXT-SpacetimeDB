function spdb_encode_bsatn_to_buffer(schema, args_struct, buf) {
    for(var i = 0; i < array_length(schema); i++) {
        var f_name = schema[i].name;
        var f_type = schema[i].type;
        var val = args_struct[$ f_name];
        
        spdb_encode_bsatn_value(f_type, val, buf);
    }
}

function spdb_encode_bsatn_value(type_str, val, buf) {
	var type_str_orig = type_str;
    type_str = string_lower(string_replace_all(type_str, " ", ""));

    if (string_pos("option<", type_str) == 1) {
        var inner_type = string_copy(type_str, 8, string_length(type_str) - 8);
        if (is_undefined(val) || val == pointer_null) {
            buffer_write(buf, buffer_u8, 0); // None
            return;
        } else {
            buffer_write(buf, buffer_u8, 1); // Some
            spdb_encode_bsatn_value(inner_type, val, buf);
            return;
        }
    }

    if (string_pos("list<", type_str) == 1 || string_pos("vec<", type_str) == 1 || string_pos("array<", type_str) == 1) {
        var inner_type = "";
        if (string_pos("list<", type_str) == 1) inner_type = string_copy(type_str, 6, string_length(type_str) - 6);
        else if (string_pos("vec<", type_str) == 1) inner_type = string_copy(type_str, 5, string_length(type_str) - 5);
        else inner_type = string_copy(type_str, 7, string_length(type_str) - 7);

        if (!is_array(val)) val = [];
        
        var len = array_length(val);
        buffer_write(buf, buffer_u32, len);
        
        for (var i = 0; i < len; i++) {
            spdb_encode_bsatn_value(inner_type, val[i], buf);
        }
        return;
    }

    if (is_undefined(val)) {
        if (type_str == "string" || type_str == "identity" || type_str == "address") val = "";
        else val = 0;
    }

    switch(type_str) {
        case "bool": buffer_write(buf, buffer_u8, val ? 1 : 0); break;
        case "u8":   buffer_write(buf, buffer_u8, val); break;
        case "i8":   buffer_write(buf, buffer_s8, val); break;
        case "u16":  buffer_write(buf, buffer_u16, val); break;
        case "i16":  buffer_write(buf, buffer_s16, val); break;
        case "u32":  buffer_write(buf, buffer_u32, val); break;
        case "i32":  buffer_write(buf, buffer_s32, val); break;
        case "u64":  buffer_write(buf, buffer_u64, val); break;
        case "i64":  buffer_write(buf, buffer_u64, val); break;
        case "f32":  buffer_write(buf, buffer_f32, val); break;
        case "f64":  buffer_write(buf, buffer_f64, val); break;
        case "string":
            var len = string_byte_length(val);
            buffer_write(buf, buffer_u32, len);
            var start_pos = buffer_tell(buf);
            if (len > 0) buffer_write(buf, buffer_text, val);
            buffer_seek(buf, buffer_seek_start, start_pos + len);
            break;
        case "identity":
        case "address":
            var expected_len = (type_str == "identity") ? 32 : 16;
            var bytes = array_create(expected_len, 0);
            if (val != "") {
                var hex = string_upper(string_replace(val, "0x", ""));
                var hex_chars = "0123456789ABCDEF";
                var b_idx = 0;
                for (var j = 1; j <= string_length(hex); j += 2) {
                    if (b_idx >= expected_len) break;
                    var c1 = max(0, string_pos(string_char_at(hex, j), hex_chars) - 1);
                    var c2 = max(0, string_pos(string_char_at(hex, j + 1), hex_chars) - 1);
                    bytes[b_idx++] = (c1 << 4) | c2;
                }
            }
			
            // BSATN Identity
            for (var j = expected_len - 1; j >= 0; j--) {
                buffer_write(buf, buffer_u8, bytes[j]);
            }
            break;
            
        default:
            var struct_schema = global.spdb_struct_schemas[? type_str_orig];
            if (!is_undefined(struct_schema)) {
                if (!is_struct(val)) val = {};
                
                for(var i = 0; i < array_length(struct_schema); i++) {
                    var f_name = struct_schema[i].name;
                    var f_type = struct_schema[i].type;
                    var f_val = val[$ f_name];
                    spdb_encode_bsatn_value(f_type, f_val, buf);
                }
            } else {
                show_debug_message("BSATN ENCODE ERROR: Unknown type -> " + string(type_str));
            }
            break;
    }
}