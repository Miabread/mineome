use super::{PositionPredicateFragment, Range, RangeOrNumber};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationPredicateFragment {
    pub block: Option<LocationPredicateBlock>,
    pub fluid: Option<LocationPredicateFluid>,

    pub biome: Option<String>,
    pub dimension: Option<String>,
    pub feature: Option<NamespacedId>,

    pub light: Option<RangeOrNumber<i32>>,
    pub position: Option<PositionPredicateFragment<f64>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationPredicateBlock {
    pub block: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub nbt: Option<String>,
    pub state: HashMap<String, LocationPredicateBlockState>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LocationPredicateBlockState {
    Boolean(bool),
    Integer(i32),
    String(String),
    Range(Range<i32>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LocationPredicateFluid {
    pub fluid: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub state: HashMap<String, LocationPredicateBlockState>,
}
