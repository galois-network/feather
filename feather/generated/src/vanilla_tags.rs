// This file is @generated. Please do not edit.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VanillaBlockTags {
    AcaciaLogs,
    Anvil,
    BambooPlantableOn,
    Banners,
    BaseStoneNether,
    BaseStoneOverworld,
    BeaconBaseBlocks,
    Beds,
    Beehives,
    BeeGrowables,
    BirchLogs,
    Buttons,
    Campfires,
    Carpets,
    Climbable,
    Corals,
    CoralBlocks,
    CoralPlants,
    CrimsonStems,
    Crops,
    DarkOakLogs,
    Doors,
    DragonImmune,
    EndermanHoldable,
    Fences,
    FenceGates,
    Fire,
    Flowers,
    FlowerPots,
    GoldOres,
    GuardedByPiglins,
    HoglinRepellents,
    Ice,
    Impermeable,
    InfiniburnEnd,
    InfiniburnNether,
    InfiniburnOverworld,
    JungleLogs,
    Leaves,
    Logs,
    LogsThatBurn,
    MushroomGrowBlock,
    NonFlammableWood,
    Nylium,
    OakLogs,
    PiglinRepellents,
    Planks,
    Portals,
    PressurePlates,
    PreventMobSpawningInside,
    Rails,
    Sand,
    Saplings,
    ShulkerBoxes,
    Signs,
    Slabs,
    SmallFlowers,
    SoulFireBaseBlocks,
    SoulSpeedBlocks,
    SpruceLogs,
    Stairs,
    StandingSigns,
    StoneBricks,
    StonePressurePlates,
    StriderWarmBlocks,
    TallFlowers,
    Trapdoors,
    UnderwaterBonemeals,
    UnstableBottomCenter,
    ValidSpawn,
    Walls,
    WallCorals,
    WallPostOverride,
    WallSigns,
    WarpedStems,
    WartBlocks,
    WitherImmune,
    WitherSummonBaseBlocks,
    WoodenButtons,
    WoodenDoors,
    WoodenFences,
    WoodenPressurePlates,
    WoodenSlabs,
    WoodenStairs,
    WoodenTrapdoors,
    Wool,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VanillaEntityTypes {
    Arrows,
    BeehiveInhabitors,
    ImpactProjectiles,
    Raiders,
    Skeletons,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VanillaFluidTags {
    Lava,
    Water,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VanillaItemTags {
    AcaciaLogs,
    Anvil,
    Arrows,
    Banners,
    BeaconPaymentItems,
    Beds,
    BirchLogs,
    Boats,
    Buttons,
    Carpets,
    Coals,
    CreeperDropMusicDiscs,
    CrimsonStems,
    DarkOakLogs,
    Doors,
    Fences,
    Fishes,
    Flowers,
    GoldOres,
    JungleLogs,
    Leaves,
    LecternBooks,
    Logs,
    LogsThatBurn,
    MusicDiscs,
    NonFlammableWood,
    OakLogs,
    PiglinLoved,
    PiglinRepellents,
    Planks,
    Rails,
    Sand,
    Saplings,
    Signs,
    Slabs,
    SmallFlowers,
    SoulFireBaseBlocks,
    SpruceLogs,
    Stairs,
    StoneBricks,
    StoneCraftingMaterials,
    StoneToolMaterials,
    TallFlowers,
    Trapdoors,
    Walls,
    WarpedStems,
    WoodenButtons,
    WoodenDoors,
    WoodenFences,
    WoodenPressurePlates,
    WoodenSlabs,
    WoodenStairs,
    WoodenTrapdoors,
    Wool,
}

impl VanillaBlockTags {
    pub fn name(&self) -> &'static str {
        match self {
            VanillaBlockTags::AcaciaLogs => "acacia_logs",
            VanillaBlockTags::Anvil => "anvil",
            VanillaBlockTags::BambooPlantableOn => "bamboo_plantable_on",
            VanillaBlockTags::Banners => "banners",
            VanillaBlockTags::BaseStoneNether => "base_stone_nether",
            VanillaBlockTags::BaseStoneOverworld => "base_stone_overworld",
            VanillaBlockTags::BeaconBaseBlocks => "beacon_base_blocks",
            VanillaBlockTags::Beds => "beds",
            VanillaBlockTags::Beehives => "beehives",
            VanillaBlockTags::BeeGrowables => "bee_growables",
            VanillaBlockTags::BirchLogs => "birch_logs",
            VanillaBlockTags::Buttons => "buttons",
            VanillaBlockTags::Campfires => "campfires",
            VanillaBlockTags::Carpets => "carpets",
            VanillaBlockTags::Climbable => "climbable",
            VanillaBlockTags::Corals => "corals",
            VanillaBlockTags::CoralBlocks => "coral_blocks",
            VanillaBlockTags::CoralPlants => "coral_plants",
            VanillaBlockTags::CrimsonStems => "crimson_stems",
            VanillaBlockTags::Crops => "crops",
            VanillaBlockTags::DarkOakLogs => "dark_oak_logs",
            VanillaBlockTags::Doors => "doors",
            VanillaBlockTags::DragonImmune => "dragon_immune",
            VanillaBlockTags::EndermanHoldable => "enderman_holdable",
            VanillaBlockTags::Fences => "fences",
            VanillaBlockTags::FenceGates => "fence_gates",
            VanillaBlockTags::Fire => "fire",
            VanillaBlockTags::Flowers => "flowers",
            VanillaBlockTags::FlowerPots => "flower_pots",
            VanillaBlockTags::GoldOres => "gold_ores",
            VanillaBlockTags::GuardedByPiglins => "guarded_by_piglins",
            VanillaBlockTags::HoglinRepellents => "hoglin_repellents",
            VanillaBlockTags::Ice => "ice",
            VanillaBlockTags::Impermeable => "impermeable",
            VanillaBlockTags::InfiniburnEnd => "infiniburn_end",
            VanillaBlockTags::InfiniburnNether => "infiniburn_nether",
            VanillaBlockTags::InfiniburnOverworld => "infiniburn_overworld",
            VanillaBlockTags::JungleLogs => "jungle_logs",
            VanillaBlockTags::Leaves => "leaves",
            VanillaBlockTags::Logs => "logs",
            VanillaBlockTags::LogsThatBurn => "logs_that_burn",
            VanillaBlockTags::MushroomGrowBlock => "mushroom_grow_block",
            VanillaBlockTags::NonFlammableWood => "non_flammable_wood",
            VanillaBlockTags::Nylium => "nylium",
            VanillaBlockTags::OakLogs => "oak_logs",
            VanillaBlockTags::PiglinRepellents => "piglin_repellents",
            VanillaBlockTags::Planks => "planks",
            VanillaBlockTags::Portals => "portals",
            VanillaBlockTags::PressurePlates => "pressure_plates",
            VanillaBlockTags::PreventMobSpawningInside => "prevent_mob_spawning_inside",
            VanillaBlockTags::Rails => "rails",
            VanillaBlockTags::Sand => "sand",
            VanillaBlockTags::Saplings => "saplings",
            VanillaBlockTags::ShulkerBoxes => "shulker_boxes",
            VanillaBlockTags::Signs => "signs",
            VanillaBlockTags::Slabs => "slabs",
            VanillaBlockTags::SmallFlowers => "small_flowers",
            VanillaBlockTags::SoulFireBaseBlocks => "soul_fire_base_blocks",
            VanillaBlockTags::SoulSpeedBlocks => "soul_speed_blocks",
            VanillaBlockTags::SpruceLogs => "spruce_logs",
            VanillaBlockTags::Stairs => "stairs",
            VanillaBlockTags::StandingSigns => "standing_signs",
            VanillaBlockTags::StoneBricks => "stone_bricks",
            VanillaBlockTags::StonePressurePlates => "stone_pressure_plates",
            VanillaBlockTags::StriderWarmBlocks => "strider_warm_blocks",
            VanillaBlockTags::TallFlowers => "tall_flowers",
            VanillaBlockTags::Trapdoors => "trapdoors",
            VanillaBlockTags::UnderwaterBonemeals => "underwater_bonemeals",
            VanillaBlockTags::UnstableBottomCenter => "unstable_bottom_center",
            VanillaBlockTags::ValidSpawn => "valid_spawn",
            VanillaBlockTags::Walls => "walls",
            VanillaBlockTags::WallCorals => "wall_corals",
            VanillaBlockTags::WallPostOverride => "wall_post_override",
            VanillaBlockTags::WallSigns => "wall_signs",
            VanillaBlockTags::WarpedStems => "warped_stems",
            VanillaBlockTags::WartBlocks => "wart_blocks",
            VanillaBlockTags::WitherImmune => "wither_immune",
            VanillaBlockTags::WitherSummonBaseBlocks => "wither_summon_base_blocks",
            VanillaBlockTags::WoodenButtons => "wooden_buttons",
            VanillaBlockTags::WoodenDoors => "wooden_doors",
            VanillaBlockTags::WoodenFences => "wooden_fences",
            VanillaBlockTags::WoodenPressurePlates => "wooden_pressure_plates",
            VanillaBlockTags::WoodenSlabs => "wooden_slabs",
            VanillaBlockTags::WoodenStairs => "wooden_stairs",
            VanillaBlockTags::WoodenTrapdoors => "wooden_trapdoors",
            VanillaBlockTags::Wool => "wool",
        }
    }
}
impl From<VanillaBlockTags> for &'static str {
    fn from(tag: VanillaBlockTags) -> Self {
        tag.name()
    }
}
impl VanillaEntityTypes {
    pub fn name(&self) -> &'static str {
        match self {
            VanillaEntityTypes::Arrows => "arrows",
            VanillaEntityTypes::BeehiveInhabitors => "beehive_inhabitors",
            VanillaEntityTypes::ImpactProjectiles => "impact_projectiles",
            VanillaEntityTypes::Raiders => "raiders",
            VanillaEntityTypes::Skeletons => "skeletons",
        }
    }
}
impl From<VanillaEntityTypes> for &'static str {
    fn from(tag: VanillaEntityTypes) -> Self {
        tag.name()
    }
}
impl VanillaFluidTags {
    pub fn name(&self) -> &'static str {
        match self {
            VanillaFluidTags::Lava => "lava",
            VanillaFluidTags::Water => "water",
        }
    }
}
impl From<VanillaFluidTags> for &'static str {
    fn from(tag: VanillaFluidTags) -> Self {
        tag.name()
    }
}
impl VanillaItemTags {
    pub fn name(&self) -> &'static str {
        match self {
            VanillaItemTags::AcaciaLogs => "acacia_logs",
            VanillaItemTags::Anvil => "anvil",
            VanillaItemTags::Arrows => "arrows",
            VanillaItemTags::Banners => "banners",
            VanillaItemTags::BeaconPaymentItems => "beacon_payment_items",
            VanillaItemTags::Beds => "beds",
            VanillaItemTags::BirchLogs => "birch_logs",
            VanillaItemTags::Boats => "boats",
            VanillaItemTags::Buttons => "buttons",
            VanillaItemTags::Carpets => "carpets",
            VanillaItemTags::Coals => "coals",
            VanillaItemTags::CreeperDropMusicDiscs => "creeper_drop_music_discs",
            VanillaItemTags::CrimsonStems => "crimson_stems",
            VanillaItemTags::DarkOakLogs => "dark_oak_logs",
            VanillaItemTags::Doors => "doors",
            VanillaItemTags::Fences => "fences",
            VanillaItemTags::Fishes => "fishes",
            VanillaItemTags::Flowers => "flowers",
            VanillaItemTags::GoldOres => "gold_ores",
            VanillaItemTags::JungleLogs => "jungle_logs",
            VanillaItemTags::Leaves => "leaves",
            VanillaItemTags::LecternBooks => "lectern_books",
            VanillaItemTags::Logs => "logs",
            VanillaItemTags::LogsThatBurn => "logs_that_burn",
            VanillaItemTags::MusicDiscs => "music_discs",
            VanillaItemTags::NonFlammableWood => "non_flammable_wood",
            VanillaItemTags::OakLogs => "oak_logs",
            VanillaItemTags::PiglinLoved => "piglin_loved",
            VanillaItemTags::PiglinRepellents => "piglin_repellents",
            VanillaItemTags::Planks => "planks",
            VanillaItemTags::Rails => "rails",
            VanillaItemTags::Sand => "sand",
            VanillaItemTags::Saplings => "saplings",
            VanillaItemTags::Signs => "signs",
            VanillaItemTags::Slabs => "slabs",
            VanillaItemTags::SmallFlowers => "small_flowers",
            VanillaItemTags::SoulFireBaseBlocks => "soul_fire_base_blocks",
            VanillaItemTags::SpruceLogs => "spruce_logs",
            VanillaItemTags::Stairs => "stairs",
            VanillaItemTags::StoneBricks => "stone_bricks",
            VanillaItemTags::StoneCraftingMaterials => "stone_crafting_materials",
            VanillaItemTags::StoneToolMaterials => "stone_tool_materials",
            VanillaItemTags::TallFlowers => "tall_flowers",
            VanillaItemTags::Trapdoors => "trapdoors",
            VanillaItemTags::Walls => "walls",
            VanillaItemTags::WarpedStems => "warped_stems",
            VanillaItemTags::WoodenButtons => "wooden_buttons",
            VanillaItemTags::WoodenDoors => "wooden_doors",
            VanillaItemTags::WoodenFences => "wooden_fences",
            VanillaItemTags::WoodenPressurePlates => "wooden_pressure_plates",
            VanillaItemTags::WoodenSlabs => "wooden_slabs",
            VanillaItemTags::WoodenStairs => "wooden_stairs",
            VanillaItemTags::WoodenTrapdoors => "wooden_trapdoors",
            VanillaItemTags::Wool => "wool",
        }
    }
}
impl From<VanillaItemTags> for &'static str {
    fn from(tag: VanillaItemTags) -> Self {
        tag.name()
    }
}