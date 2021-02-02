pub mod advancement;
pub mod loot_table;
pub mod predicate;
pub mod predicate_fragment;
pub mod recipe;
pub mod tag;

use crate::internal_prelude::*;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackMeta {
    pub pack_format: usize,
    pub description: String,
}
