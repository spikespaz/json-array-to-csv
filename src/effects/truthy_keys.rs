use serde::Deserialize;
use serde_json::{Result, Value};

use crate::errors::bail_on_value_type;

use super::Effect;

#[derive(Deserialize)]
pub struct TruthyKeys;

impl Effect for TruthyKeys {
    fn apply(&self, value: &Value) -> Result<Value> {
        let Some(object) = value.as_object() else {
            bail_on_value_type!(&value, expected = "a JSON object");
        };

        let mut truthy_keys = Vec::new();

        for (key, value) in object.into_iter() {
            if is_truthy(value) {
                truthy_keys.push(key.clone());
            }
        }

        Ok(truthy_keys.into())
    }
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
