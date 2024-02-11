use std::{hash, mem, str};

use anyhow::{anyhow, Result};
use enum_map::{Enum, EnumArray};
use quartz_nbt::NbtCompound;
use {world_block_macros as macros, world_resource as resource};

use self::macros::blocks;

mod behavior;
mod material;
mod sound;
pub mod state;

/// Respresents all blocks in the game. The implementation of this type is automatically derived by
/// the `blocks!` proc macro.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct Block(u32);

impl Block {
    /// Creates a new block out of a block ID and with default block state.
    #[must_use]
    pub const fn from_id(id: BlockId) -> Self {
        Block(id.to_numeric() as u32)
    }

    /// Returns the ID of the block.
    #[allow(clippy::cast_possible_truncation)]
    #[must_use]
    pub fn id(self) -> BlockId {
        unsafe { BlockId::from_numeric_unchecked(self.0 as u16) }
    }

    /// Returns the underlying integer representing this block.
    #[must_use]
    pub fn as_u32(self) -> u32 {
        self.0
    }

    pub fn set_property(&mut self, property: state::Property, offset: u32) {
        let bits = property.id().bits();

        debug_assert!(offset + bits <= 32);

        let lower = self.0 & ((1 << offset) - 1);
        let field = property.to_bits();
        let upper = self.0 & u32::MAX.checked_shl(offset + bits).unwrap_or(0);

        debug_assert!(u32::BITS - field.leading_zeros() <= bits);

        self.0 = upper | (field << offset) | lower;
    }
}

impl TryFrom<&NbtCompound> for Block {
    type Error = anyhow::Error;

    fn try_from(tag: &NbtCompound) -> Result<Self, Self::Error> {
        let location = resource::Location::new(tag.get::<_, &str>("Name")?)?;
        let block_id = location.path().parse::<BlockId>()?;
        let mut block = Block::from_id(block_id);
        let state_definition = block_id.state_definition();

        if !state_definition.properties.is_empty() {
            let properties = tag.get::<_, &NbtCompound>("Properties")?;

            for &state::PropertyDefinition { name, id, offset } in state_definition.properties {
                let property_value = properties.get::<_, &str>(name)?;
                let property = state::Property::from_str(id, property_value)?;
                block.set_property(property, offset);
            }
        }

        Ok(block)
    }
}

impl BlockId {
    /// Converts the given numeric ID to a `BlockId`.
    #[must_use]
    pub const fn from_numeric(id: u16) -> Option<Self> {
        // NOTE: We use this instead of a `match` so that the compiler doesn't struggle with
        // exhaustiveness checks so much. This is what a `match` would desugar down to anyway.
        if id <= Self::MAX {
            // SAFETY: We checked that the ID is in bounds of the enum discriminants.
            Some(unsafe { BlockId::from_numeric_unchecked(id) })
        } else {
            None
        }
    }

    /// Converts the given numeric ID to a `BlockId` without checking that the value is within
    /// bounds.
    ///
    /// # Safety
    ///
    /// The given `id` must not exceed [`BlockId::MAX`].
    #[must_use]
    pub const unsafe fn from_numeric_unchecked(id: u16) -> Self {
        debug_assert!(id <= Self::MAX);

        mem::transmute::<u16, BlockId>(id)
    }

    /// Converts the `BlockId` to a numeric ID.
    #[must_use]
    pub const fn to_numeric(self) -> u16 {
        self as u16
    }

    /// Returns an iterator over all block IDs.
    pub fn iter() -> impl Iterator<Item = Self> {
        (0..=Self::MAX).map(|id| {
            // SAFETY: The ID is in bounds of the enum discriminants.
            unsafe { BlockId::from_numeric_unchecked(id) }
        })
    }
}

impl Enum for BlockId {
    const LENGTH: usize = Self::COUNT;

    #[allow(clippy::cast_possible_truncation)]
    fn from_usize(id: usize) -> Self {
        if id < Self::COUNT {
            // SAFETY: We checked that the ID is in bounds of the enum discriminants.
            unsafe { BlockId::from_numeric_unchecked(id as u16) }
        } else {
            unreachable!()
        }
    }

    fn into_usize(self) -> usize {
        usize::from(self.to_numeric())
    }
}

impl<V> EnumArray<V> for BlockId {
    type Array = [V; Self::COUNT];
}

impl PartialEq for BlockId {
    fn eq(&self, other: &Self) -> bool {
        self.to_numeric() == other.to_numeric()
    }
}

impl Eq for BlockId {}

impl hash::Hash for BlockId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.to_numeric().hash(state);
    }
}

impl str::FromStr for BlockId {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        BlockId::from_location(s).ok_or_else(|| anyhow!("`{s}` does not match any block ID"))
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

blocks! {
    // Air is at the very top so that it has a numeric ID of 0. Meaning that for example a chunk
    // initialized with all zeros is valid and does precisely what you expect.
    struct Air {}

    struct AcaciaButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct AcaciaDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct AcaciaFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct AcaciaFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct AcaciaLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct AcaciaLog {
        axis: state::Axis,
    }

    struct AcaciaPlanks {}

    struct AcaciaPressurePlate {
        powered: state::Powered,
    }

    struct AcaciaSapling {
        stage: state::Stage,
    }

    struct AcaciaSign {
        rotation: state::Rotation16,
    }

    struct AcaciaSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct AcaciaStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct AcaciaTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct AcaciaWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct AcaciaWood {
        axis: state::Axis,
    }

    struct ActivatorRail {
        shape: state::RailShape,
        powered: state::Powered,
        waterlogged: state::Waterlogged,
    }

    struct Allium {}

    struct AmethystBlock {}

    struct AmethystCluster {
        facing: state::Facing,
        waterlogged: state::Waterlogged,
    }

    struct AncientDebris {}

    struct Andesite {}

    struct AndesiteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct AndesiteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct AndesiteWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Anvil {
        facing: state::HorizontalFacing,
    }

    struct AttachedMelonStem {
        facing: state::HorizontalFacing,
    }

    struct AttachedPumpkinStem {
        facing: state::HorizontalFacing,
    }

    struct Azalea {}

    struct AzaleaLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct AzureBluet {}

    struct Bamboo {
        age: state::Age1,
        leaves: state::BambooLeaves,
        stage: state::Stage,
    }

    struct BambooSapling {
        stage: state::Stage,
    }

    struct Barrel {
        facing: state::Facing,
        open: state::Open,
    }

    struct Barrier {}

    struct Basalt {
        axis: state::Axis,
    }

    struct Beacon {}

    struct Bedrock {}

    struct BeeNest {
        facing: state::HorizontalFacing,
        honey_level: state::LevelHoney,
    }

    struct Beehive {
        facing: state::HorizontalFacing,
        honey_level: state::LevelHoney,
    }

    struct Beetroots {
        age: state::Age3,
    }

    struct Bell {
        facing: state::HorizontalFacing,
        attachment: state::BellAttachType,
        powered: state::Powered,
    }

    struct BigDripleaf {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
        tilt: state::Tilt,
    }

    struct BigDripleafStem {
        facing: state::Facing,
        waterlogged: state::Waterlogged,
    }

    struct BirchButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct BirchDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct BirchFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct BirchFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct BirchLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct BirchLog {
        axis: state::Axis,
    }

    struct BirchPlanks {}

    struct BirchPressurePlate {
        powered: state::Powered,
    }

    struct BirchSapling {
        stage: state::Stage,
    }

    struct BirchSign {
        rotation: state::Rotation16,
    }

    struct BirchSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct BirchStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct BirchTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct BirchWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct BirchWood {
        axis: state::Axis,
    }

    struct BlackBanner {
        rotation: state::Rotation16,
    }

    struct BlackBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct BlackCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct BlackCandleCake {
        lit: state::Lit,
    }

    struct BlackCarpet {}

    struct BlackConcrete {}

    struct BlackConcretePowder {}

    struct BlackGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct BlackShulkerBox {}

    struct BlackStainedGlass {}

    struct BlackStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct BlackTerracotta {}

    struct BlackWallBanner {
        facing: state::HorizontalFacing,
    }

    struct BlackWool {}

    struct Blackstone {}

    struct BlackstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct BlackstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct BlackstoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct BlastFurnace {
        facing: state::HorizontalFacing,
        lit: state::Lit,
    }

    struct BlueBanner {
        rotation: state::Rotation16,
    }

    struct BlueBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct BlueCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct BlueCandleCake {
        lit: state::Lit,
    }

    struct BlueCarpet {}

    struct BlueConcrete {}

    struct BlueConcretePowder {}

    struct BlueGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct BlueIce {}

    struct BlueOrchid {}

    struct BlueShulkerBox {}

    struct BlueStainedGlass {}

    struct BlueStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct BlueTerracotta {}

    struct BlueWallBanner {
        facing: state::HorizontalFacing,
    }

    struct BlueWool {}

    struct BoneBlock {
        axis: state::Axis,
    }

    struct Bookshelf {}

    struct BrainCoral {
        waterlogged: state::Waterlogged,
    }

    struct BrainCoralBlock {}

    struct BrainCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct BrainCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct BrewingStand {
        has_bottle_0: state::HasBottle0,
        has_bottle_1: state::HasBottle1,
        has_bottle_2: state::HasBottle2,
    }

    struct BrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct BrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct BrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Bricks {}

    struct BrownBanner {
        rotation: state::Rotation16,
    }

    struct BrownBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct BrownCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct BrownCandleCake {
        lit: state::Lit,
    }

    struct BrownCarpet {}

    struct BrownConcrete {}

    struct BrownConcretePowder {}

    struct BrownGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct BrownMushroom {}

    struct BrownMushroomBlock {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
        down: state::Down,
    }

    struct BrownShulkerBox {}

    struct BrownStainedGlass {}

    struct BrownStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct BrownTerracotta {}

    struct BrownWallBanner {
        facing: state::HorizontalFacing,
    }

    struct BrownWool {}

    struct BubbleColumn {
        drag: state::Drag,
    }

    struct BubbleCoral {
        waterlogged: state::Waterlogged,
    }

    struct BubbleCoralBlock {}

    struct BubbleCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct BubbleCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct BuddingAmethyst {}

    struct Cactus {
        age: state::Age15,
    }

    struct Cake {
        bites: state::Bites,
    }

    struct Calcite {}

    struct Campfire {
        facing: state::HorizontalFacing,
        lit: state::Lit,
        signal_fire: state::SignalFire,
        waterlogged: state::Waterlogged,
    }

    struct Candle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct CandleCake {
        lit: state::Lit,
    }

    struct Carrots {
        age: state::Age7,
    }

    struct CartographyTable {}

    struct CarvedPumpkin {
        facing: state::HorizontalFacing,
    }

    struct Cauldron {}

    struct CaveAir {}

    struct CaveVines {
        age: state::Age25,
        berries: state::Berries,
    }

    struct CaveVinesPlant {
        berries: state::Berries,
    }

    struct Chain {
        waterlogged: state::Waterlogged,
        axis: state::Axis,
    }

    struct ChainCommandBlock {
        facing: state::Facing,
        conditional: state::Conditional,
    }

    struct Chest {
        facing: state::HorizontalFacing,
        ty: state::ChestType,
        waterlogged: state::Waterlogged,
    }

    struct ChippedAnvil {
        facing: state::HorizontalFacing,
    }

    struct ChiseledDeepslate {}

    struct ChiseledNetherBricks {}

    struct ChiseledPolishedBlackstone {}

    struct ChiseledQuartzBlock {}

    struct ChiseledRedSandstone {}

    struct ChiseledSandstone {}

    struct ChiseledStoneBricks {}

    struct ChorusFlower {
        age: state::Age5,
    }

    struct ChorusPlant {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
        down: state::Down,
    }

    struct Clay {}

    struct CoalBlock {}

    struct CoalOre {}

    struct CoarseDirt {}

    struct CobbledDeepslate {}

    struct CobbledDeepslateSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CobbledDeepslateStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct CobbledDeepslateWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Cobblestone {}

    struct CobblestoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CobblestoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct CobblestoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Cobweb {}

    struct Cocoa {
        facing: state::HorizontalFacing,
        age: state::Age2,
    }

    struct CommandBlock {
        facing: state::Facing,
        conditional: state::Conditional,
    }

    struct Comparator {
        facing: state::HorizontalFacing,
        mode: state::ComparatorMode,
        powered: state::Powered,
    }

    struct Composter {
        level: state::LevelComposter,
    }

    struct Conduit {
        waterlogged: state::Waterlogged,
    }

    struct CopperBlock {}

    struct CopperOre {}

    struct Cornflower {}

    struct CrackedDeepslateBricks {}

    struct CrackedDeepslateTiles {}

    struct CrackedNetherBricks {}

    struct CrackedPolishedBlackstoneBricks {}

    struct CrackedStoneBricks {}

    struct CraftingTable {}

    struct CreeperHead {
        rotation: state::Rotation16,
    }

    struct CreeperWallHead {
        facing: state::HorizontalFacing,
    }

    struct CrimsonButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct CrimsonDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct CrimsonFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct CrimsonFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct CrimsonFungus {}

    struct CrimsonHyphae {
        axis: state::Axis,
    }

    struct CrimsonNylium {}

    struct CrimsonPlanks {}

    struct CrimsonPressurePlate {
        powered: state::Powered,
    }

    struct CrimsonRoots {}

    struct CrimsonSign {
        rotation: state::Rotation16,
    }

    struct CrimsonSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CrimsonStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct CrimsonStem {
        axis: state::Axis,
    }

    struct CrimsonTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct CrimsonWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct CryingObsidian {}

    struct CutCopper {}

    struct CutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct CutRedSandstone {}

    struct CutRedSandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CutSandstone {}

    struct CutSandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct CyanBanner {
        rotation: state::Rotation16,
    }

    struct CyanBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct CyanCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct CyanCandleCake {
        lit: state::Lit,
    }

    struct CyanCarpet {}

    struct CyanConcrete {}

    struct CyanConcretePowder {}

    struct CyanGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct CyanShulkerBox {}

    struct CyanStainedGlass {}

    struct CyanStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct CyanTerracotta {}

    struct CyanWallBanner {
        facing: state::HorizontalFacing,
    }

    struct CyanWool {}

    struct DamagedAnvil {
        facing: state::HorizontalFacing,
    }

    struct Dandelion {}

    struct DarkOakButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct DarkOakDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct DarkOakFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct DarkOakFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct DarkOakLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct DarkOakLog {
        axis: state::Axis,
    }

    struct DarkOakPlanks {}

    struct DarkOakPressurePlate {
        powered: state::Powered,
    }

    struct DarkOakSapling {
        stage: state::Stage,
    }

    struct DarkOakSign {
        rotation: state::Rotation16,
    }

    struct DarkOakSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct DarkOakStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct DarkOakTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct DarkOakWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct DarkOakWood {
        axis: state::Axis,
    }

    struct DarkPrismarine {}

    struct DarkPrismarineSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct DarkPrismarineStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct DaylightDetector {
        power: state::Power,
        inverted: state::Inverted,
    }

    struct DeadBrainCoral {
        waterlogged: state::Waterlogged,
    }

    struct DeadBrainCoralBlock {}

    struct DeadBrainCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct DeadBrainCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct DeadBubbleCoral {
        waterlogged: state::Waterlogged,
    }

    struct DeadBubbleCoralBlock {}

    struct DeadBubbleCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct DeadBubbleCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct DeadBush {}

    struct DeadFireCoral {
        waterlogged: state::Waterlogged,
    }

    struct DeadFireCoralBlock {}

    struct DeadFireCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct DeadFireCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct DeadHornCoral {
        waterlogged: state::Waterlogged,
    }

    struct DeadHornCoralBlock {}

    struct DeadHornCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct DeadHornCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct DeadTubeCoral {
        waterlogged: state::Waterlogged,
    }

    struct DeadTubeCoralBlock {}

    struct DeadTubeCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct DeadTubeCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct Deepslate {
        axis: state::Axis,
    }

    struct DeepslateBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateBricks {}

    struct DeepslateCoalOre {}

    struct DeepslateCopperOre {}

    struct DeepslateDiamondOre {}

    struct DeepslateEmeraldOre {}

    struct DeepslateGoldOre {}

    struct DeepslateIronOre {}

    struct DeepslateLapisOre {}

    struct DeepslateRedstoneOre {
        lit: state::Lit,
    }

    struct DeepslateTileSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateTileStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateTileWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct DeepslateTiles {}

    struct DetectorRail {
        shape: state::RailShape,
        powered: state::Powered,
        waterlogged: state::Waterlogged,
    }

    struct DiamondBlock {}

    struct DiamondOre {}

    struct Diorite {}

    struct DioriteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct DioriteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct DioriteWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Dirt {}

    struct DirtPath {}

    struct Dispenser {
        facing: state::Facing,
        triggered: state::Triggered,
    }

    struct DragonEgg {}

    struct DragonHead {
        rotation: state::Rotation16,
    }

    struct DragonWallHead {
        facing: state::HorizontalFacing,
    }

    struct DriedKelpBlock {}

    struct DripstoneBlock {}

    struct Dropper {
        facing: state::Facing,
        triggered: state::Triggered,
    }

    struct EmeraldBlock {}

    struct EmeraldOre {}

    struct EnchantingTable {}

    struct EndGateway {}

    struct EndPortal {}

    struct EndPortalFrame {
        facing: state::HorizontalFacing,
        eye: state::Eye,
    }

    struct EndRod {
        facing: state::Facing,
    }

    struct EndStone {}

    struct EndStoneBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct EndStoneBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct EndStoneBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct EndStoneBricks {}

    struct EnderChest {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct ExposedCopper {}

    struct ExposedCutCopper {}

    struct ExposedCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct ExposedCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct Farmland {
        moisture: state::Moisture,
    }

    struct Fern {}

    struct Fire {
        age: state::Age15,
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
    }

    struct FireCoral {
        waterlogged: state::Waterlogged,
    }

    struct FireCoralBlock {}

    struct FireCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct FireCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct FletchingTable {}

    struct FlowerPot {}

    struct FloweringAzalea {}

    struct FloweringAzaleaLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct FrostedIce {
        age: state::Age3,
    }

    struct Furnace {
        facing: state::HorizontalFacing,
        lit: state::Lit,
    }

    struct GildedBlackstone {}

    struct Glass {}

    struct GlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct GlowLichen {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
        down: state::Down,
        waterlogged: state::Waterlogged,
    }

    struct Glowstone {}

    struct GoldBlock {}

    struct GoldOre {}

    struct Granite {}

    struct GraniteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct GraniteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct GraniteWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Grass {}

    struct GrassBlock {
        snowy: state::Snowy,
    }

    struct Gravel {}

    struct GrayBanner {
        rotation: state::Rotation16,
    }

    struct GrayBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct GrayCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct GrayCandleCake {
        lit: state::Lit,
    }

    struct GrayCarpet {}

    struct GrayConcrete {}

    struct GrayConcretePowder {}

    struct GrayGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct GrayShulkerBox {}

    struct GrayStainedGlass {}

    struct GrayStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct GrayTerracotta {}

    struct GrayWallBanner {
        facing: state::HorizontalFacing,
    }

    struct GrayWool {}

    struct GreenBanner {
        rotation: state::Rotation16,
    }

    struct GreenBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct GreenCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct GreenCandleCake {
        lit: state::Lit,
    }

    struct GreenCarpet {}

    struct GreenConcrete {}

    struct GreenConcretePowder {}

    struct GreenGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct GreenShulkerBox {}

    struct GreenStainedGlass {}

    struct GreenStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct GreenTerracotta {}

    struct GreenWallBanner {
        facing: state::HorizontalFacing,
    }

    struct GreenWool {}

    struct Grindstone {
        facing: state::Facing,
        face: state::AttachFace,
    }

    struct HangingRoots {
        waterlogged: state::Waterlogged,
    }

    struct HayBlock {
        axis: state::Axis,
    }

    struct HeavyWeightedPressurePlate {
        power: state::Power,
    }

    struct HoneyBlock {}

    struct HoneycombBlock {}

    struct Hopper {
        facing: state::HopperFacing,
        enabled: state::Enabled,
    }

    struct HornCoral {
        waterlogged: state::Waterlogged,
    }

    struct HornCoralBlock {}

    struct HornCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct HornCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct Ice {}

    struct InfestedChiseledStoneBricks {}

    struct InfestedCobblestone {}

    struct InfestedCrackedStoneBricks {}

    struct InfestedDeepslate {
        axis: state::Axis,
    }

    struct InfestedMossyStoneBricks {}

    struct InfestedStone {}

    struct InfestedStoneBricks {}

    struct IronBars {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct IronBlock {}

    struct IronDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct IronOre {}

    struct IronTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct JackOLantern {
        facing: state::HorizontalFacing,
    }

    struct Jigsaw {
        orientation: state::FrontAndTop,
    }

    struct Jukebox {
        has_record: state::HasRecord,
    }

    struct JungleButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct JungleDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct JungleFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct JungleFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct JungleLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct JungleLog {
        axis: state::Axis,
    }

    struct JunglePlanks {}

    struct JunglePressurePlate {
        powered: state::Powered,
    }

    struct JungleSapling {
        stage: state::Stage,
    }

    struct JungleSign {
        rotation: state::Rotation16,
    }

    struct JungleSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct JungleStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct JungleTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct JungleWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct JungleWood {
        axis: state::Axis,
    }

    struct Kelp {
        age: state::Age25,
    }

    struct KelpPlant {}

    struct Ladder {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct Lantern {
        hanging: state::Hanging,
        waterlogged: state::Waterlogged,
    }

    struct LapisBlock {}

    struct LapisOre {}

    struct LargeAmethystBud {
        facing: state::Facing,
        waterlogged: state::Waterlogged,
    }

    struct LargeFern {
        half: state::DoubleBlockHalf,
    }

    struct Lava {
        level: state::Level,
    }

    struct LavaCauldron {}

    struct Lectern {
        facing: state::HorizontalFacing,
        powered: state::Powered,
        has_book: state::HasBook,
    }

    struct Lever {
        facing: state::Facing,
        face: state::AttachFace,
        powered: state::Powered,
    }

    struct Light {
        level: state::Level,
        waterlogged: state::Waterlogged,
    }

    struct LightBlueBanner {
        rotation: state::Rotation16,
    }

    struct LightBlueBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct LightBlueCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct LightBlueCandleCake {
        lit: state::Lit,
    }

    struct LightBlueCarpet {}

    struct LightBlueConcrete {}

    struct LightBlueConcretePowder {}

    struct LightBlueGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct LightBlueShulkerBox {}

    struct LightBlueStainedGlass {}

    struct LightBlueStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct LightBlueTerracotta {}

    struct LightBlueWallBanner {
        facing: state::HorizontalFacing,
    }

    struct LightBlueWool {}

    struct LightGrayBanner {
        rotation: state::Rotation16,
    }

    struct LightGrayBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct LightGrayCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct LightGrayCandleCake {
        lit: state::Lit,
    }

    struct LightGrayCarpet {}

    struct LightGrayConcrete {}

    struct LightGrayConcretePowder {}

    struct LightGrayGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct LightGrayShulkerBox {}

    struct LightGrayStainedGlass {}

    struct LightGrayStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct LightGrayTerracotta {}

    struct LightGrayWallBanner {
        facing: state::HorizontalFacing,
    }

    struct LightGrayWool {}

    struct LightWeightedPressurePlate {
        power: state::Power,
    }

    struct LightningRod {
        facing: state::Facing,
        powered: state::Powered,
        waterlogged: state::Waterlogged,
    }

    struct Lilac {
        half: state::DoubleBlockHalf,
    }

    struct LilyOfTheValley {}

    struct LilyPad {}

    struct LimeBanner {
        rotation: state::Rotation16,
    }

    struct LimeBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct LimeCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct LimeCandleCake {
        lit: state::Lit,
    }

    struct LimeCarpet {}

    struct LimeConcrete {}

    struct LimeConcretePowder {}

    struct LimeGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct LimeShulkerBox {}

    struct LimeStainedGlass {}

    struct LimeStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct LimeTerracotta {}

    struct LimeWallBanner {
        facing: state::HorizontalFacing,
    }

    struct LimeWool {}

    struct Lodestone {}

    struct Loom {
        facing: state::HorizontalFacing,
    }

    struct MagentaBanner {
        rotation: state::Rotation16,
    }

    struct MagentaBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct MagentaCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct MagentaCandleCake {
        lit: state::Lit,
    }

    struct MagentaCarpet {}

    struct MagentaConcrete {}

    struct MagentaConcretePowder {}

    struct MagentaGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct MagentaShulkerBox {}

    struct MagentaStainedGlass {}

    struct MagentaStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct MagentaTerracotta {}

    struct MagentaWallBanner {
        facing: state::HorizontalFacing,
    }

    struct MagentaWool {}

    struct MagmaBlock {}

    struct MediumAmethystBud {
        facing: state::Facing,
        waterlogged: state::Waterlogged,
    }

    struct Melon {}

    struct MelonStem {
        age: state::Age7,
    }

    struct MossBlock {}

    struct MossCarpet {}

    struct MossyCobblestone {}

    struct MossyCobblestoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct MossyCobblestoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct MossyCobblestoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct MossyStoneBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct MossyStoneBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct MossyStoneBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct MossyStoneBricks {}

    struct MovingPiston {
        facing: state::Facing,
        ty: state::PistonType,
    }

    struct MushroomStem {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
        down: state::Down,
    }

    struct Mycelium {
        snowy: state::Snowy,
    }

    struct NetherBrickFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct NetherBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct NetherBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct NetherBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct NetherBricks {}

    struct NetherGoldOre {}

    struct NetherPortal {
        axis: state::HorizontalAxis,
    }

    struct NetherQuartzOre {}

    struct NetherSprouts {}

    struct NetherWart {
        age: state::Age3,
    }

    struct NetherWartBlock {}

    struct NetheriteBlock {}

    struct Netherrack {}

    struct NoteBlock {
        instrument: state::NoteBlockInstrument,
        note: state::Note,
        powered: state::Powered,
    }

    struct OakButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct OakDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct OakFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct OakFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct OakLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct OakLog {
        axis: state::Axis,
    }

    struct OakPlanks {}

    struct OakPressurePlate {
        powered: state::Powered,
    }

    struct OakSapling {
        stage: state::Stage,
    }

    struct OakSign {
        rotation: state::Rotation16,
    }

    struct OakSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct OakStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct OakTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct OakWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct OakWood {
        axis: state::Axis,
    }

    struct Observer {
        facing: state::Facing,
        powered: state::Powered,
    }

    struct Obsidian {}

    struct OrangeBanner {
        rotation: state::Rotation16,
    }

    struct OrangeBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct OrangeCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct OrangeCandleCake {
        lit: state::Lit,
    }

    struct OrangeCarpet {}

    struct OrangeConcrete {}

    struct OrangeConcretePowder {}

    struct OrangeGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct OrangeShulkerBox {}

    struct OrangeStainedGlass {}

    struct OrangeStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct OrangeTerracotta {}

    struct OrangeTulip {}

    struct OrangeWallBanner {
        facing: state::HorizontalFacing,
    }

    struct OrangeWool {}

    struct OxeyeDaisy {}

    struct OxidizedCopper {}

    struct OxidizedCutCopper {}

    struct OxidizedCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct OxidizedCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PackedIce {}

    struct Peony {
        half: state::DoubleBlockHalf,
    }

    struct PetrifiedOakSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PinkBanner {
        rotation: state::Rotation16,
    }

    struct PinkBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct PinkCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct PinkCandleCake {
        lit: state::Lit,
    }

    struct PinkCarpet {}

    struct PinkConcrete {}

    struct PinkConcretePowder {}

    struct PinkGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct PinkShulkerBox {}

    struct PinkStainedGlass {}

    struct PinkStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct PinkTerracotta {}

    struct PinkTulip {}

    struct PinkWallBanner {
        facing: state::HorizontalFacing,
    }

    struct PinkWool {}

    struct Piston {
        facing: state::Facing,
        extended: state::Extended,
    }

    struct PistonHead {
        facing: state::Facing,
        ty: state::PistonType,
        short: state::Short,
    }

    struct PlayerHead {
        rotation: state::Rotation16,
    }

    struct PlayerWallHead {
        facing: state::HorizontalFacing,
    }

    struct Podzol {
        snowy: state::Snowy,
    }

    struct PointedDripstone {
        vertical_direction: state::VerticalDirection,
        thickness: state::DripstoneThickness,
        waterlogged: state::Waterlogged,
    }

    struct PolishedAndesite {}

    struct PolishedAndesiteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedAndesiteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBasalt {
        axis: state::Axis,
    }

    struct PolishedBlackstone {}

    struct PolishedBlackstoneBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBlackstoneBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBlackstoneBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBlackstoneBricks {}

    struct PolishedBlackstoneButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct PolishedBlackstonePressurePlate {
        powered: state::Powered,
    }

    struct PolishedBlackstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBlackstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PolishedBlackstoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct PolishedDeepslate {}

    struct PolishedDeepslateSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedDeepslateStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PolishedDeepslateWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct PolishedDiorite {}

    struct PolishedDioriteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedDioriteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PolishedGranite {}

    struct PolishedGraniteSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PolishedGraniteStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct Poppy {}

    struct Potatoes {
        age: state::Age7,
    }

    struct PottedAcaciaSapling {
        stage: state::Stage,
    }

    struct PottedAllium {}

    struct PottedAzaleaBush {}

    struct PottedAzureBluet {}

    struct PottedBamboo {}

    struct PottedBirchSapling {
        stage: state::Stage,
    }

    struct PottedBlueOrchid {}

    struct PottedBrownMushroom {}

    struct PottedCactus {}

    struct PottedCornflower {}

    struct PottedCrimsonFungus {}

    struct PottedCrimsonRoots {}

    struct PottedDandelion {}

    struct PottedDarkOakSapling {
        stage: state::Stage,
    }

    struct PottedDeadBush {}

    struct PottedFern {}

    struct PottedFloweringAzaleaBush {}

    struct PottedJungleSapling {
        stage: state::Stage,
    }

    struct PottedLilyOfTheValley {}

    struct PottedOakSapling {
        stage: state::Stage,
    }

    struct PottedOrangeTulip {}

    struct PottedOxeyeDaisy {}

    struct PottedPinkTulip {}

    struct PottedPoppy {}

    struct PottedRedMushroom {}

    struct PottedRedTulip {}

    struct PottedSpruceSapling {
        stage: state::Stage,
    }

    struct PottedWarpedFungus {}

    struct PottedWarpedRoots {}

    struct PottedWhiteTulip {}

    struct PottedWitherRose {}

    struct PowderSnow {}

    struct PowderSnowCauldron {
        level: state::LevelCauldron,
    }

    struct PoweredRail {
        shape: state::RailShape,
        powered: state::Powered,
        waterlogged: state::Waterlogged,
    }

    struct Prismarine {}

    struct PrismarineBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PrismarineBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PrismarineBricks {}

    struct PrismarineSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PrismarineStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct PrismarineWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Pumpkin {}

    struct PumpkinStem {
        age: state::Age7,
    }

    struct PurpleBanner {
        rotation: state::Rotation16,
    }

    struct PurpleBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct PurpleCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct PurpleCandleCake {
        lit: state::Lit,
    }

    struct PurpleCarpet {}

    struct PurpleConcrete {}

    struct PurpleConcretePowder {}

    struct PurpleGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct PurpleShulkerBox {}

    struct PurpleStainedGlass {}

    struct PurpleStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct PurpleTerracotta {}

    struct PurpleWallBanner {
        facing: state::HorizontalFacing,
    }

    struct PurpleWool {}

    struct PurpurBlock {}

    struct PurpurPillar {
        axis: state::Axis,
    }

    struct PurpurSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct PurpurStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct QuartzBlock {}

    struct QuartzBricks {}

    struct QuartzPillar {
        axis: state::Axis,
    }

    struct QuartzSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct QuartzStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct Rail {
        shape: state::RailShape,
        waterlogged: state::Waterlogged,
    }

    struct RawCopperBlock {}

    struct RawGoldBlock {}

    struct RawIronBlock {}

    struct RedBanner {
        rotation: state::Rotation16,
    }

    struct RedBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct RedCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct RedCandleCake {
        lit: state::Lit,
    }

    struct RedCarpet {}

    struct RedConcrete {}

    struct RedConcretePowder {}

    struct RedGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct RedMushroom {}

    struct RedMushroomBlock {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
        down: state::Down,
    }

    struct RedNetherBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct RedNetherBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct RedNetherBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct RedNetherBricks {}

    struct RedSand {}

    struct RedSandstone {}

    struct RedSandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct RedSandstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct RedSandstoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct RedShulkerBox {}

    struct RedStainedGlass {}

    struct RedStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct RedTerracotta {}

    struct RedTulip {}

    struct RedWallBanner {
        facing: state::HorizontalFacing,
    }

    struct RedWool {}

    struct RedstoneBlock {}

    struct RedstoneLamp {
        lit: state::Lit,
    }

    struct RedstoneOre {
        lit: state::Lit,
    }

    struct RedstoneTorch {
        lit: state::Lit,
    }

    struct RedstoneWallTorch {
        facing: state::HorizontalFacing,
        lit: state::Lit,
    }

    struct RedstoneWire {
        north: state::RedstoneSide,
        east: state::RedstoneSide,
        south: state::RedstoneSide,
        west: state::RedstoneSide,
        power: state::Power,
    }

    struct Repeater {
        facing: state::HorizontalFacing,
        delay: state::Delay,
        locked: state::Locked,
        powered: state::Powered,
    }

    struct RepeatingCommandBlock {
        facing: state::Facing,
        conditional: state::Conditional,
    }

    struct RespawnAnchor {
        charges: state::RespawnAnchorCharges,
    }

    struct RootedDirt {}

    struct RoseBush {
        half: state::DoubleBlockHalf,
    }

    struct Sand {}

    struct Sandstone {}

    struct SandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct SandstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct SandstoneWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct Scaffolding {
        distance: state::StabilityDistance,
        waterlogged: state::Waterlogged,
        bottom: state::Bottom,
    }

    struct SculkSensor {
        sculk_sensor_phase: state::SculkSensorPhase,
        power: state::Power,
        waterlogged: state::Waterlogged,
    }

    struct SeaLantern {}

    struct SeaPickle {
        pickles: state::Pickles,
        waterlogged: state::Waterlogged,
    }

    struct Seagrass {}

    struct Shroomlight {}

    struct ShulkerBox {
        facing: state::Facing,
    }

    struct SkeletonSkull {
        rotation: state::Rotation16,
    }

    struct SkeletonWallSkull {
        facing: state::HorizontalFacing,
    }

    struct SlimeBlock {}

    struct SmallAmethystBud {
        facing: state::Facing,
        waterlogged: state::Waterlogged,
    }

    struct SmallDripleaf {
        facing: state::HorizontalFacing,
        half: state::DoubleBlockHalf,
        waterlogged: state::Waterlogged,
    }

    struct SmithingTable {}

    struct Smoker {
        facing: state::HorizontalFacing,
        lit: state::Lit,
    }

    struct SmoothBasalt {}

    struct SmoothQuartz {}

    struct SmoothQuartzSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct SmoothQuartzStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct SmoothRedSandstone {}

    struct SmoothRedSandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct SmoothRedSandstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct SmoothSandstone {}

    struct SmoothSandstoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct SmoothSandstoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct SmoothStone {}

    struct SmoothStoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct Snow {
        layers: state::Layers,
    }

    struct SnowBlock {}

    struct SoulCampfire {
        facing: state::HorizontalFacing,
        lit: state::Lit,
        signal_fire: state::SignalFire,
        waterlogged: state::Waterlogged,
    }

    struct SoulFire {
        age: state::Age15,
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
    }

    struct SoulLantern {
        hanging: state::Hanging,
        waterlogged: state::Waterlogged,
    }

    struct SoulSand {}

    struct SoulSoil {}

    struct SoulTorch {}

    struct SoulWallTorch {
        facing: state::HorizontalFacing,
    }

    struct Spawner {}

    struct Sponge {}

    struct SporeBlossom {}

    struct SpruceButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct SpruceDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct SpruceFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct SpruceFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct SpruceLeaves {
        distance: state::Distance,
        persistent: state::Persistent,
    }

    struct SpruceLog {
        axis: state::Axis,
    }

    struct SprucePlanks {}

    struct SprucePressurePlate {
        powered: state::Powered,
    }

    struct SpruceSapling {
        stage: state::Stage,
    }

    struct SpruceSign {
        rotation: state::Rotation16,
    }

    struct SpruceSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct SpruceStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct SpruceTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct SpruceWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct SpruceWood {
        axis: state::Axis,
    }

    struct StickyPiston {
        facing: state::Facing,
        extended: state::Extended,
    }

    struct Stone {}

    struct StoneBrickSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct StoneBrickStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct StoneBrickWall {
        north: state::WallSide,
        east: state::WallSide,
        south: state::WallSide,
        west: state::WallSide,
        up: state::Up,
        waterlogged: state::Waterlogged,
    }

    struct StoneBricks {}

    struct StoneButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct StonePressurePlate {
        powered: state::Powered,
    }

    struct StoneSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct StoneStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct Stonecutter {
        facing: state::HorizontalFacing,
    }

    struct StrippedAcaciaLog {
        axis: state::Axis,
    }

    struct StrippedAcaciaWood {
        axis: state::Axis,
    }

    struct StrippedBirchLog {
        axis: state::Axis,
    }

    struct StrippedBirchWood {
        axis: state::Axis,
    }

    struct StrippedCrimsonHyphae {
        axis: state::Axis,
    }

    struct StrippedCrimsonStem {
        axis: state::Axis,
    }

    struct StrippedDarkOakLog {
        axis: state::Axis,
    }

    struct StrippedDarkOakWood {
        axis: state::Axis,
    }

    struct StrippedJungleLog {
        axis: state::Axis,
    }

    struct StrippedJungleWood {
        axis: state::Axis,
    }

    struct StrippedOakLog {
        axis: state::Axis,
    }

    struct StrippedOakWood {
        axis: state::Axis,
    }

    struct StrippedSpruceLog {
        axis: state::Axis,
    }

    struct StrippedSpruceWood {
        axis: state::Axis,
    }

    struct StrippedWarpedHyphae {
        axis: state::Axis,
    }

    struct StrippedWarpedStem {
        axis: state::Axis,
    }

    struct StructureBlock {
        mode: state::StructureMode,
    }

    struct StructureVoid {}

    struct SugarCane {
        age: state::Age15,
    }

    struct Sunflower {
        half: state::DoubleBlockHalf,
    }

    struct SweetBerryBush {
        age: state::Age3,
    }

    struct TallGrass {
        half: state::DoubleBlockHalf,
    }

    struct TallSeagrass {
        half: state::DoubleBlockHalf,
    }

    struct Target {
        power: state::Power,
    }

    struct Terracotta {}

    struct TintedGlass {}

    struct Tnt {
        unstable: state::Unstable,
    }

    struct Torch {}

    struct TrappedChest {}

    struct Tripwire {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        attached: state::Attached,
        powered: state::Powered,
    }

    struct TripwireHook {
        facing: state::HorizontalFacing,
        attached: state::Attached,
        powered: state::Powered,
    }

    struct TubeCoral {
        waterlogged: state::Waterlogged,
    }

    struct TubeCoralBlock {}

    struct TubeCoralFan {
        waterlogged: state::Waterlogged,
    }

    struct TubeCoralWallFan {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct Tuff {}

    struct TurtleEgg {
        hatch: state::Hatch,
        eggs: state::Eggs,
    }

    struct TwistingVines {
        age: state::Age25,
    }

    struct TwistingVinesPlant {}

    struct Vine {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        up: state::Up,
    }

    struct VoidAir {}

    struct WallTorch {
        facing: state::HorizontalFacing,
    }

    struct WarpedButton {
        facing: state::Facing,
        powered: state::Powered,
        face: state::AttachFace,
    }

    struct WarpedDoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        hinge: state::DoorHingeSide,
        half: state::DoubleBlockHalf,
        powered: state::Powered,
    }

    struct WarpedFence {
        north: state::North,
        east: state::East,
        west: state::West,
        south: state::South,
        waterlogged: state::Waterlogged,
    }

    struct WarpedFenceGate {
        facing: state::HorizontalFacing,
        open: state::Open,
        powered: state::Powered,
        in_wall: state::InWall,
    }

    struct WarpedFungus {}

    struct WarpedHyphae {
        axis: state::Axis,
    }

    struct WarpedNylium {}

    struct WarpedPlanks {}

    struct WarpedPressurePlate {
        powered: state::Powered,
    }

    struct WarpedRoots {}

    struct WarpedSign {
        rotation: state::Rotation16,
    }

    struct WarpedSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WarpedStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WarpedStem {
        axis: state::Axis,
    }

    struct WarpedTrapdoor {
        facing: state::HorizontalFacing,
        open: state::Open,
        half: state::Half,
        waterlogged: state::Waterlogged,
    }

    struct WarpedWallSign {
        facing: state::HorizontalFacing,
        waterlogged: state::Waterlogged,
    }

    struct WarpedWartBlock {}

    struct Water {
        level: state::Level,
    }

    struct WaterCauldron {
        level: state::LevelCauldron,
    }

    struct WaxedCopperBlock {}

    struct WaxedCutCopper {}

    struct WaxedCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WaxedCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WaxedExposedCopper {}

    struct WaxedExposedCutCopper {}

    struct WaxedExposedCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WaxedExposedCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WaxedOxidizedCopper {}

    struct WaxedOxidizedCutCopper {}

    struct WaxedOxidizedCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WaxedOxidizedCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WaxedWeatheredCopper {}

    struct WaxedWeatheredCutCopper {}

    struct WaxedWeatheredCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WaxedWeatheredCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WeatheredCopper {}

    struct WeatheredCutCopper {}

    struct WeatheredCutCopperSlab {
        ty: state::SlabType,
        waterlogged: state::Waterlogged,
    }

    struct WeatheredCutCopperStairs {
        facing: state::HorizontalFacing,
        half: state::Half,
        shape: state::StairsShape,
        waterlogged: state::Waterlogged,
    }

    struct WeepingVines {
        age: state::Age25,
    }

    struct WeepingVinesPlant {}

    struct WetSponge {}

    struct Wheat {
        age: state::Age7,
    }

    struct WhiteBanner {
        rotation: state::Rotation16,
    }

    struct WhiteBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct WhiteCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct WhiteCandleCake {
        lit: state::Lit,
    }

    struct WhiteCarpet {}

    struct WhiteConcrete {}

    struct WhiteConcretePowder {}

    struct WhiteGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct WhiteShulkerBox {}

    struct WhiteStainedGlass {}

    struct WhiteStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct WhiteTerracotta {}

    struct WhiteTulip {}

    struct WhiteWallBanner {
        facing: state::HorizontalFacing,
    }

    struct WhiteWool {}

    struct WitherRose {}

    struct WitherSkeletonSkull {
        rotation: state::Rotation16,
    }

    struct WitherSkeletonWallSkull {
        facing: state::HorizontalFacing,
    }

    struct YellowBanner {
        rotation: state::Rotation16,
    }

    struct YellowBed {
        facing: state::HorizontalFacing,
        part: state::BedPart,
        occupied: state::Occupied,
    }

    struct YellowCandle {
        candles: state::Candles,
        lit: state::Lit,
        waterlogged: state::Waterlogged,
    }

    struct YellowCandleCake {
        lit: state::Lit,
    }

    struct YellowCarpet {}

    struct YellowConcrete {}

    struct YellowConcretePowder {}

    struct YellowGlazedTerracotta {
        facing: state::HorizontalFacing,
    }

    struct YellowShulkerBox {}

    struct YellowStainedGlass {}

    struct YellowStainedGlassPane {
        north: state::North,
        east: state::East,
        south: state::South,
        west: state::West,
        waterlogged: state::Waterlogged,
    }

    struct YellowTerracotta {}

    struct YellowWallBanner {
        facing: state::HorizontalFacing,
    }

    struct YellowWool {}

    struct ZombieHead {
        rotation: state::Rotation16,
    }

    struct ZombieWallHead {
        facing: state::HorizontalFacing,
    }
}
