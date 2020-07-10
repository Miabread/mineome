use super::{
    ItemCriteriaFragment, LocationCriteriaFragment, PlayerCriteriaFragment, Range, RangeOrNumber,
};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityCriteriaFragment {
    pub entity_type: Option<NamespacedId>,
    pub nbt: Option<String>,

    pub team: Option<String>,
    pub flags: Option<EntityCriteriaFlags>,

    pub distance: Option<EntityCriteriaDistance>,
    pub effects: HashMap<NamespacedId, EntityCriteriaEffect>,
    pub equipment: Option<EntityCriteriaEquipment>,
    pub location: Option<LocationCriteriaFragment>,

    #[serde(rename = "type")]
    pub vehicle: Option<Box<EntityCriteriaFragment>>,
    pub player: Option<PlayerCriteriaFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityCriteriaDistance {
    pub absolute: Option<Range<f32>>,
    pub horizontal: Option<Range<f32>>,
    pub x: Option<Range<f32>>,
    pub y: Option<Range<f32>>,
    pub z: Option<Range<f32>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityCriteriaEffect {
    pub amplifier: RangeOrNumber<i32>,
    pub duration: RangeOrNumber<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityCriteriaEquipment {
    pub mainhand: Option<ItemCriteriaFragment>,
    pub offhand: Option<ItemCriteriaFragment>,
    pub head: Option<ItemCriteriaFragment>,
    pub chest: Option<ItemCriteriaFragment>,
    pub legs: Option<ItemCriteriaFragment>,
    pub feet: Option<ItemCriteriaFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct EntityCriteriaFlags {
    pub is_on_fire: Option<bool>,
    pub is_sneaking: Option<bool>,
    pub is_sprinting: Option<bool>,
    pub is_swimming: Option<bool>,
    pub is_baby: Option<bool>,
}
