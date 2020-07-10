use super::RangeOrNumber;
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PlayerCriteriaFragment {
    pub gamemode: Option<String>,
    pub advancements: HashMap<NamespacedId, PlayerCriteriaAdvancement>,

    pub level: Option<RangeOrNumber<i32>>,
    pub recipes: HashMap<NamespacedId, bool>,
    pub stats: Vec<PlayerCriteriaStatistic>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlayerCriteriaAdvancement {
    Boolean(bool),
    Criteria(HashMap<String, bool>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PlayerCriteriaStatistic {
    #[serde(rename = "type")]
    pub base: NamespacedId,
    pub stat: NamespacedId,
    pub value: RangeOrNumber<i32>,
}
