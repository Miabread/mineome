pub mod damage;
pub mod entity;
pub mod item;
pub mod location;
pub mod player;
pub mod position;

use crate::internal_prelude::*;

pub use damage::{DamageCriteriaFragment, DamageTypeCriteriaFragment};
pub use entity::EntityCriteriaFragment;
pub use item::ItemCriteriaFragment;
pub use location::LocationCriteriaFragment;
pub use player::PlayerCriteriaFragment;
pub use position::{DistanceCriteriaFragment, PositionCriteriaFragment};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum RangeOrNumber<T> {
    Number(T),
    Range(Range<T>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Range<T> {
    pub min: T,
    pub max: T,
}
