use super::{predicate::Predicate, predicate_fragment::RangeOrNumber};
use crate::internal_prelude::*;

pub mod loot_function;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootTable {
    #[serde(rename = "type")]
    loot_table_type: LootTableType,
    pools: Vec<LootTablePool>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootTableType {
    Empty,
    Entity,
    Block,
    Chest,
    Fishing,
    Gift,
    AdvancementReward,
    Barter,
    Command,
    Selector,
    AdvancementEntity,
    Generic,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootTablePool {
    conditions: Vec<Predicate>,
    functions: (),
    rolls: RangeOrNumber<i32>,
    bonus_rolls: Option<RangeOrNumber<f32>>,
    entries: Vec<LootTableEntry>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootTableEntry {
    conditions: Vec<Predicate>,
    entry_type: LootTableEntryType,
    name: Option<NamespacedId>,
    children: Vec<LootTableEntry>,
    expand: Option<bool>,
    functions: (),
    weight: i32,
    quality: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootTableEntryType {
    Item,
    Tag,
    LootTable,
    Group,
    Alternatives,
    Sequence,
    Dynamic,
    Empty,
}
