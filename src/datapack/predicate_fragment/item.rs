use super::RangeOrNumber;
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemPredicateFragment {
    pub item: Option<NamespacedId>,
    pub tag: Option<NamespacedId>,
    pub nbt: Option<String>,

    pub potion: Option<String>,
    pub count: Option<RangeOrNumber<i32>>,
    pub durability: Option<RangeOrNumber<i32>>,

    pub enchantments: Vec<ItemEnchantmentPredicate>,
    pub stored_enchantments: Vec<ItemEnchantmentPredicate>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ItemEnchantmentPredicate {
    pub enchantment: NamespacedId,
    pub levels: RangeOrNumber<i32>,
}
