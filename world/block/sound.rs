use world_sound::Event;

#[derive(Debug)]
pub struct Type {
    pub volume: f32,
    pub pitch: f32,
    pub break_sound: Event,
    pub step_sound: Event,
    pub place_sound: Event,
    pub hit_sound: Event,
    pub fall_sound: Event,
}

impl Type {
    pub(crate) const WOOD: Self = Self::new(
        Event::WoodBreak,
        Event::WoodStep,
        Event::WoodPlace,
        Event::WoodHit,
        Event::WoodFall,
    );

    pub(crate) const GRAVEL: Self = Self::new(
        Event::GravelBreak,
        Event::GravelStep,
        Event::GravelPlace,
        Event::GravelHit,
        Event::GravelFall,
    );

    pub(crate) const GRASS: Self = Self::new(
        Event::GrassBreak,
        Event::GrassStep,
        Event::GrassPlace,
        Event::GrassHit,
        Event::GrassFall,
    );

    pub(crate) const LILY_PAD: Self = Self::new(
        Event::GrassBreak,
        Event::GrassStep,
        Event::LilyPadPlace,
        Event::GrassHit,
        Event::GrassFall,
    );

    pub(crate) const STONE: Self = Self::new(
        Event::StoneBreak,
        Event::StoneStep,
        Event::StonePlace,
        Event::StoneHit,
        Event::StoneFall,
    );

    pub(crate) const METAL: Self = Self::with_volume_pitch(
        1.0,
        1.5,
        Event::MetalBreak,
        Event::MetalStep,
        Event::MetalPlace,
        Event::MetalHit,
        Event::MetalFall,
    );

    pub(crate) const GLASS: Self = Self::new(
        Event::GlassBreak,
        Event::GlassStep,
        Event::GlassPlace,
        Event::GlassHit,
        Event::GlassFall,
    );

    pub(crate) const WOOL: Self = Self::new(
        Event::WoolBreak,
        Event::WoolStep,
        Event::WoolPlace,
        Event::WoolHit,
        Event::WoolFall,
    );

    pub(crate) const SAND: Self = Self::new(
        Event::SandBreak,
        Event::SandStep,
        Event::SandPlace,
        Event::SandHit,
        Event::SandFall,
    );

    pub(crate) const SNOW: Self = Self::new(
        Event::SnowBreak,
        Event::SnowStep,
        Event::SnowPlace,
        Event::SnowHit,
        Event::SnowFall,
    );

    pub(crate) const POWDER_SNOW: Self = Self::new(
        Event::PowderSnowBreak,
        Event::PowderSnowStep,
        Event::PowderSnowPlace,
        Event::PowderSnowHit,
        Event::PowderSnowFall,
    );

    pub(crate) const LADDER: Self = Self::new(
        Event::LadderBreak,
        Event::LadderStep,
        Event::LadderPlace,
        Event::LadderHit,
        Event::LadderFall,
    );

    pub(crate) const ANVIL: Self = Self::with_volume_pitch(
        0.3,
        1.0,
        Event::AnvilBreak,
        Event::AnvilStep,
        Event::AnvilPlace,
        Event::AnvilHit,
        Event::AnvilFall,
    );

    pub(crate) const SLIME_BLOCK: Self = Self::new(
        Event::SlimeBlockBreak,
        Event::SlimeBlockStep,
        Event::SlimeBlockPlace,
        Event::SlimeBlockHit,
        Event::SlimeBlockFall,
    );

    pub(crate) const HONEY_BLOCK: Self = Self::new(
        Event::HoneyBlockBreak,
        Event::HoneyBlockStep,
        Event::HoneyBlockPlace,
        Event::HoneyBlockHit,
        Event::HoneyBlockFall,
    );

    pub(crate) const WET_GRASS: Self = Self::new(
        Event::WetGrassBreak,
        Event::WetGrassStep,
        Event::WetGrassPlace,
        Event::WetGrassHit,
        Event::WetGrassFall,
    );

    pub(crate) const CORAL_BLOCK: Self = Self::new(
        Event::CoralBlockBreak,
        Event::CoralBlockStep,
        Event::CoralBlockPlace,
        Event::CoralBlockHit,
        Event::CoralBlockFall,
    );

    pub(crate) const BAMBOO: Self = Self::new(
        Event::BambooBreak,
        Event::BambooStep,
        Event::BambooPlace,
        Event::BambooHit,
        Event::BambooFall,
    );

    pub(crate) const BAMBOO_SAPLING: Self = Self::new(
        Event::BambooSaplingBreak,
        Event::BambooStep,
        Event::BambooSaplingPlace,
        Event::BambooSaplingHit,
        Event::BambooFall,
    );

    pub(crate) const SCAFFOLDING: Self = Self::new(
        Event::ScaffoldingBreak,
        Event::ScaffoldingStep,
        Event::ScaffoldingPlace,
        Event::ScaffoldingHit,
        Event::ScaffoldingFall,
    );

    pub(crate) const SWEET_BERRY_BUSH: Self = Self::new(
        Event::SweetBerryBushBreak,
        Event::GrassStep,
        Event::SweetBerryBushPlace,
        Event::GrassHit,
        Event::GrassFall,
    );

    pub(crate) const CROP: Self = Self::new(
        Event::CropBreak,
        Event::GrassStep,
        Event::CropPlanted,
        Event::GrassHit,
        Event::GrassFall,
    );

    pub(crate) const HARD_CROP: Self = Self::new(
        Event::WoodBreak,
        Event::WoodStep,
        Event::CropPlanted,
        Event::WoodHit,
        Event::WoodFall,
    );

    pub(crate) const VINE: Self = Self::new(
        Event::VineBreak,
        Event::VineStep,
        Event::VinePlace,
        Event::VineHit,
        Event::VineFall,
    );

    pub(crate) const NETHER_WART: Self = Self::new(
        Event::NetherWartBreak,
        Event::StoneStep,
        Event::NetherWartPlanted,
        Event::StoneHit,
        Event::StoneFall,
    );

    pub(crate) const LANTERN: Self = Self::new(
        Event::LanternBreak,
        Event::LanternStep,
        Event::LanternPlace,
        Event::LanternHit,
        Event::LanternFall,
    );

    pub(crate) const STEM: Self = Self::new(
        Event::StemBreak,
        Event::StemStep,
        Event::StemPlace,
        Event::StemHit,
        Event::StemFall,
    );

    pub(crate) const NYLIUM: Self = Self::new(
        Event::NyliumBreak,
        Event::NyliumStep,
        Event::NyliumPlace,
        Event::NyliumHit,
        Event::NyliumFall,
    );

    pub(crate) const FUNGUS: Self = Self::new(
        Event::FungusBreak,
        Event::FungusStep,
        Event::FungusPlace,
        Event::FungusHit,
        Event::FungusFall,
    );

    pub(crate) const ROOTS: Self = Self::new(
        Event::RootsBreak,
        Event::RootsStep,
        Event::RootsPlace,
        Event::RootsHit,
        Event::RootsFall,
    );

    pub(crate) const SHROOMLIGHT: Self = Self::new(
        Event::ShroomlightBreak,
        Event::ShroomlightStep,
        Event::ShroomlightPlace,
        Event::ShroomlightHit,
        Event::ShroomlightFall,
    );

    pub(crate) const WEEPING_VINES: Self = Self::new(
        Event::WeepingVinesBreak,
        Event::WeepingVinesStep,
        Event::WeepingVinesPlace,
        Event::WeepingVinesHit,
        Event::WeepingVinesFall,
    );

    pub(crate) const TWISTING_VINES: Self = Self::with_volume_pitch(
        1.0,
        0.5,
        Event::WeepingVinesBreak,
        Event::WeepingVinesStep,
        Event::WeepingVinesPlace,
        Event::WeepingVinesHit,
        Event::WeepingVinesFall,
    );

    pub(crate) const SOUL_SAND: Self = Self::new(
        Event::SoulSandBreak,
        Event::SoulSandStep,
        Event::SoulSandPlace,
        Event::SoulSandHit,
        Event::SoulSandFall,
    );

    pub(crate) const SOUL_SOIL: Self = Self::new(
        Event::SoulSoilBreak,
        Event::SoulSoilStep,
        Event::SoulSoilPlace,
        Event::SoulSoilHit,
        Event::SoulSoilFall,
    );

    pub(crate) const BASALT: Self = Self::new(
        Event::BasaltBreak,
        Event::BasaltStep,
        Event::BasaltPlace,
        Event::BasaltHit,
        Event::BasaltFall,
    );

    pub(crate) const WART_BLOCK: Self = Self::new(
        Event::WartBlockBreak,
        Event::WartBlockStep,
        Event::WartBlockPlace,
        Event::WartBlockHit,
        Event::WartBlockFall,
    );

    pub(crate) const NETHERRACK: Self = Self::new(
        Event::NetherrackBreak,
        Event::NetherrackStep,
        Event::NetherrackPlace,
        Event::NetherrackHit,
        Event::NetherrackFall,
    );

    pub(crate) const NETHER_BRICKS: Self = Self::new(
        Event::NetherBricksBreak,
        Event::NetherBricksStep,
        Event::NetherBricksPlace,
        Event::NetherBricksHit,
        Event::NetherBricksFall,
    );

    pub(crate) const NETHER_SPROUTS: Self = Self::new(
        Event::NetherSproutsBreak,
        Event::NetherSproutsStep,
        Event::NetherSproutsPlace,
        Event::NetherSproutsHit,
        Event::NetherSproutsFall,
    );

    pub(crate) const NETHER_ORE: Self = Self::new(
        Event::NetherOreBreak,
        Event::NetherOreStep,
        Event::NetherOrePlace,
        Event::NetherOreHit,
        Event::NetherOreFall,
    );

    pub(crate) const BONE_BLOCK: Self = Self::new(
        Event::BoneBlockBreak,
        Event::BoneBlockStep,
        Event::BoneBlockPlace,
        Event::BoneBlockHit,
        Event::BoneBlockFall,
    );

    pub(crate) const NETHERITE_BLOCK: Self = Self::new(
        Event::NetheriteBlockBreak,
        Event::NetheriteBlockStep,
        Event::NetheriteBlockPlace,
        Event::NetheriteBlockHit,
        Event::NetheriteBlockFall,
    );

    pub(crate) const ANCIENT_DEBRIS: Self = Self::new(
        Event::AncientDebrisBreak,
        Event::AncientDebrisStep,
        Event::AncientDebrisPlace,
        Event::AncientDebrisHit,
        Event::AncientDebrisFall,
    );

    pub(crate) const LODESTONE: Self = Self::new(
        Event::LodestoneBreak,
        Event::LodestoneStep,
        Event::LodestonePlace,
        Event::LodestoneHit,
        Event::LodestoneFall,
    );

    pub(crate) const CHAIN: Self = Self::new(
        Event::ChainBreak,
        Event::ChainStep,
        Event::ChainPlace,
        Event::ChainHit,
        Event::ChainFall,
    );

    pub(crate) const NETHER_GOLD_ORE: Self = Self::new(
        Event::NetherGoldOreBreak,
        Event::NetherGoldOreStep,
        Event::NetherGoldOrePlace,
        Event::NetherGoldOreHit,
        Event::NetherGoldOreFall,
    );

    pub(crate) const GILDED_BLACKSTONE: Self = Self::new(
        Event::GildedBlackstoneBreak,
        Event::GildedBlackstoneStep,
        Event::GildedBlackstonePlace,
        Event::GildedBlackstoneHit,
        Event::GildedBlackstoneFall,
    );

    pub(crate) const CANDLE: Self = Self::new(
        Event::CandleBreak,
        Event::CandleStep,
        Event::CandlePlace,
        Event::CandleHit,
        Event::CandleFall,
    );

    pub(crate) const AMETHYST: Self = Self::new(
        Event::AmethystBlockBreak,
        Event::AmethystBlockStep,
        Event::AmethystBlockPlace,
        Event::AmethystBlockHit,
        Event::AmethystBlockFall,
    );

    pub(crate) const AMETHYST_CLUSTER: Self = Self::new(
        Event::AmethystClusterBreak,
        Event::AmethystClusterStep,
        Event::AmethystClusterPlace,
        Event::AmethystClusterHit,
        Event::AmethystClusterFall,
    );

    pub(crate) const SMALL_AMETHYST_BUD: Self = Self::new(
        Event::SmallAmethystBudBreak,
        Event::AmethystClusterStep,
        Event::SmallAmethystBudPlace,
        Event::AmethystClusterHit,
        Event::AmethystClusterFall,
    );

    pub(crate) const MEDIUM_AMETHYST_BUD: Self = Self::new(
        Event::MediumAmethystBudBreak,
        Event::AmethystClusterStep,
        Event::MediumAmethystBudPlace,
        Event::AmethystClusterHit,
        Event::AmethystClusterFall,
    );

    pub(crate) const LARGE_AMETHYST_BUD: Self = Self::new(
        Event::LargeAmethystBudBreak,
        Event::AmethystClusterStep,
        Event::LargeAmethystBudPlace,
        Event::AmethystClusterHit,
        Event::AmethystClusterFall,
    );

    pub(crate) const TUFF: Self = Self::new(
        Event::TuffBreak,
        Event::TuffStep,
        Event::TuffPlace,
        Event::TuffHit,
        Event::TuffFall,
    );

    pub(crate) const CALCITE: Self = Self::new(
        Event::CalciteBreak,
        Event::CalciteStep,
        Event::CalcitePlace,
        Event::CalciteHit,
        Event::CalciteFall,
    );

    pub(crate) const DRIPSTONE_BLOCK: Self = Self::new(
        Event::DripstoneBlockBreak,
        Event::DripstoneBlockStep,
        Event::DripstoneBlockPlace,
        Event::DripstoneBlockHit,
        Event::DripstoneBlockFall,
    );

    pub(crate) const POINTED_DRIPSTONE: Self = Self::new(
        Event::PointedDripstoneBreak,
        Event::PointedDripstoneStep,
        Event::PointedDripstonePlace,
        Event::PointedDripstoneHit,
        Event::PointedDripstoneFall,
    );

    pub(crate) const COPPER: Self = Self::new(
        Event::CopperBreak,
        Event::CopperStep,
        Event::CopperPlace,
        Event::CopperHit,
        Event::CopperFall,
    );

    pub(crate) const CAVE_VINES: Self = Self::new(
        Event::CaveVinesBreak,
        Event::CaveVinesStep,
        Event::CaveVinesPlace,
        Event::CaveVinesHit,
        Event::CaveVinesFall,
    );

    pub(crate) const SPORE_BLOSSOM: Self = Self::new(
        Event::SporeBlossomBreak,
        Event::SporeBlossomStep,
        Event::SporeBlossomPlace,
        Event::SporeBlossomHit,
        Event::SporeBlossomFall,
    );

    pub(crate) const AZALEA: Self = Self::new(
        Event::AzaleaBreak,
        Event::AzaleaStep,
        Event::AzaleaPlace,
        Event::AzaleaHit,
        Event::AzaleaFall,
    );

    pub(crate) const FLOWERING_AZALEA: Self = Self::new(
        Event::FloweringAzaleaBreak,
        Event::FloweringAzaleaStep,
        Event::FloweringAzaleaPlace,
        Event::FloweringAzaleaHit,
        Event::FloweringAzaleaFall,
    );

    pub(crate) const MOSS_CARPET: Self = Self::new(
        Event::MossCarpetBreak,
        Event::MossCarpetStep,
        Event::MossCarpetPlace,
        Event::MossCarpetHit,
        Event::MossCarpetFall,
    );

    pub(crate) const MOSS: Self = Self::new(
        Event::MossBreak,
        Event::MossStep,
        Event::MossPlace,
        Event::MossHit,
        Event::MossFall,
    );

    pub(crate) const BIG_DRIPLEAF: Self = Self::new(
        Event::BigDripleafBreak,
        Event::BigDripleafStep,
        Event::BigDripleafPlace,
        Event::BigDripleafHit,
        Event::BigDripleafFall,
    );

    pub(crate) const SMALL_DRIPLEAF: Self = Self::new(
        Event::SmallDripleafBreak,
        Event::SmallDripleafStep,
        Event::SmallDripleafPlace,
        Event::SmallDripleafHit,
        Event::SmallDripleafFall,
    );

    pub(crate) const ROOTED_DIRT: Self = Self::new(
        Event::RootedDirtBreak,
        Event::RootedDirtStep,
        Event::RootedDirtPlace,
        Event::RootedDirtHit,
        Event::RootedDirtFall,
    );

    pub(crate) const HANGING_ROOTS: Self = Self::new(
        Event::HangingRootsBreak,
        Event::HangingRootsStep,
        Event::HangingRootsPlace,
        Event::HangingRootsHit,
        Event::HangingRootsFall,
    );

    pub(crate) const AZALEA_LEAVES: Self = Self::new(
        Event::AzaleaLeavesBreak,
        Event::AzaleaLeavesStep,
        Event::AzaleaLeavesPlace,
        Event::AzaleaLeavesHit,
        Event::AzaleaLeavesFall,
    );

    pub(crate) const SCULK_SENSOR: Self = Self::new(
        Event::SculkSensorBreak,
        Event::SculkSensorStep,
        Event::SculkSensorPlace,
        Event::SculkSensorHit,
        Event::SculkSensorFall,
    );

    pub(crate) const GLOW_LICHEN: Self = Self::new(
        Event::GrassBreak,
        Event::VineStep,
        Event::GrassPlace,
        Event::GrassHit,
        Event::GrassFall,
    );

    pub(crate) const DEEPSLATE: Self = Self::new(
        Event::DeepslateBreak,
        Event::DeepslateStep,
        Event::DeepslatePlace,
        Event::DeepslateHit,
        Event::DeepslateFall,
    );

    pub(crate) const DEEPSLATE_BRICKS: Self = Self::new(
        Event::DeepslateBricksBreak,
        Event::DeepslateBricksStep,
        Event::DeepslateBricksPlace,
        Event::DeepslateBricksHit,
        Event::DeepslateBricksFall,
    );

    pub(crate) const DEEPSLATE_TILES: Self = Self::new(
        Event::DeepslateTilesBreak,
        Event::DeepslateTilesStep,
        Event::DeepslateTilesPlace,
        Event::DeepslateTilesHit,
        Event::DeepslateTilesFall,
    );

    pub(crate) const POLISHED_DEEPSLATE: Self = Self::new(
        Event::PolishedDeepslateBreak,
        Event::PolishedDeepslateStep,
        Event::PolishedDeepslatePlace,
        Event::PolishedDeepslateHit,
        Event::PolishedDeepslateFall,
    );

    const fn new(
        break_sound: Event,
        step_sound: Event,
        place_sound: Event,
        hit_sound: Event,
        fall_sound: Event,
    ) -> Self {
        Type {
            volume: 1.0,
            pitch: 1.0,
            break_sound,
            step_sound,
            place_sound,
            hit_sound,
            fall_sound,
        }
    }

    const fn with_volume_pitch(
        volume: f32,
        pitch: f32,
        break_sound: Event,
        step_sound: Event,
        place_sound: Event,
        hit_sound: Event,
        fall_sound: Event,
    ) -> Self {
        Type {
            volume,
            pitch,
            break_sound,
            step_sound,
            place_sound,
            hit_sound,
            fall_sound,
        }
    }
}
