use super::{
    super::{predicate::Predicate, predicate_fragment::*},
    LootTableEntry,
};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootFunction {
    #[serde(flatten)]
    function_type: LootFunctionType,
    conditions: Vec<Predicate>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "function")]
pub enum LootFunctionType {
    ApplyBonus {
        enchantment: NamespacedId,
        formula: LootFunctionApplyBonusFormula,
        parameters: LootFunctionApplyBonusParameters,
    },
    CopyName {
        source: String,
    },
    CopyNbt {
        source: LootFunctionCopyNbtSource,
        ops: Vec<LootFunctionCopyNbtOps>,
    },
    CopyState {
        block: NamespacedId,
        properties: Vec<String>,
    },
    EnchantRandomly {
        enchantments: Vec<NamespacedId>,
    },
    EnchantWithLevels {
        treasure: bool,
        levels: RangeOrNumber<i32>,
    },
    ExplorationMap {
        destination: String,
        decoration: Option<String>,
        zoom: Option<i32>,
        search_radius: Option<i32>,
        skip_existing_chunks: Option<bool>,
    },
    ExplosionDecay,
    FurnaceSmelt,
    FillPlayerHead {
        entity: EntityPredicateSubject,
    },
    LimitCount {
        limit: RangeOrNumber<i32>,
    },
    LootingEnchant {
        count: RangeOrNumber<i32>,
        limit: i32,
    },
    SetAttributes {
        modifier: Vec<LootFunctionSetAttributesModifier>,
    },
    SetContents {
        entries: Vec<LootTableEntry>,
    },
    SetCount(LootFunctionSetCount),
    SetDamage {
        damage: RangeOrNumber<f32>,
    },
    SetLootTable {
        name: NamespacedId,
        seed: Option<i32>,
    },
    SetLore {
        lore: String,
        entity: EntityPredicateSubject,
        replace: Option<bool>,
    },
    SetName {
        name: String,
        entity: EntityPredicateSubject,
    },
    SetNbt {
        tag: String,
    },
    SetStewEffect {
        effects: Vec<LootFunctionSetStewEffect>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootFunctionApplyBonusFormula {
    BinomialWithBonusCount,
    UniformBonusCount,
    OreDrops,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootFunctionApplyBonusParameters {
    extra: Option<i32>,
    probability: Option<f32>,
    #[serde(rename = "bonusMultiplier")]
    bonus_multiplier: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootFunctionCopyNbtSource {
    BlockEntity,
    This,
    Killer,
    KillerPlayer,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootFunctionCopyNbtOps {
    source: String,
    target: String,
    op: LootFunctionCopyNbtOp,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootFunctionCopyNbtOp {
    Replace,
    Append,
    Merge,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootFunctionSetAttributesModifier {
    name: String,
    attribute: String,
    operation: LootFunctionSetAttributesOperation,
    amount: RangeOrNumber<f32>,
    id: Option<String>,
    slot: Option<LootFunctionSetAttributesSlot>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootFunctionSetAttributesOperation {
    Addition,
    MultiplyBase,
    MultiplyTotal,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LootFunctionSetAttributesSlot {
    Mainhand,
    Offhand,
    Feet,
    Legs,
    Chest,
    Head,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum LootFunctionSetCount {
    Integer(i32),
    Distribution(LootFunctionSetCountType),
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum LootFunctionSetCountType {
    Uniform(Range<i32>),
    Binomial { n: i32, p: f32 },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct LootFunctionSetStewEffect {
    #[serde(rename = "type")]
    effect_type: NamespacedId,
    duration: i32,
}
