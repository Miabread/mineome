use super::{EntityCriteriaFragment, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamageCriteriaFragment {
    pub blocked: Option<bool>,

    pub dealt: Option<RangeOrNumber<f64>>,
    pub taken: Option<RangeOrNumber<f64>>,

    pub source_entity: Option<EntityCriteriaFragment>,
    #[serde(rename = "type")]
    pub damage_type: Option<DamageTypeCriteriaFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamageTypeCriteriaFragment {
    pub bypasses_armor: Option<bool>,
    pub bypasses_invulnerability: Option<bool>,
    pub bypasses_magic: Option<bool>,

    pub is_explosion: Option<bool>,
    pub is_fire: Option<bool>,
    pub is_magic: Option<bool>,
    pub is_projectile: Option<bool>,
    pub is_lightning: Option<bool>,

    pub direct_entity: Option<EntityCriteriaFragment>,
    pub source_entity: Option<EntityCriteriaFragment>,
}
