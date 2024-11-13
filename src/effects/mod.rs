mod filters;
mod numbers;
mod objects;

use serde::Deserialize;

use serde_json::{Result, Value};

use self::filters::{FilterKeysEffect, TruthyFilterEffect};
use self::numbers::{CeilEffect, FloorEffect, RoundEffect};
use self::objects::ObjectKeysEffect;

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
        #[serde(rename = "filter_keys")]
        FilterKeys(FilterKeysEffect),
        #[serde(rename = "truthy_filter")]
        TruthyFilter(TruthyFilterEffect),
        #[serde(rename = "object_keys")]
        ObjectKeys(ObjectKeysEffect),
        #[serde(rename = "round")]
        Round(RoundEffect),
        #[serde(rename = "floor")]
        Floor(FloorEffect),
        #[serde(rename = "ceil")]
        Ceil(CeilEffect),
    }
}
