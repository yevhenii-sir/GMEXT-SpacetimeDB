//! Runtime BSATN в†’ JSON decoding using GameMaker-registered schemas.
//!
//! This module provides pure functions for decoding BSATN-encoded data into
//! JSON values, using runtime-registered table and struct schemas. This is
//! necessary because GameMaker cannot use compile-time types like the
//! standard SpacetimeDB Rust SDK.

use serde_json::{json, Value};
use std::collections::HashMap;
use std::fmt::Write;

/// Parse a hex string into a byte vector.
pub fn parse_hex(hex_str: &str) -> Option<Vec<u8>> {
    if !hex_str.len().is_multiple_of(2) {
        return None;
    }
    (0..hex_str.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex_str[i..i + 2], 16))
        .collect::<Result<Vec<u8>, _>>()
        .ok()
}

/// A simple cursor-based reader for BSATN-encoded data.
pub struct BsatnReader<'a> {
    pub data: &'a [u8],
    pub pos: usize,
}

impl<'a> BsatnReader<'a> {
    pub fn read_bytes(&mut self, len: usize) -> Option<&'a [u8]> {
        if self.pos + len > self.data.len() {
            return None;
        }
        let slice = &self.data[self.pos..self.pos + len];
        self.pos += len;
        Some(slice)
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.read_bytes(1).map(|b| b[0])
    }
    pub fn read_bool(&mut self) -> Option<bool> {
        self.read_u8().map(|b| b != 0)
    }
    pub fn read_u16(&mut self) -> Option<u16> {
        self.read_bytes(2)
            .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_i16(&mut self) -> Option<i16> {
        self.read_bytes(2)
            .map(|b| i16::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_u32(&mut self) -> Option<u32> {
        self.read_bytes(4)
            .map(|b| u32::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_i32(&mut self) -> Option<i32> {
        self.read_bytes(4)
            .map(|b| i32::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_f32(&mut self) -> Option<f32> {
        self.read_u32().map(f32::from_bits)
    }
    pub fn read_u64(&mut self) -> Option<u64> {
        self.read_bytes(8)
            .map(|b| u64::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_i64(&mut self) -> Option<i64> {
        self.read_bytes(8)
            .map(|b| i64::from_le_bytes(b.try_into().unwrap()))
    }
    pub fn read_f64(&mut self) -> Option<f64> {
        self.read_u64().map(f64::from_bits)
    }

    pub fn read_string(&mut self) -> Option<String> {
        let len = self.read_u32()? as usize;
        let b = self.read_bytes(len)?;
        String::from_utf8(b.to_vec()).ok()
    }
}

/// Decode a single BSATN field using a type name and struct schemas.
///
/// Supports primitive types, identity/address (with hex reversal),
/// `Option<T>`, `Vec<T>`/`Array<T>`/`List<T>`, and struct types
/// looked up from the runtime schema registry.
pub fn decode_field(
    reader: &mut BsatnReader,
    type_name: &str,
    struct_schemas: &HashMap<String, Value>,
) -> Option<Value> {
    let original_trimmed = type_name.trim();
    let t = original_trimmed.to_lowercase();

    match t.as_str() {
        "bool" => reader.read_bool().map(Value::Bool),
        "u8" => reader.read_u8().map(|v| Value::Number(v.into())),
        "i8" => reader
            .read_bytes(1)
            .map(|b| Value::Number((b[0] as i8).into())),
        "u16" => reader.read_u16().map(|v| Value::Number(v.into())),
        "i16" => reader.read_i16().map(|v| Value::Number(v.into())),
        "u32" => reader.read_u32().map(|v| Value::Number(v.into())),
        "i32" => reader.read_i32().map(|v| Value::Number(v.into())),
        "u64" => reader.read_u64().map(|v| Value::Number(v.into())),
        "i64" => reader.read_i64().map(|v| Value::Number(v.into())),
        "f32" => reader.read_f32().map(|v| {
            serde_json::Number::from_f64(v as f64)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }),
        "f64" => reader.read_f64().map(|v| {
            serde_json::Number::from_f64(v)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }),
        "string" => reader.read_string().map(Value::String),
        "identity" => reader.read_bytes(32).map(|b| {
            let mut rev = b.to_vec();
            rev.reverse();
            let mut s = String::with_capacity(66);
            s.push_str("0x");
            for byte in rev {
                let _ = write!(&mut s, "{:02x}", byte);
            }
            Value::String(s)
        }),
        "address" => reader.read_bytes(16).map(|b| {
            let mut rev = b.to_vec();
            rev.reverse();
            let mut s = String::with_capacity(34);
            s.push_str("0x");
            for byte in rev {
                let _ = write!(&mut s, "{:02x}", byte);
            }
            Value::String(s)
        }),
        _ if t.starts_with("option<") && t.ends_with(">") => {
            let inner_type = original_trimmed["option<".len()..original_trimmed.len() - 1].trim();
            let tag = reader.read_u8()?;
            if tag == 0 {
                Some(Value::Null)
            } else if tag == 1 {
                decode_field(reader, inner_type, struct_schemas)
            } else {
                None
            }
        }
        _ if (t.starts_with("array<") || t.starts_with("vec<") || t.starts_with("list<"))
            && t.ends_with(">") =>
        {
            let inner_type = if t.starts_with("array<") {
                original_trimmed["array<".len()..original_trimmed.len() - 1].trim()
            } else if t.starts_with("list<") {
                original_trimmed["list<".len()..original_trimmed.len() - 1].trim()
            } else {
                original_trimmed["vec<".len()..original_trimmed.len() - 1].trim()
            };

            let len = reader.read_u32()? as usize;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                arr.push(decode_field(reader, inner_type, struct_schemas)?);
            }
            Some(Value::Array(arr))
        }
        _ => {
            let schema_opt = struct_schemas
                .get(original_trimmed)
                .or_else(|| struct_schemas.iter().find(|(k, _)| k.to_lowercase() == t).map(|(_, v)| v));

            if let Some(schema_val) = schema_opt {
                let fields = schema_val.as_array().unwrap();
                let mut obj = serde_json::Map::new();
                for field in fields {
                    let f_name = field.get("name").unwrap().as_str().unwrap();
                    let f_type = field.get("type").unwrap().as_str().unwrap();
                    let f_val = decode_field(reader, f_type, struct_schemas)?;
                    obj.insert(f_name.to_string(), f_val);
                }
                return Some(Value::Object(obj));
            }
            None
        }
    }
}

/// Decode BSATN-encoded rows using a hex string and a pre-parsed schema Value.
///
/// The `schema` should be a JSON array of field descriptors,
/// each with "name" and "type" keys. Alternatively, it can be a JSON
/// string containing such an array, or an object with "fields"/"columns"/"elements".
pub fn decode_bsatn_rows(
    hex_str: &str,
    schema: &Value,
    struct_schemas: &HashMap<String, Value>,
) -> Result<Vec<Value>, String> {
    let bytes = parse_hex(hex_str).ok_or_else(|| "parse_hex failed".to_string())?;
    let mut schema = schema.clone();
    if let Some(inner_str) = schema.as_str() {
        if let Ok(parsed_inner) = serde_json::from_str(inner_str) {
            schema = parsed_inner;
        }
    }

    let fields = schema
        .as_array()
        .or_else(|| schema.get("fields").and_then(|v| v.as_array()))
        .or_else(|| schema.get("columns").and_then(|v| v.as_array()))
        .or_else(|| schema.get("elements").and_then(|v| v.as_array()))
        .ok_or_else(|| "schema is not an array!".to_string())?;

    let mut reader = BsatnReader {
        data: &bytes,
        pos: 0,
    };
    let mut rows = Vec::new();

    while reader.pos < reader.data.len() {
        let mut row_obj = serde_json::Map::new();
        for field in fields {
            let name = field
                .get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");
            let type_name = field
                .get("type")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            let start_pos = reader.pos;
            let val = decode_field(&mut reader, type_name, struct_schemas).ok_or_else(|| {
                format!(
                    "field '{}' of type '{}' failed at byte {}/{}",
                    name,
                    type_name,
                    start_pos,
                    reader.data.len()
                )
            })?;
            row_obj.insert(name.to_string(), val);
        }
        rows.push(Value::Object(row_obj));
    }
    Ok(rows)
}

/// Heuristically decode a BSATN-encoded reducer error payload.
///
/// SpacetimeDB reducer errors are BSATN-encoded values of the reducer's error type.
/// Common patterns:
/// - String: u32 LE length + UTF-8 bytes  (e.g. "InvalidOperation_NoValue")
/// - Enum:   u8 tag + variant payload     (e.g. tag=0 for first variant)
/// - Unit:   empty bytes                  (unit struct / unit enum variant with no payload)
///
/// If a `reducer_error_schema` is registered for this reducer, it will be used
/// for structured decoding. Otherwise, heuristic decoding is attempted.
pub fn try_decode_bsatn_error(
    hex_str: &str,
    reducer_name: &Option<String>,
    struct_schemas: &HashMap<String, Value>,
    reducer_error_schemas: &HashMap<String, String>,
) -> Value {
    let bytes = match parse_hex(hex_str) {
        Some(b) => b,
        None => return json!({"_raw_hex": hex_str, "_decode_error": "parse_hex failed"}),
    };

    if bytes.is_empty() {
        // Empty bytes = unit type (e.g. unit enum variant or unit struct)
        return json!({"_tag": "Unit"});
    }

    // 1. Try schema-based decoding if a reducer error schema is registered
    if let Some(rname) = reducer_name {
        if let Some(schema_str) = reducer_error_schemas.get(rname) {
            let schema_val: Value = match serde_json::from_str(schema_str) {
                Ok(v) => v,
                Err(_) => {
                    return json!({"_raw_hex": hex_str, "_decode_error": "invalid error schema JSON"});
                }
            };

            // If the schema says the type is "string", decode as BSATN string
            if let Some(type_name) = schema_val.as_str() {
                let t = type_name.trim().to_lowercase();
                if t == "string" {
                    let mut reader = BsatnReader {
                        data: &bytes,
                        pos: 0,
                    };
                    if let Some(s) = reader.read_string() {
                        if reader.pos == bytes.len() {
                            return Value::String(s);
                        }
                    }
                }
            }

            // If the schema is an array of variant definitions (enum schema)
            if let Some(variants) = schema_val.as_array() {
                let tag = bytes[0] as usize;
                if tag < variants.len() {
                    let variant = &variants[tag];
                    let v_name = variant
                        .get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unknown");
                    let v_type = variant
                        .get("type")
                        .and_then(|v| v.as_str())
                        .unwrap_or("Unit");

                    let mut reader = BsatnReader {
                        data: &bytes,
                        pos: 1,
                    }; // skip tag byte

                    if v_type.trim().to_lowercase() == "unit" {
                        return json!({ v_name: null });
                    }

                    if let Some(decoded) = decode_field(&mut reader, v_type, struct_schemas) {
                        return json!({ v_name: decoded });
                    } else {
                        return json!({ v_name: format!("<decode_failed for type {}>", v_type) });
                    }
                }
                // Tag out of range, fall through to heuristic
            }

            // If the schema is a single object with "type" field describing a struct
            if let Some(type_name) = schema_val.get("type").and_then(|v| v.as_str()) {
                let mut reader = BsatnReader {
                    data: &bytes,
                    pos: 0,
                };
                if let Some(decoded) = decode_field(&mut reader, type_name, struct_schemas) {
                    return decoded;
                }
            }
        }
    }

    // 2. Heuristic: try BSATN String (u32 LE length prefix + valid UTF-8)
    if bytes.len() >= 4 {
        let len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;
        if len + 4 == bytes.len() {
            if let Ok(s) = String::from_utf8(bytes[4..].to_vec()) {
                return Value::String(s);
            }
        }
        // Also allow len + 4 <= bytes.len() (trailing data tolerated)
        if len > 0 && len + 4 <= bytes.len() {
            if let Ok(s) = String::from_utf8(bytes[4..4 + len].to_vec()) {
                return Value::String(s);
            }
        }
    }

    // 3. Heuristic: try BSATN enum (tag byte + payload)
    if bytes.len() > 1 && bytes[0] < 16 {
        let tag = bytes[0];
        let rest = &bytes[1..];
        if rest.len() >= 4 {
            let len = u32::from_le_bytes([rest[0], rest[1], rest[2], rest[3]]) as usize;
            if len + 4 == rest.len() {
                if let Ok(s) = String::from_utf8(rest[4..].to_vec()) {
                    return json!({ format!("Variant{}", tag): s });
                }
            }
        }
        if rest.is_empty() {
            return json!({ format!("Variant{}", tag): null });
        }
    }

    // 4. Fallback: return raw hex with a hint
    let truncated = if hex_str.len() > 256 {
        &hex_str[..256]
    } else {
        hex_str
    };
    json!({
        "_raw_hex": truncated,
        "_decode_hint": "unknown BSATN error type; register a reducer error schema for structured decoding"
    })
}

