use super::super::{
    predicate::Predicate,
    predicate_fragment::{entity::EntityPredicateEffect, location::LocationPredicateBlockState, *},
};
use crate::internal_prelude::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "trigger", content = "conditions")]
pub enum AdvancementPredicate {
    #[serde(rename = "minecraft:impossible")]
    Impossible,
    #[serde(rename = "minecraft:bee_nest_destroyed")]
    BeeNestDestroyed {
        block: Option<NamespacedId>,
        item: Option<ItemPredicateFragment>,
        num_bees_inside: Option<i32>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:bred_animals")]
    BredAnimals {
        child: Option<EntityPredicateFragment>,
        parent: Option<EntityPredicateFragment>,
        partner: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:brewed_potion")]
    BrewedPotion {
        potion: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:changed_dimension")]
    ChangedDimension {
        from: Option<NamespacedId>,
        to: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:channeled_lightning")]
    ChanneledLightning {
        victims: Vec<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:construct_beacon")]
    ConstructBeacon {
        level: Option<RangeOrNumber<i32>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:consume_item")]
    ConsumeItem {
        item: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:cured_zombie_villager")]
    CuredZombieVillager {
        villager: Option<EntityPredicateFragment>,
        zombie: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:effects_changed")]
    EffectsChanged {
        effects: HashMap<NamespacedId, EntityPredicateEffect>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:enchanted_item")]
    EnchantedItem {
        item: Option<ItemPredicateFragment>,
        levels: Option<RangeOrNumber<i32>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:enter_block")]
    EnterBlock {
        block: Option<NamespacedId>,
        state: HashMap<String, Vec<LocationPredicateBlockState>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:entity_hurt_player")]
    EntityHurtPlayer {
        damage: Option<DamagePredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:entity_killed_player")]
    EntityKilledPlayer {
        entity: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:filled_bucket")]
    FilledBucket {
        item: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:fishing_rod_hooked")]
    FishingRodHooked {
        entity: Option<EntityPredicateFragment>,
        item: Option<ItemPredicateFragment>,
        rod: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:hero_of_the_village")]
    HeroOfTheVillage {
        location: Option<LocationPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:inventory_changed")]
    InventoryChanged {
        items: Vec<ItemPredicateFragment>,
        slots: Option<InventoryChangedPredicateSlots>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:item_durability_changed")]
    ItemDurabilityChanged {
        delta: Option<RangeOrNumber<i32>>,
        durability: Option<RangeOrNumber<i32>>,
        item: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:item_used_on_block")]
    ItemUsedOnBlock {
        location: Option<LocationPredicateFragment>,
        item: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:killed_by_crossbow")]
    KilledByCrossbow {
        unique_entity_types: Option<RangeOrNumber<i32>>,
        victims: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:levitation")]
    Levitation {
        distance: Option<DistancePredicateFragment<i32>>,
        duration: Option<RangeOrNumber<i32>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:location")]
    Location {
        location: Option<LocationPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:nether_travel")]
    NetherTravel {
        distance: Option<DistancePredicateFragment<f32>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:placed_block")]
    PlacedBlock {
        block: Option<NamespacedId>,
        item: Option<ItemPredicateFragment>,
        location: Option<LocationPredicateFragment>,
        state: HashMap<String, Vec<LocationPredicateBlockState>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:player_generates_container_loot")]
    PlayerGeneratesContainerLoot {
        loot_table: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:player_hurt_entity")]
    PlayerHurtEntity {
        damage: Option<DamagePredicateFragment>,
        entity: Option<Box<EntityPredicateFragment>>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:player_interacted_with_entity")]
    PlayerInteractedWithEntity {
        item: Option<ItemPredicateFragment>,
        entity: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:player_killed_entity")]
    PlayerKilledEntity {
        entity: Option<EntityPredicateFragment>,
        killing_blow: Option<DamageTypePredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:recipe_unlocked")]
    RecipeUnlocked {
        recipe: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:shot_crossbow")]
    ShotCrossbow {
        item: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:slept_in_bed")]
    SleptInBed {
        location: Option<LocationPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:slide_down_block")]
    SlideDownBlock {
        block: Option<NamespacedId>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:summoned_entity")]
    SummonedEntity {
        entity: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:tame_animal")]
    TameAnimal {
        entity: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:target_hit")]
    TargetHit {
        signal_strength: Option<i32>,
        projectile: Option<NamespacedId>,
        shooter: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:thrown_item_picked_up_by_entity")]
    ThrownItemPickedUpByEntity {
        item: Option<ItemPredicateFragment>,
        entity: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:tick")]
    Tick { player: Option<Predicate> },
    #[serde(rename = "minecraft:used_ender_eye")]
    UsedEnderEye {
        distance: RangeOrNumber<f64>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:used_totem")]
    UsedTotem {
        item: Option<ItemPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:villager_trade")]
    VillagerTrade {
        item: Option<ItemPredicateFragment>,
        villager: Option<EntityPredicateFragment>,
        player: Option<Predicate>,
    },
    #[serde(rename = "minecraft:voluntary_exile")]
    VoluntaryExile {
        location: Option<LocationPredicateFragment>,
        player: Option<Predicate>,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InventoryChangedPredicateSlots {
    pub empty: Option<RangeOrNumber<i32>>,
    pub full: Option<RangeOrNumber<i32>>,
    pub occupied: Option<RangeOrNumber<i32>>,
}
