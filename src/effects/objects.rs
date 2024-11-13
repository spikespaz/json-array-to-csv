use serde::Deserialize;
use serde_json::{Result, Value};

use crate::errors::bail_on_value_type;

use super::Effect;

#[derive(Deserialize)]
pub struct ObjectKeysEffect;

impl Effect for ObjectKeysEffect {
    fn apply(&self, value: &Value) -> Result<Value> {
        let Some(object) = value.as_object() else {
            bail_on_value_type!(&value, expected = "a JSON object");
        };

        Ok(Value::from_iter(object.keys().cloned()))
    }
}
