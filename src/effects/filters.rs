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
