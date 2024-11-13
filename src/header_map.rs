use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

use crate::errors::{bail_on_value_type, missing_field_at_pointer};

pub type HeaderMap = IndexMap<String, FieldMapper>;

#[derive(Deserialize)]
#[serde(tag = "filter_kind")]
pub enum FieldMapper {
    #[serde(rename = "truthy_keys")]
    TruthyKeys(TruthyKeys),
    #[serde(untagged)]
    PathPointer(String),
}

#[derive(Deserialize)]
pub struct TruthyKeys {
    root_pointer: String,
    #[serde(default)]
    include_fields: Vec<String>,
    #[serde(default)]
    exclude_fields: Vec<String>,
}

impl FieldMapper {
    pub fn map_record(mapper: &FieldMapper, record: &Value) -> serde_json::Result<Value> {
        match mapper {
            FieldMapper::PathPointer(pointer) => record
                .pointer(pointer)
                .ok_or(missing_field_at_pointer(pointer))
                .cloned(),
            FieldMapper::TruthyKeys(transform) => {
                transform_truthy_keys(transform, record).map(Value::from)
            }
        }
    }
}

fn transform_truthy_keys(
    transform: &TruthyKeys,
    record: &Value,
) -> serde_json::Result<Vec<String>> {
    let TruthyKeys {
        root_pointer,
        include_fields,
        exclude_fields,
    } = transform;

    if !record.is_object() {
        bail_on_value_type!(&record, expected = "a JSON object");
    };

    let field_object = record
        .pointer(root_pointer)
        .ok_or_else(|| missing_field_at_pointer(root_pointer))
        .and_then(|value| {
            if let Some(object) = value.as_object() {
                Ok(object)
            } else {
                bail_on_value_type!(value, expected = "a JSON object");
            }
        })?;

    let mut truthy_keys = Vec::new();

    for (key, value) in field_object.into_iter() {
        if !include_fields.is_empty() && !include_fields.contains(key) {
            continue;
        }
        if !exclude_fields.is_empty() && exclude_fields.contains(key) {
            continue;
        }
        if is_truthy(value) {
            dbg!(key);
            truthy_keys.push(key.clone());
        }
    }

    Ok(truthy_keys)
}

/// Returns `true` if "information is present" (the value is `true`, non-null, or not empty).
/// Numerical values will always return `true`, so you can't rely on
/// this function to return `false` if the value *seems* equivalent to `0`,
/// since the type is not known for comparison purposes.
fn is_truthy(value: &serde_json::Value) -> bool {
    use serde_json::Value;
    match value {
        Value::Null | Value::Bool(false) => false,
        Value::String(inner) if inner.is_empty() => false,
        Value::Array(vec) if vec.is_empty() => false,
        Value::Object(map) if map.is_empty() => false,
        _ => true,
    }
}
