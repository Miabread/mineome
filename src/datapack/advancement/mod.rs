mod criteria;

use crate::internal_prelude::*;
use criteria::AdvancementCriteria;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Advancement {
    display: Option<AdvancementDisplay>,
    parent: Option<String>,
    criteria: HashMap<String, AdvancementCriteria>,
    requirements: Vec<Vec<String>>,
    rewards: Option<AdvancementRewards>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementDisplay {
    icon: AdvancementIcon,
    title: String,
    frame: AdvancementFrame,
    background: Option<String>,
    description: String,
    show_toast: bool,
    announce_to_chat: bool,
    hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementIcon {
    item: NamespacedId,
    nbt: String,
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
    recipes: Vec<NamespacedId>,
    loot: Vec<NamespacedId>,
    experience: i32,
    function: NamespacedId,
}
