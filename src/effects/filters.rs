use serde::Deserialize;
use serde_json::{Map, Result, Value};

use crate::bail_on_value_type;

use super::Effect;

#[derive(Deserialize)]
pub struct FilterKeysEffect {
    #[serde(default)]
    include: Vec<String>,
    #[serde(default)]
    exclude: Vec<String>,
}

impl Effect for FilterKeysEffect {
    fn apply(&self, value: &Value) -> Result<Value> {
        let Some(object) = value.as_object() else {
            bail_on_value_type!(value, expected = "a JSON object");
        };

        Ok(object
            .into_iter()
            .filter(|&(key, _)| {
                (!self.include.is_empty() && self.include.contains(key))
                    || (!self.exclude.is_empty() && !self.exclude.contains(key))
            })
            .map(|(key, value)| (key.clone(), value.clone()))
            .collect::<Map<_, _>>()
            .into())
    }
}

#[derive(Deserialize)]
pub struct TruthyFilterEffect {
    #[serde(default)]
    invert: bool,
}

impl Effect for TruthyFilterEffect {
    fn apply(&self, value: &Value) -> Result<Value> {
        if let Some(object) = value.as_object() {
            Ok(object
                .into_iter()
                .filter(|&(_, value)| is_truthy(value) ^ self.invert)
                .map(|(key, value)| (key.clone(), value.clone()))
                .collect::<Map<_, _>>()
                .into())
        } else if let Some(array) = value.as_array() {
            Ok(Value::from_iter(
                array
                    .iter()
                    .filter(|&value| is_truthy(value) ^ self.invert)
                    .cloned(),
            ))
        } else {
            bail_on_value_type!(value, expected = "a JSON object or array");
        }
    }
}

/// Returns `true` if "information is present" (the value is `true`, non-null, or not empty).
/// Numerical values will always return `true`, so you can't rely on
/// this function to return `false` if the value *seems* equivalent to `0`,
/// since the type is not known for comparison purposes.
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null | Value::Bool(false) => false,
        Value::String(inner) if inner.is_empty() => false,
        Value::Array(vec) if vec.is_empty() => false,
        Value::Object(map) if map.is_empty() => false,
        _ => true,
    }
}
