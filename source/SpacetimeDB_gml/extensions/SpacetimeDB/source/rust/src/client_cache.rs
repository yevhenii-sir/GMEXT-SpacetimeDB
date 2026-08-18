//! Decoded JSON row cache for subscribed SpacetimeDB tables.
//!
//! Wave-1 limits (documented):
//! - No unique secondary indexes.
//! - No refcount for overlapping subscriptions.
//! - Unsubscribe does **not** purge rows; cache is cleared on disconnect/destroy only.
//!
//! Rows are keyed by stringified primary-key values from the registered schema
//! (`primary_key` field, defaulting to `"id"` when present).

use serde_json::Value;
use std::collections::{HashMap, HashSet};

/// Per-table decoded row store.
#[derive(Debug, Default, Clone)]
pub struct TableCache {
    pub pk_field: String,
    pub rows: HashMap<String, Value>,
}

/// Client-wide cache of subscribed table rows.
#[derive(Debug, Default, Clone)]
pub struct ClientCache {
    pub tables: HashMap<String, TableCache>,
}

impl ClientCache {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clear(&mut self) {
        self.tables.clear();
    }

    /// Ensure a table entry exists with the given PK field name.
    pub fn ensure_table(&mut self, table: &str, pk_field: &str) -> &mut TableCache {
        self.tables
            .entry(table.to_string())
            .and_modify(|t| {
                if t.pk_field.is_empty() {
                    t.pk_field = pk_field.to_string();
                }
            })
            .or_insert_with(|| TableCache {
                pk_field: pk_field.to_string(),
                rows: HashMap::new(),
            })
    }

    pub fn count(&self, table: &str) -> usize {
        self.tables.get(table).map(|t| t.rows.len()).unwrap_or(0)
    }

    pub fn iter_values(&self, table: &str) -> Vec<Value> {
        match self.tables.get(table) {
            Some(t) => t.rows.values().cloned().collect(),
            None => Vec::new(),
        }
    }

    pub fn find(&self, table: &str, pk_key: &str) -> Option<Value> {
        self.tables.get(table)?.rows.get(pk_key).cloned()
    }

    /// Upsert decoded rows (subscribe snapshot or inserts).
    pub fn apply_inserts(&mut self, table: &str, pk_field: &str, inserts: &[Value]) {
        let tc = self.ensure_table(table, pk_field);
        let pk = tc.pk_field.clone();
        for row in inserts {
            if let Some(key) = pk_key_from_row(row, &pk) {
                tc.rows.insert(key, row.clone());
            }
        }
    }

    /// Apply deletes, skipping any PK that also appears in `insert_pks` (update pairing).
    pub fn apply_deletes(
        &mut self,
        table: &str,
        pk_field: &str,
        deletes: &[Value],
        insert_pks: &HashSet<String>,
    ) {
        let tc = self.ensure_table(table, pk_field);
        let pk = tc.pk_field.clone();
        for row in deletes {
            if let Some(key) = pk_key_from_row(row, &pk) {
                if insert_pks.contains(&key) {
                    continue;
                }
                tc.rows.remove(&key);
            }
        }
    }

    /// Apply a PersistentTable-style insert/delete batch with update pairing.
    pub fn apply_diff(&mut self, table: &str, pk_field: &str, inserts: &[Value], deletes: &[Value]) {
        let mut insert_pks = HashSet::new();
        for row in inserts {
            if let Some(key) = pk_key_from_row(row, pk_field) {
                insert_pks.insert(key);
            }
        }
        self.apply_inserts(table, pk_field, inserts);
        self.apply_deletes(table, pk_field, deletes, &insert_pks);
    }
}

/// Resolve PK field from a registered schema Value (`{ primary_key?, fields: [...] }`).
pub fn pk_field_from_schema(schema: &Value) -> String {
    if let Some(pk) = schema.get("primary_key").and_then(|v| v.as_str()) {
        if !pk.is_empty() {
            return pk.to_string();
        }
    }
    // Default: "id" if present in fields, else "id" anyway (find may fail without values).
    if let Some(fields) = schema.get("fields").and_then(|v| v.as_array()) {
        for f in fields {
            if f.get("name").and_then(|n| n.as_str()) == Some("id") {
                return "id".to_string();
            }
        }
    }
    "id".to_string()
}

/// Normalize a schema Value so `primary_key` is always set.
pub fn ensure_schema_primary_key(mut schema: Value) -> Value {
    let pk = pk_field_from_schema(&schema);
    if let Some(obj) = schema.as_object_mut() {
        obj.insert("primary_key".to_string(), Value::String(pk));
    }
    schema
}

/// Stringify a row's primary key for map lookup.
pub fn pk_key_from_row(row: &Value, pk_field: &str) -> Option<String> {
    let v = row.get(pk_field)?;
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => None,
        other => Some(other.to_string()),
    }
}

/// Stringify a GM/JSON pk value for lookup (matches `pk_key_from_row` formatting).
pub fn pk_key_from_value(v: &Value) -> Option<String> {
    match v {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => None,
        other => Some(other.to_string()),
    }
}
