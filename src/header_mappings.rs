use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::{Result, Value};

use crate::effects::{Effect as _, MapEffect};
use crate::errors::missing_field_at_pointer;

pub type HeaderMappings = IndexMap<String, FieldMapper>;

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
