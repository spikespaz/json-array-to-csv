mod truthy_keys;

use serde::Deserialize;

use serde_json::{Result, Value};

use self::truthy_keys::TruthyKeys;

pub trait Effect {
    fn apply(&self, value: &Value) -> Result<Value>;
}

macro_rules! impl_map_effect_enum {
    (
        $( #[ $enum_meta:meta ] )*
        $enum_vis:vis enum $EnumIdent:ident {
            $(
                $( #[ $variant_meta:meta ] )*
                $VariantIdent:ident ( $VariantType:ty ) ,
            )+
        }
    ) => {
        $(#[$enum_meta])*
        $enum_vis enum $EnumIdent {
            $(
                $(#[$variant_meta])*
                $VariantIdent($VariantType),
            )+
        }

        impl Effect for $EnumIdent {
            fn apply(&self, value: &Value) -> serde_json::Result<Value> {
                match self {
                    $(
                        $EnumIdent::$VariantIdent(effect) => effect.apply(value),
                    )+
                }
            }
        }
    }
}

impl_map_effect_enum! {
    #[derive(Deserialize)]
    #[serde(tag = "kind")]
    pub enum MapEffect {
        #[serde(rename = "truthy_keys")]
        TruthyKeys(TruthyKeys),
    }
}
