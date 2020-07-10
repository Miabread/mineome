use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum TagType {
    Items,
    Blocks,
    Fluids,
    EntityTypes,
    Functions,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Tag {
    pub target_type: TagType,
    pub replace: bool,
    pub values: Vec<NamespacedId>,
}
