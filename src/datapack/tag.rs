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
    target_type: TagType,
    replace: bool,
    values: Vec<NamespacedId>,
}
