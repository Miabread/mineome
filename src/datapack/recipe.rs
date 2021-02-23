use crate::{internal_prelude::*, misc::namespaced_id::FieldTaggedNamespacedId};
use serde_with::serde_as;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Recipe {
    #[serde(flatten)]
    pub group: Option<String>,
    #[serde(flatten)]
    pub variant: RecipeVariant,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CookingRecipeCommon {
    pub ingredient: Ingredient,
    pub result: NamespacedId,
    pub experience: f64,
    #[serde(rename = "cookingtime")]
    pub cooking_time: Option<i32>,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CraftingShapedRecipe {
    pub pattern: Vec<String>,
    pub key: HashMap<char, Ingredient>,
    pub result: RecipeResultWithCount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CraftingShapelessRecipe {
    pub ingredients: Vec<Ingredient>,
    pub result: RecipeResultWithCount,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(untagged)]
pub enum Ingredient {
    Specific(#[serde_as(as = "FieldTaggedNamespacedId")] NamespacedId),
    Selection(#[serde_as(as = "Vec<FieldTaggedNamespacedId>")] Vec<NamespacedId>),
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RecipeResultWithCount {
    pub count: Option<i32>,
    #[serde_as(as = "FieldTaggedNamespacedId")]
    #[serde(flatten)]
    pub item: NamespacedId,
}

#[serde_as]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SmithingRecipe {
    #[serde_as(as = "FieldTaggedNamespacedId")]
    pub base: NamespacedId,
    #[serde_as(as = "FieldTaggedNamespacedId")]
    pub addition: NamespacedId,
    #[serde_as(as = "FieldTaggedNamespacedId")]
    pub result: NamespacedId,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StonecuttingRecipe {
    pub ingredient: Ingredient,
    pub result: NamespacedId,
    pub count: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RecipeVariant {
    #[serde(rename = "minecraft:blasting")]
    Blasting(CookingRecipeCommon),
    #[serde(rename = "minecraft:campfire_cooking")]
    CampfireCooking(CookingRecipeCommon),
    #[serde(rename = "minecraft:crafting_shaped")]
    CraftingShaped(CraftingShapedRecipe),
    #[serde(rename = "minecraft:crafting_shapeless")]
    CraftingShapeless(CraftingShapelessRecipe),
    #[serde(rename = "minecraft:smelting")]
    Smelting(CookingRecipeCommon),
    #[serde(rename = "minecraft:smithing")]
    Smithing(SmithingRecipe),
    #[serde(rename = "minecraft:smoking")]
    Smoking(CookingRecipeCommon),
    #[serde(rename = "minecraft:stonecutting")]
    Stonecutting(StonecuttingRecipe),
    #[serde(rename = "minecraft:crafting_special_armordye")]
    CraftingSpecialArmorDye,
    #[serde(rename = "minecraft:crafting_special_bannerduplicate")]
    CraftingSpecialBannerDuplicate,
    #[serde(rename = "minecraft:crafting_special_bookcloning")]
    CraftingSpecialBookCloning,
    #[serde(rename = "minecraft:crafting_special_firework_rocket")]
    CraftingSpecialFireworkRocket,
    #[serde(rename = "minecraft:crafting_special_firework_star")]
    CraftingSpecialFireworkStar,
    #[serde(rename = "minecraft:crafting_special_firework_star_fade")]
    CraftingSpecialFireworkStarFade,
    #[serde(rename = "minecraft:crafting_special_mapcloning")]
    CraftingSpecialMapCloning,
    #[serde(rename = "minecraft:crafting_special_mapextending")]
    CraftingSpecialMapExtending,
    #[serde(rename = "minecraft:crafting_special_repairitem")]
    CraftingSpecialRepairItem,
    #[serde(rename = "minecraft:crafting_special_shielddecoration")]
    CraftingSpecialShieldDecoration,
    #[serde(rename = "minecraft:crafting_special_shulkerboxcoloring")]
    CraftingSpecialShulkerBoxColoring,
    #[serde(rename = "minecraft:crafting_special_tippedarrow")]
    CraftingSpecialTippedArrow,
    #[serde(rename = "minecraft:crafting_special_suspiciousstew")]
    CraftingSpecialSuspiciousStew,
}
