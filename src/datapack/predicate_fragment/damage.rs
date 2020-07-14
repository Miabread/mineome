use super::{EntityPredicateFragment, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamagePredicateFragment {
    pub blocked: Option<bool>,

    pub dealt: Option<RangeOrNumber<f64>>,
    pub taken: Option<RangeOrNumber<f64>>,

    pub source_entity: Option<EntityPredicateFragment>,
    #[serde(rename = "type")]
    pub damage_type: Option<DamageTypePredicateFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamageTypePredicateFragment {
    pub bypasses_armor: Option<bool>,
    pub bypasses_invulnerability: Option<bool>,
    pub bypasses_magic: Option<bool>,

    pub is_explosion: Option<bool>,
    pub is_fire: Option<bool>,
    pub is_magic: Option<bool>,
    pub is_projectile: Option<bool>,
    pub is_lightning: Option<bool>,

    pub direct_entity: Option<EntityPredicateFragment>,
    pub source_entity: Option<EntityPredicateFragment>,
}
