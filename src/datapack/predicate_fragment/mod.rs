pub mod damage;
pub mod entity;
pub mod item;
pub mod location;
pub mod player;
pub mod position;

use crate::internal_prelude::*;

pub use damage::{DamagePredicateFragment, DamageTypePredicateFragment};
pub use entity::EntityPredicateFragment;
pub use item::ItemPredicateFragment;
pub use location::LocationPredicateFragment;
pub use player::PlayerPredicateFragment;
pub use position::{DistancePredicateFragment, PositionPredicateFragment};

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
