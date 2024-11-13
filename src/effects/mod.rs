mod truthy_keys;

use serde::Deserialize;

use serde_json::{Result, Value};

use self::truthy_keys::TruthyKeys;

pub trait Effect {
    fn apply(&self, value: &Value) -> Result<Value>;
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
            Self::TruthyKeys(effect) => effect.apply(value),
        }
    }
}
