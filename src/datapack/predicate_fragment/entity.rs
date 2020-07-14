use super::{
    DistancePredicateFragment, ItemPredicateFragment, LocationPredicateFragment,
    PlayerPredicateFragment, RangeOrNumber,
};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityPredicateFragment {
    pub entity_type: Option<NamespacedId>,
    pub nbt: Option<String>,

    pub team: Option<String>,
    pub flags: Option<EntityPredicateFlags>,

    pub distance: Option<DistancePredicateFragment<f32>>,
    pub effects: HashMap<NamespacedId, EntityPredicateEffect>,
    pub equipment: Option<EntityPredicateEquipment>,
    pub location: Option<LocationPredicateFragment>,

    #[serde(rename = "type")]
    pub vehicle: Option<Box<EntityPredicateFragment>>,
    pub player: Option<PlayerPredicateFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityPredicateEffect {
    pub amplifier: RangeOrNumber<i32>,
    pub duration: RangeOrNumber<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityPredicateEquipment {
    pub mainhand: Option<ItemPredicateFragment>,
    pub offhand: Option<ItemPredicateFragment>,
    pub head: Option<ItemPredicateFragment>,
    pub chest: Option<ItemPredicateFragment>,
    pub legs: Option<ItemPredicateFragment>,
    pub feet: Option<ItemPredicateFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityPredicateFlags {
    pub is_on_fire: Option<bool>,
    pub is_sneaking: Option<bool>,
    pub is_sprinting: Option<bool>,
    pub is_swimming: Option<bool>,
    pub is_baby: Option<bool>,
}
