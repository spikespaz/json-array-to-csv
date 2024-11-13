use serde::Deserialize;
use serde_json::{Result, Value};

use crate::errors::bail_on_value_type;

use super::Effect;

macro_rules! impl_round_effect {
    ($EffectIdent:ident, $round_op:path) => {
        #[derive(Deserialize)]
        pub struct $EffectIdent {
            decimals: u32,
        }

        impl Effect for $EffectIdent {
            fn apply(&self, value: &Value) -> Result<Value> {
                if let Some(float) = value.as_f64() {
                    let shift = f64::powf(10.0, self.decimals as f64);
                    let number = $round_op(float * shift) / shift;
                    if self.decimals == 0 {
                        Ok(Value::from(number as u64))
                    } else {
                        Ok(Value::from(number))
                    }
                } else {
                    bail_on_value_type!(value, expected = "a number");
                }
            }
        }
    };
}

impl_round_effect!(RoundEffect, f64::round);
impl_round_effect!(FloorEffect, f64::floor);
impl_round_effect!(CeilEffect, f64::ceil);
