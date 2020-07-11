use super::{PositionCriteriaFragment, Range, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationCriteriaFragment {
    pub block: Option<LocationCriteriaBlock>,
    pub fluid: Option<LocationCriteriaFluid>,

    pub biome: Option<String>,
    pub dimension: Option<String>,
    pub feature: Option<NamespacedId>,

    pub light: Option<RangeOrNumber<i32>>,
    pub position: Option<PositionCriteriaFragment<f64>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationCriteriaBlock {
    pub block: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub nbt: Option<String>,
    pub state: HashMap<String, LocationCriteriaBlockState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LocationCriteriaBlockState {
    Boolean(bool),
    Integer(i32),
    String(String),
    Range(Range<i32>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationCriteriaFluid {
    pub fluid: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub state: HashMap<String, LocationCriteriaBlockState>,
}
