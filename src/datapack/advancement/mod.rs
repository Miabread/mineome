pub mod criteria;

use crate::internal_prelude::*;
use criteria::AdvancementPredicate;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Advancement {
    pub parent: Option<String>,
    pub display: Option<AdvancementDisplay>,

    pub criteria: HashMap<String, AdvancementPredicate>,
    pub requirements: Vec<Vec<String>>,
    pub rewards: Option<AdvancementRewards>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementDisplay {
    pub title: String,
    pub description: String,

    pub icon: AdvancementIcon,
    pub frame: AdvancementFrame,
    pub background: Option<String>,

    pub show_toast: bool,
    pub announce_to_chat: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementIcon {
    pub item: NamespacedId,
    pub nbt: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvancementFrame {
    Task,
    Goal,
    Challenge,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementRewards {
    pub recipes: Vec<NamespacedId>,
    pub loot: Vec<NamespacedId>,
    pub experience: i32,
    pub function: NamespacedId,
}
