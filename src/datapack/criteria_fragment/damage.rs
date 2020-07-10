use super::{EntityCriteriaFragment, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamageCriteriaFragment {
    blocked: Option<bool>,

    dealt: Option<RangeOrNumber<f64>>,
    taken: Option<RangeOrNumber<f64>>,

    source_entity: Option<EntityCriteriaFragment>,
    #[serde(rename = "type")]
    damage_type: Option<DamageTypeCriteriaFragment>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct DamageTypeCriteriaFragment {
    bypasses_armor: Option<bool>,
    bypasses_invulnerability: Option<bool>,
    bypasses_magic: Option<bool>,

    is_explosion: Option<bool>,
    is_fire: Option<bool>,
    is_magic: Option<bool>,
    is_projectile: Option<bool>,
    is_lightning: Option<bool>,

    direct_entity: Option<EntityCriteriaFragment>,
    source_entity: Option<EntityCriteriaFragment>,
}
