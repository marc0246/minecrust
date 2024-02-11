//! Everything concerning block state.

use std::fmt;
use std::ops::RangeInclusive;
use std::str::FromStr;

use crate::macros::properties;

/// Defines the state properties of a particular block.
pub struct Definition {
    /// The names of the properties.
    pub property_names: &'static [&'static str],
    /// The properties.
    pub properties: &'static [PropertyDefinition],
}

/// Defines a state property of a particular block.
pub struct PropertyDefinition {
    /// The name of the property.
    pub name: &'static str,
    /// The ID of the property.
    pub id: PropertyId,
    /// The offset within the block state at which the property is stored.
    pub offset: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PropertyType {
    Boolean,
    Integer,
    Enum,
}

///////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct Integer<const MIN: u32, const MAX: u32>(u32);

impl<const MIN: u32, const MAX: u32> Integer<MIN, MAX> {
    const MIN: u32 = MIN;

    const MAX: u32 = MAX;

    /// Wraps the provided `u32` in an `Integer`.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the `u32` is outside of the range of this type.
    pub fn new(n: u32) -> Result<Self, OutOfRangeError> {
        assert!(MAX > MIN);

        if (MIN..=MAX).contains(&n) {
            Ok(Integer(n))
        } else {
            Err(OutOfRangeError {
                value: n,
                range: MIN..=MAX,
            })
        }
    }

    /// Replaces the wrapped `u32` with the provided one.
    ///
    /// # Errors
    ///
    /// Returns `Err` when the `u32` is outside of the range of this type.
    pub fn set(&mut self, n: u32) -> Result<(), OutOfRangeError> {
        if (MIN..=MAX).contains(&n) {
            self.0 = n;

            Ok(())
        } else {
            Err(OutOfRangeError {
                value: n,
                range: MIN..=MAX,
            })
        }
    }
}

impl<const MIN: u32, const MAX: u32> Default for Integer<MIN, MAX> {
    fn default() -> Self {
        assert!(MAX > MIN);

        Integer(MIN)
    }
}

impl<const MIN: u32, const MAX: u32> FromStr for Integer<MIN, MAX> {
    type Err = ParseIntegerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let n = s.parse().map_err(ParseIntegerError::ParseIntError)?;

        Self::new(n).map_err(ParseIntegerError::OutOfRange)
    }
}

#[derive(Debug)]
pub struct OutOfRangeError {
    value: u32,
    range: RangeInclusive<u32>,
}

impl fmt::Display for OutOfRangeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected integer in the range [{}, {}], found {}",
            self.range.start(),
            self.range.end(),
            self.value,
        )
    }
}

impl std::error::Error for OutOfRangeError {}

#[derive(Debug)]
pub enum ParseIntegerError {
    ParseIntError(std::num::ParseIntError),
    OutOfRange(OutOfRangeError),
}

impl fmt::Display for ParseIntegerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ParseIntError(e) => e.fmt(f),
            Self::OutOfRange(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ParseIntegerError {}

///////////////////////////////////////////////////////////////////////////////////////////////////

properties! {
    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Booleans
    ///////////////////////////////////////////////////////////////////////////////////////////////

    pub struct Attached(bool);
    pub struct Berries(bool);
    pub struct Bottom(bool);
    pub struct Conditional(bool);
    pub struct Disarmed(bool);
    pub struct Drag(bool);
    pub struct Enabled(bool);
    pub struct Extended(bool);
    pub struct Eye(bool);
    pub struct Falling(bool);
    pub struct Hanging(bool);
    pub struct HasBottle0(bool);
    pub struct HasBottle1(bool);
    pub struct HasBottle2(bool);
    pub struct HasRecord(bool);
    pub struct HasBook(bool);
    pub struct Inverted(bool);
    pub struct InWall(bool);
    pub struct Lit(bool);
    pub struct Locked(bool);
    pub struct Occupied(bool);
    pub struct Open(bool);
    pub struct Persistent(bool);
    pub struct Powered(bool);
    pub struct Short(bool);
    pub struct SignalFire(bool);
    pub struct Snowy(bool);
    pub struct Triggered(bool);
    pub struct Unstable(bool);
    pub struct Waterlogged(bool);
    pub struct VineEnd(bool);

    pub struct Up(bool);
    pub struct Down(bool);
    pub struct North(bool);
    pub struct East(bool);
    pub struct South(bool);
    pub struct West(bool);

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Enums
    ///////////////////////////////////////////////////////////////////////////////////////////////

    pub enum AttachFace {
        #[default]
        Floor,
        Wall,
        Ceiling,
    }

    pub enum Axis {
        #[default]
        X,
        Y,
        Z,
    }

    pub enum BambooLeaves {
        #[default]
        None,
        Small,
        Large,
    }

    pub enum BedPart {
        #[default]
        Head,
        Foot,
    }

    pub enum BellAttachType {
        #[default]
        Floor,
        Ceiling,
        SingleWall,
        DoubleWall,
    }

    pub enum ChestType {
        #[default]
        Single,
        Right,
        Left,
    }

    pub enum ComparatorMode {
        #[default]
        Compare,
        Subtract,
    }

    pub enum DoorHingeSide {
        #[default]
        Left,
        Right,
    }

    pub enum DoubleBlockHalf {
        #[default]
        Upper,
        Lower,
    }

    pub enum DripstoneThickness {
        #[default]
        TipMerge,
        Tip,
        Frustum,
        Middle,
        Base,
    }

    pub enum Facing {
        #[default]
        Down,
        Up,
        North,
        South,
        West,
        East,
    }

    pub enum FrontAndTop {
        #[default]
        DownNorth,
        DownEast,
        DownSouth,
        DownWest,
        UpNorth,
        UpEast,
        UpSouth,
        UpWest,
        NorthUp,
        EastUp,
        SouthUp,
        WestUp,
    }

    pub enum Half {
        #[default]
        Top,
        Bottom,
    }

    pub enum HorizontalAxis {
        #[default]
        X,
        Z,
    }

    pub enum HopperFacing {
        #[default]
        Down,
        North,
        East,
        South,
        West,
    }

    pub enum HorizontalFacing {
        #[default]
        North,
        East,
        South,
        West,
    }

    pub enum NoteBlockInstrument {
        #[default]
        Harp,
        Basedrum,
        Snare,
        Hat,
        Bass,
        Flute,
        Bell,
        Guitar,
        Chime,
        Xylophone,
        IronXylophone,
        CowBell,
        Didgeridoo,
        Bit,
        Banjo,
        Pling,
    }

    pub enum PistonType {
        #[default]
        Normal,
        Sticky,
    }

    pub enum RailShape {
        #[default]
        NorthSouth,
        EastWest,
        AscendingEast,
        AscendingWest,
        AscendingNorth,
        AscendingSouth,
        SouthEast,
        SouthWest,
        NorthWest,
        NorthEast,
    }

    pub enum RedstoneSide {
        #[default]
        Up,
        Side,
        None,
    }

    pub enum SculkSensorPhase {
        #[default]
        Inactive,
        Active,
        Cooldown,
    }

    pub enum SlabType {
        #[default]
        Top,
        Bottom,
        Double,
    }

    pub enum StairsShape {
        #[default]
        Straight,
        InnerLeft,
        InnerRight,
        OuterLeft,
        OuterRight,
    }

    pub enum StructureMode {
        #[default]
        Save,
        Load,
        Corner,
        Data,
    }

    pub enum Tilt {
        #[default]
        None,
        Unstable,
        Partial,
        Full,
    }

    pub enum VerticalDirection {
        #[default]
        Up,
        Down,
    }

    pub enum WallSide {
        #[default]
        None,
        Low,
        Tall,
    }

    pub enum WoodType {
        #[default]
        Oak,
        Spruce,
        Birch,
        Acacia,
        Jungle,
        DarkOak,
        Crimson,
        Warped,
    }

    ///////////////////////////////////////////////////////////////////////////////////////////////
    // Integers
    ///////////////////////////////////////////////////////////////////////////////////////////////

    pub struct Age1(Integer<0, 1>);
    pub struct Age2(Integer<0, 2>);
    pub struct Age3(Integer<0, 3>);
    pub struct Age5(Integer<0, 5>);
    pub struct Age7(Integer<0, 7>);
    pub struct Age15(Integer<0, 15>);
    pub struct Age25(Integer<0, 25>);
    pub struct Bites(Integer<0, 6>);
    pub struct Candles(Integer<1, 4>);
    pub struct Delay(Integer<1, 4>);
    pub struct Distance(Integer<1, 7>);
    pub struct Eggs(Integer<1, 4>);
    pub struct Hatch(Integer<0, 2>);
    pub struct Layers(Integer<1, 8>);
    pub struct LevelCauldron(Integer<1, 3>);
    pub struct LevelComposter(Integer<0, 8>);
    pub struct LevelFlowing(Integer<1, 8>); // TODO: what's this?
    pub struct LevelHoney(Integer<0, 5>);
    pub struct Level(Integer<0, 15>);
    pub struct Moisture(Integer<0, 7>);
    pub struct Note(Integer<0, 24>);
    pub struct Pickles(Integer<1, 4>);
    pub struct Power(Integer<0, 15>);
    pub struct Stage(Integer<0, 1>);
    pub struct StabilityDistance(Integer<0, 7>);
    pub struct RespawnAnchorCharges(Integer<0, 4>);
    pub struct Rotation16(Integer<0, 15>);
}
