use super::criteria_fragment::{
    entity::EntityCriteriaEffect, location::LocationCriteriaBlockState, *,
};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Advancement {
    pub parent: Option<String>,
    pub display: Option<AdvancementDisplay>,

    pub criteria: HashMap<String, AdvancementCriteria>,
    pub requirements: Vec<Vec<String>>,
    pub rewards: Option<AdvancementRewards>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementDisplay {
    pub title: String,
    pub description: String,

    pub icon: AdvancementIcon,
    pub frame: AdvancementFrame,
    pub background: Option<String>,

    pub show_toast: bool,
    pub announce_to_chat: bool,
    pub hidden: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementIcon {
    pub item: NamespacedId,
    pub nbt: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AdvancementFrame {
    Task,
    Goal,
    Challenge,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AdvancementRewards {
    pub recipes: Vec<NamespacedId>,
    pub loot: Vec<NamespacedId>,
    pub experience: i32,
    pub function: NamespacedId,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "trigger", content = "conditions")]
pub enum AdvancementCriteria {
    #[serde(rename = "minecraft:impossible")]
    Impossible,
    #[serde(rename = "minecraft:bee_nest_destroyed")]
    BeeNestDestroyed {
        block: Option<NamespacedId>,
        item: Option<ItemCriteriaFragment>,
        num_bees_inside: Option<i32>,
    },
    #[serde(rename = "minecraft:bred_animals")]
    BredAnimals {
        child: Option<EntityCriteriaFragment>,
        parent: Option<EntityCriteriaFragment>,
        partner: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:brewed_potion")]
    BrewedPotion { potion: Option<NamespacedId> },
    #[serde(rename = "minecraft:changed_dimension")]
    ChangedDimension {
        from: Option<NamespacedId>,
        to: Option<NamespacedId>,
    },
    #[serde(rename = "minecraft:channeled_lightning")]
    ChanneledLightning {
        victims: Vec<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:construct_beacon")]
    ConstructBeacon { level: Option<RangeOrNumber<i32>> },
    #[serde(rename = "minecraft:consume_item")]
    ConsumeItem { item: Option<ItemCriteriaFragment> },
    #[serde(rename = "minecraft:cured_zombie_villager")]
    CuredZombieVillager {
        villager: Option<EntityCriteriaFragment>,
        zombie: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:effects_changed")]
    EffectsChanged {
        effects: HashMap<NamespacedId, EntityCriteriaEffect>,
    },
    #[serde(rename = "minecraft:enchanted_item")]
    EnchantedItem {
        item: Option<ItemCriteriaFragment>,
        levels: Option<RangeOrNumber<i32>>,
    },
    #[serde(rename = "minecraft:enter_block")]
    EnterBlock {
        block: Option<NamespacedId>,
        state: HashMap<String, Vec<LocationCriteriaBlockState>>,
    },
    #[serde(rename = "minecraft:entity_hurt_player")]
    EntityHurtPlayer {
        damage: Option<DamageCriteriaFragment>,
    },
    #[serde(rename = "minecraft:entity_killed_player")]
    EntityKilledPlayer {
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:filled_bucket")]
    FilledBucket { item: Option<ItemCriteriaFragment> },
    #[serde(rename = "minecraft:fishing_rod_hooked")]
    FishingRodHooked {
        entity: Option<EntityCriteriaFragment>,
        item: Option<ItemCriteriaFragment>,
        rod: Option<ItemCriteriaFragment>,
    },
    #[serde(rename = "minecraft:hero_of_the_village")]
    HeroOfTheVillage {
        location: Option<LocationCriteriaFragment>,
    },
    #[serde(rename = "minecraft:inventory_changed")]
    InventoryChanged {
        items: Vec<ItemCriteriaFragment>,
        slots: Option<InventoryChangedCriteriaSlots>,
    },
    #[serde(rename = "minecraft:item_durability_changed")]
    ItemDurabilityChanged {
        delta: Option<RangeOrNumber<i32>>,
        durability: Option<RangeOrNumber<i32>>,
        item: Option<ItemCriteriaFragment>,
    },
    #[serde(rename = "minecraft:item_used_on_block")]
    ItemUsedOnBlock {
        location: Option<LocationCriteriaFragment>,
        item: Option<ItemCriteriaFragment>,
    },
    #[serde(rename = "minecraft:killed_by_crossbow")]
    KilledByCrossbow {
        unique_entity_types: Option<RangeOrNumber<i32>>,
        victims: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:levitation")]
    Levitation {
        distance: Option<DistanceCriteriaFragment<i32>>,
        duration: Option<RangeOrNumber<i32>>,
    },
    #[serde(rename = "minecraft:location")]
    Location {
        location: Option<LocationCriteriaFragment>,
    },
    #[serde(rename = "minecraft:nether_travel")]
    NetherTravel {
        distance: Option<DistanceCriteriaFragment<f32>>,
    },
    #[serde(rename = "minecraft:placed_block")]
    PlacedBlock {
        block: Option<NamespacedId>,
        item: Option<ItemCriteriaFragment>,
        location: Option<LocationCriteriaFragment>,
        state: HashMap<String, Vec<LocationCriteriaBlockState>>,
    },
    #[serde(rename = "minecraft:player_generates_container_loot")]
    PlayerGeneratesContainerLoot { loot_table: Option<NamespacedId> },
    #[serde(rename = "minecraft:player_hurt_entity")]
    PlayerHurtEntity {
        damage: Option<DamageCriteriaFragment>,
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:player_interacted_with_entity")]
    PlayerInteractedWithEntity {
        item: Option<ItemCriteriaFragment>,
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:player_killed_entity")]
    PlayerKilledEntity {
        entity: Option<EntityCriteriaFragment>,
        killing_blow: Option<DamageTypeCriteriaFragment>,
    },
    #[serde(rename = "minecraft:recipe_unlocked")]
    RecipeUnlocked { recipe: Option<NamespacedId> },
    #[serde(rename = "minecraft:shot_crossbow")]
    ShotCrossbow { item: Option<NamespacedId> },
    #[serde(rename = "minecraft:slept_in_bed")]
    SleptInBed {
        location: Option<LocationCriteriaFragment>,
    },
    #[serde(rename = "minecraft:slide_down_block")]
    SlideDownBlock { block: Option<NamespacedId> },
    #[serde(rename = "minecraft:summoned_entity")]
    SummonedEntity {
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:tame_animal")]
    TameAnimal {
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:target_hit")]
    TargetHit {
        signal_strength: Option<i32>,
        projectile: Option<NamespacedId>,
        shooter: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:thrown_item_picked_up_by_entity")]
    ThrownItemPickedUpByEntity {
        item: Option<ItemCriteriaFragment>,
        entity: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:tick")]
    Tick,
    #[serde(rename = "minecraft:used_ender_eye")]
    UsedEnderEye { distance: RangeOrNumber<f64> },
    #[serde(rename = "minecraft:used_totem")]
    UsedTotem { item: Option<ItemCriteriaFragment> },
    #[serde(rename = "minecraft:villager_trade")]
    VillagerTrade {
        item: Option<ItemCriteriaFragment>,
        villager: Option<EntityCriteriaFragment>,
    },
    #[serde(rename = "minecraft:voluntary_exile")]
    VoluntaryExile {
        location: Option<LocationCriteriaFragment>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InventoryChangedCriteriaSlots {
    pub empty: Option<RangeOrNumber<i32>>,
    pub full: Option<RangeOrNumber<i32>>,
    pub occupied: Option<RangeOrNumber<i32>>,
}
