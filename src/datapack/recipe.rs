use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Recipe {
    group: Option<String>,

    #[serde(flatten)]
    variant: RecipeVariant,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CookingRecipeCommon {
    ingredient: NamespacedId,
    result: NamespacedId,
    experience: f64,
    #[serde(rename = "cookingtime")]
    cooking_time: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CraftingShapedRecipe {
    pattern: (String, String, String),
    ingredients: Vec<NamespacedId>,
    key: HashMap<char, NamespacedId>,
    result: RecipeResultWithCount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CraftingShapelessRecipe {
    ingredients: Vec<NamespacedId>,
    result: RecipeResultWithCount,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct RecipeResultWithCount {
    count: Option<i32>,
    item: NamespacedId,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SmithingRecipe {
    base: NamespacedId,
    addition: NamespacedId,
    result: NamespacedId,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct StonecuttingRecipe {
    ingredient: NamespacedId,
    result: NamespacedId,
    count: i32,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum RecipeVariant {
    #[serde(rename = "blasting")]
    Blasting(CookingRecipeCommon),
    #[serde(rename = "campfire_cooking")]
    CampfireCooking(CookingRecipeCommon),
    #[serde(rename = "crafting_shaped")]
    CraftingShaped(CraftingShapedRecipe),
    #[serde(rename = "crafting_shapeless")]
    CraftingShapeless(CraftingShapelessRecipe),
    #[serde(rename = "smelting")]
    Smelting(CookingRecipeCommon),
    #[serde(rename = "smithing")]
    Smithing(SmithingRecipe),
    #[serde(rename = "smoking")]
    Smoking(CookingRecipeCommon),
    #[serde(rename = "stonecutting")]
    Stonecutting(StonecuttingRecipe),
    #[serde(rename = "crafting_special_armordye")]
    CraftingSpecialArmorDye,
    #[serde(rename = "crafting_special_bannerduplicate")]
    CraftingSpecialBannerDuplicate,
    #[serde(rename = "crafting_special_bookcloning")]
    CraftingSpecialBookCloning,
    #[serde(rename = "crafting_special_firework_rocket")]
    CraftingSpecialFireworkRocket,
    #[serde(rename = "crafting_special_firework_star")]
    CraftingSpecialFireworkStar,
    #[serde(rename = "crafting_special_firework_star_fade")]
    CraftingSpecialFireworkStarFade,
    #[serde(rename = "crafting_special_mapcloning")]
    CraftingSpecialMapCloning,
    #[serde(rename = "crafting_special_mapextending")]
    CraftingSpecialMapExtending,
    #[serde(rename = "crafting_special_repairitem")]
    CraftingSpecialRepairItem,
    #[serde(rename = "crafting_special_shielddecoration")]
    CraftingSpecialShieldDecoration,
    #[serde(rename = "crafting_special_shulkerboxcoloring")]
    CraftingSpecialShulkerBoxColoring,
    #[serde(rename = "crafting_special_tippedarrow")]
    CraftingSpecialTippedArrow,
    #[serde(rename = "crafting_special_suspiciousstew")]
    CraftingSpecialSuspiciousStew,
}
