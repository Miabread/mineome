use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "trigger", content = "conditions")]
pub enum AdvancementCriteria {}
