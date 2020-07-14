use super::RangeOrNumber;
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PlayerPredicateFragment {
    pub gamemode: Option<String>,
    pub advancements: HashMap<NamespacedId, PlayerPredicateAdvancement>,

    pub level: Option<RangeOrNumber<i32>>,
    pub recipes: HashMap<NamespacedId, bool>,
    pub stats: Vec<PlayerPredicateStatistic>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum PlayerPredicateAdvancement {
    Boolean(bool),
    Predicate(HashMap<String, bool>),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct PlayerPredicateStatistic {
    #[serde(rename = "type")]
    pub base: NamespacedId,
    pub stat: NamespacedId,
    pub value: RangeOrNumber<i32>,
}
