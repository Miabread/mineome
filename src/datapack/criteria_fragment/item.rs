use super::RangeOrNumber;
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemCriteriaFragment {
    pub item: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub nbt: Option<String>,

    pub potion: Option<String>,
    pub count: Option<RangeOrNumber<i32>>,
    pub durability: Option<RangeOrNumber<i32>>,

    pub enchantments: Vec<ItemEnchantmentCriteria>,
    pub stored_enchantments: Vec<ItemEnchantmentCriteria>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemEnchantmentCriteria {
    pub enchantment: NamespacedId,
    pub levels: RangeOrNumber<i32>,
}
