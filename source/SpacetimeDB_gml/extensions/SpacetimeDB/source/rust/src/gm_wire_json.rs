//! Convert between `serde_json::Value` and gm_ext_wire streams / GMValueOwned.

use gm_ext_wire::{ArrayStream, GMValueOwned, StructStream};
use serde_json::{Map, Number, Value};
use std::collections::HashMap;

pub fn value_to_struct_stream(v: &Value) -> StructStream {
    match v {
        Value::Object(map) => {
            let mut s = StructStream::new();
            for (k, val) in map {
                add_value(&mut s, k, val);
            }
            s
        }
        other => {
            let mut s = StructStream::new();
            add_value(&mut s, "value", other);
            s
        }
    }
}

pub fn values_to_array_stream(values: impl IntoIterator<Item = Value>) -> ArrayStream {
    let mut arr = ArrayStream::new();
    for v in values {
        push_value(&mut arr, &v);
    }
    arr
}

pub fn push_value(arr: &mut ArrayStream, v: &Value) {
    match v {
        Value::Null => arr.push_undefined(),
        Value::Bool(b) => arr.push_bool(*b),
        Value::Number(n) => arr.push_f64(json_number_as_f64(n)),
        Value::String(s) => arr.push_string(s),
        Value::Array(items) => {
            let mut inner = ArrayStream::new();
            for item in items {
                push_value(&mut inner, item);
            }
            arr.push_array(&inner);
        }
        Value::Object(map) => {
            let mut s = StructStream::new();
            for (k, val) in map {
                add_value(&mut s, k, val);
            }
            arr.push_struct(&s);
        }
    }
}

pub fn add_value(s: &mut StructStream, key: &str, v: &Value) {
    match v {
        Value::Null => s.add_undefined(key),
        Value::Bool(b) => s.add_bool(key, *b),
        Value::Number(n) => s.add_f64(key, json_number_as_f64(n)),
        Value::String(str) => s.add_string(key, str),
        Value::Array(items) => {
            let mut inner = ArrayStream::new();
            for item in items {
                push_value(&mut inner, item);
            }
            s.add_array(key, &inner);
        }
        Value::Object(map) => {
            let mut nested = StructStream::new();
            for (k, val) in map {
                add_value(&mut nested, k, val);
            }
            s.add_struct(key, &nested);
        }
    }
}

fn json_number_as_f64(n: &Number) -> f64 {
    n.as_f64()
        .or_else(|| n.as_i64().map(|i| i as f64))
        .or_else(|| n.as_u64().map(|u| u as f64))
        .unwrap_or(0.0)
}

pub fn gmvalue_to_json(v: &GMValueOwned) -> Value {
    match v {
        GMValueOwned::Undefined => Value::Null,
        GMValueOwned::Bool(b) => Value::Bool(*b),
        GMValueOwned::U8(n) => Value::Number((*n).into()),
        GMValueOwned::I8(n) => Value::Number((*n).into()),
        GMValueOwned::U16(n) => Value::Number((*n).into()),
        GMValueOwned::I16(n) => Value::Number((*n).into()),
        GMValueOwned::U32(n) => Value::Number((*n).into()),
        GMValueOwned::I32(n) => Value::Number((*n).into()),
        GMValueOwned::U64(n) => Value::Number(Number::from(*n)),
        GMValueOwned::F32(n) => json_f64(*n as f64),
        GMValueOwned::F64(n) => json_f64(*n),
        GMValueOwned::String(s) => Value::String(s.clone()),
        GMValueOwned::Pointer(p) => Value::Number(Number::from(*p)),
        GMValueOwned::Buffer { length, address } => Value::Object({
            let mut m = Map::new();
            m.insert("length".into(), Value::Number(Number::from(*length)));
            m.insert("address".into(), Value::Number(Number::from(*address)));
            m
        }),
        GMValueOwned::Array(items) => {
            Value::Array(items.iter().map(gmvalue_to_json).collect())
        }
        GMValueOwned::Struct(map) => Value::Object(
            map.iter()
                .map(|(k, val)| (k.clone(), gmvalue_to_json(val)))
                .collect(),
        ),
    }
}

pub fn gm_struct_to_json(map: &HashMap<String, GMValueOwned>) -> Value {
    Value::Object(
        map.iter()
            .map(|(k, v)| (k.clone(), gmvalue_to_json(v)))
            .collect(),
    )
}

pub fn gm_array_to_json(items: &[GMValueOwned]) -> Value {
    Value::Array(items.iter().map(gmvalue_to_json).collect())
}

fn json_f64(n: f64) -> Value {
    Number::from_f64(n)
        .map(Value::Number)
        .unwrap_or(Value::Null)
}
