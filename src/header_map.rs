use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::{Result, Value};

use crate::errors::{bail_on_value_type, missing_field_at_pointer};

pub trait Effect {
    fn apply(&self, value: &Value) -> Result<Value>;
}

pub type HeaderMap = IndexMap<String, FieldMapper>;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum FieldMapper {
    PathPointer(String),
    WithEffects {
        pointer: String,
        #[serde(default)]
        effects: Vec<MapEffect>,
    },
}

impl FieldMapper {
    pub(crate) fn pointer(&self) -> &str {
        match self {
            Self::PathPointer(pointer) | Self::WithEffects { pointer, .. } => pointer,
        }
    }

    pub(crate) fn effects(&self) -> Option<&Vec<MapEffect>> {
        match self {
            Self::PathPointer(_) => None,
            Self::WithEffects { effects, .. } => Some(effects),
        }
    }

    pub(crate) fn resolve(&self, record: &Value) -> Result<Value> {
        let pointer = self.pointer();
        let value = record
            .pointer(pointer)
            .ok_or_else(|| missing_field_at_pointer(pointer))?
            .clone();
        if let Some(effects) = self.effects() {
            effects
                .iter()
                .try_fold(value, |value, effect| effect.apply(&value))
        } else {
            Ok(value)
        }
    }
}

#[derive(Deserialize)]
#[serde(tag = "kind")]
pub enum MapEffect {
    #[serde(rename = "truthy_keys")]
    TruthyKeys(TruthyKeys),
}

impl Effect for MapEffect {
    fn apply(&self, value: &Value) -> Result<Value> {
        match self {
            MapEffect::TruthyKeys(effect) => effect.apply(value),
        }
    }
}

#[derive(Deserialize)]
pub struct TruthyKeys {
    #[serde(default)]
    include_fields: Vec<String>,
    #[serde(default)]
    exclude_fields: Vec<String>,
}

impl TruthyKeys {
    fn collect(params: &TruthyKeys, value: &Value) -> Result<Vec<String>> {
        let TruthyKeys {
            include_fields,
            exclude_fields,
        } = params;

        let Some(object) = value.as_object() else {
            bail_on_value_type!(&value, expected = "a JSON object");
        };

        let mut truthy_keys = Vec::new();

        for (key, value) in object.into_iter() {
            if !include_fields.is_empty() && !include_fields.contains(key) {
                continue;
            }
            if !exclude_fields.is_empty() && exclude_fields.contains(key) {
                continue;
            }
            if is_truthy(value) {
                truthy_keys.push(key.clone());
            }
        }

        Ok(truthy_keys)
    }
}

impl Effect for TruthyKeys {
    fn apply(&self, value: &Value) -> Result<Value> {
        TruthyKeys::collect(self, value).map(Value::from)
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
