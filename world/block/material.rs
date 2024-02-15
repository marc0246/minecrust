#[allow(clippy::struct_excessive_bools)] // This is, in fact, not a state machine.
#[derive(Debug)]
pub struct Material {
    pub color: Color,
    pub blocks_motion: bool,
    pub solid_blocking: bool,
    pub solid: bool,
    pub replaceable: bool,
    pub flammable: bool,
    pub liquid: bool,
    pub push_reaction: PushReaction,
}

impl Material {
    pub(crate) const AIR: Self = Self::new(Color::None)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .replaceable();

    pub(crate) const STRUCTURAL_AIR: Self = Self::new(Color::None)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .replaceable();

    pub(crate) const PORTAL: Self = Self::new(Color::None)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .not_pushable();

    pub(crate) const CLOTH_DECORATION: Self = Self::new(Color::Wool)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .flammable();

    pub(crate) const PLANT: Self = Self::new(Color::Plant)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push();

    pub(crate) const WATER_PLANT: Self = Self::new(Color::Water)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push();

    pub(crate) const REPLACEABLE_PLANT: Self = Self::new(Color::Plant)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable()
        .flammable();

    pub(crate) const REPLACEABLE_FIREPROOF_PLANT: Self = Self::new(Color::Plant)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable();

    pub(crate) const REPLACEABLE_WATER_PLANT: Self = Self::new(Color::Water)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable();

    pub(crate) const WATER: Self = Self::new(Color::Water)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable()
        .liquid();

    pub(crate) const BUBBLE_COLUMN: Self = Self::new(Color::Water)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable()
        .liquid();

    pub(crate) const LAVA: Self = Self::new(Color::Fire)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable()
        .liquid();

    pub(crate) const TOP_SNOW: Self = Self::new(Color::Snow)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable();

    pub(crate) const FIRE: Self = Self::new(Color::None)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push()
        .replaceable();

    pub(crate) const DECORATION: Self = Self::new(Color::None)
        .no_collider()
        .non_solid_blocking()
        .non_solid()
        .destroy_on_push();

    pub(crate) const WEB: Self = Self::new(Color::Wool)
        .no_collider()
        .non_solid_blocking()
        .destroy_on_push();

    pub(crate) const SCULK: Self = Self::new(Color::ColorBlack);

    pub(crate) const BUILDABLE_GLASS: Self = Self::new(Color::None);

    pub(crate) const CLAY: Self = Self::new(Color::Clay);

    pub(crate) const DIRT: Self = Self::new(Color::Dirt);

    pub(crate) const GRASS: Self = Self::new(Color::Grass);

    pub(crate) const ICE_SOLID: Self = Self::new(Color::Ice);

    pub(crate) const SAND: Self = Self::new(Color::Sand);

    pub(crate) const SPONGE: Self = Self::new(Color::ColorYellow);

    pub(crate) const SHULKER_SHELL: Self = Self::new(Color::ColorPurple);

    pub(crate) const WOOD: Self = Self::new(Color::Wood).flammable();

    pub(crate) const NETHER_WOOD: Self = Self::new(Color::Wood);

    pub(crate) const BAMBOO_SAPLING: Self = Self::new(Color::Wood)
        .no_collider()
        .flammable()
        .destroy_on_push();

    pub(crate) const BAMBOO: Self = Self::new(Color::Wood).flammable().destroy_on_push();

    pub(crate) const WOOL: Self = Self::new(Color::Wool).flammable();

    pub(crate) const EXPLOSIVE: Self = Self::new(Color::Fire).non_solid_blocking().flammable();

    pub(crate) const LEAVES: Self = Self::new(Color::Plant)
        .non_solid_blocking()
        .flammable()
        .destroy_on_push();

    pub(crate) const GLASS: Self = Self::new(Color::None).non_solid_blocking();

    pub(crate) const ICE: Self = Self::new(Color::Ice).non_solid_blocking();

    pub(crate) const CACTUS: Self = Self::new(Color::Plant)
        .non_solid_blocking()
        .destroy_on_push();

    pub(crate) const STONE: Self = Self::new(Color::Stone);

    pub(crate) const METAL: Self = Self::new(Color::Metal);

    pub(crate) const SNOW: Self = Self::new(Color::Snow);

    pub(crate) const HEAVY_METAL: Self = Self::new(Color::Metal).not_pushable();

    pub(crate) const BARRIER: Self = Self::new(Color::None).not_pushable();

    pub(crate) const PISTON: Self = Self::new(Color::Stone).not_pushable();

    pub(crate) const MOSS: Self = Self::new(Color::Plant).destroy_on_push();

    pub(crate) const VEGETABLE: Self = Self::new(Color::Plant).destroy_on_push();

    pub(crate) const EGG: Self = Self::new(Color::Plant).destroy_on_push();

    pub(crate) const CAKE: Self = Self::new(Color::None).destroy_on_push();

    pub(crate) const AMETHYST: Self = Self::new(Color::ColorPurple);

    pub(crate) const POWDER_SNOW: Self = Self::new(Color::Snow).no_collider().non_solid();

    const fn new(color: Color) -> Self {
        Material {
            color,
            push_reaction: PushReaction::Normal,
            blocks_motion: true,
            flammable: false,
            liquid: false,
            solid_blocking: true,
            replaceable: false,
            solid: true,
        }
    }

    const fn no_collider(mut self) -> Self {
        self.blocks_motion = false;

        self
    }

    const fn non_solid_blocking(mut self) -> Self {
        self.solid_blocking = false;

        self
    }

    const fn non_solid(mut self) -> Self {
        self.solid = false;

        self
    }

    const fn flammable(mut self) -> Self {
        self.flammable = true;

        self
    }

    const fn replaceable(mut self) -> Self {
        self.replaceable = true;

        self
    }

    const fn liquid(mut self) -> Self {
        self.liquid = true;

        self
    }

    const fn destroy_on_push(mut self) -> Self {
        self.push_reaction = PushReaction::Destroy;

        self
    }

    const fn not_pushable(mut self) -> Self {
        self.push_reaction = PushReaction::Block;

        self
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// Colors used for blocks displayed in a map.
#[allow(clippy::enum_variant_names)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Color {
    None,
    Grass,
    Sand,
    Wool,
    Fire,
    Ice,
    Metal,
    Plant,
    Snow,
    Clay,
    Dirt,
    Stone,
    Water,
    Wood,
    Quartz,
    ColorOrange,
    ColorMagenta,
    ColorLightBlue,
    ColorYellow,
    ColorLightGreen,
    ColorPink,
    ColorGray,
    ColorLightGray,
    ColorCyan,
    ColorPurple,
    ColorBlue,
    ColorBrown,
    ColorGreen,
    ColorRed,
    ColorBlack,
    Gold,
    Diamond,
    Lapis,
    Emerald,
    Podzol,
    Nether,
    TerracottaWhite,
    TerracottaOrange,
    TerracottaMagenta,
    TerracottaLightBlue,
    TerracottaYellow,
    TerracottaLightGreen,
    TerracottaPink,
    TerracottaGray,
    TerracottaLightGray,
    TerracottaCyan,
    TerracottaPurple,
    TerracottaBlue,
    TerracottaBrown,
    TerracottaGreen,
    TerracottaRed,
    TerracottaBlack,
    CrimsonNylium,
    CrimsonStem,
    CrimsonHyphae,
    WarpedNylium,
    WarpedStem,
    WarpedHyphae,
    WarpedWartBlock,
    Deepslate,
    RawIron,
    GlowLichen,
}

impl Color {
    #[allow(clippy::unreadable_literal)] // Sir, this is a color.
    #[must_use]
    #[rustfmt::skip]
    pub fn rgb(self) -> u32 {
        match self {
            Self::None                 => 0x000000,
            Self::Grass                => 0x7fb238,
            Self::Sand                 => 0xf7e9a3,
            Self::Wool                 => 0xc7c7c7,
            Self::Fire                 => 0xff0000,
            Self::Ice                  => 0xa0a0ff,
            Self::Metal                => 0xa7a7a7,
            Self::Plant                => 0x007c00,
            Self::Snow                 => 0xffffff,
            Self::Clay                 => 0xa4a8b8,
            Self::Dirt                 => 0x976d4d,
            Self::Stone                => 0x707070,
            Self::Water                => 0x4040ff,
            Self::Wood                 => 0x8f7748,
            Self::Quartz               => 0xfffcf5,
            Self::ColorOrange          => 0xd87f33,
            Self::ColorMagenta         => 0xb24cd8,
            Self::ColorLightBlue       => 0x6699d8,
            Self::ColorYellow          => 0xe5e533,
            Self::ColorLightGreen      => 0x7fcc19,
            Self::ColorPink            => 0xf27fa5,
            Self::ColorGray            => 0x4c4c4c,
            Self::ColorLightGray       => 0x999999,
            Self::ColorCyan            => 0x4c7f99,
            Self::ColorPurple          => 0x7f3fb2,
            Self::ColorBlue            => 0x334cb2,
            Self::ColorBrown           => 0x664c33,
            Self::ColorGreen           => 0x667f33,
            Self::ColorRed             => 0x993333,
            Self::ColorBlack           => 0x191919,
            Self::Gold                 => 0xfaee4d,
            Self::Diamond              => 0x5cdbd5,
            Self::Lapis                => 0x4a80ff,
            Self::Emerald              => 0x00d93a,
            Self::Podzol               => 0x815631,
            Self::Nether               => 0x700200,
            Self::TerracottaWhite      => 0xd1b1a1,
            Self::TerracottaOrange     => 0x9f5224,
            Self::TerracottaMagenta    => 0x95576c,
            Self::TerracottaLightBlue  => 0x706c8a,
            Self::TerracottaYellow     => 0xba8524,
            Self::TerracottaLightGreen => 0x677535,
            Self::TerracottaPink       => 0xa04d4e,
            Self::TerracottaGray       => 0x392923,
            Self::TerracottaLightGray  => 0x876b62,
            Self::TerracottaCyan       => 0x575c5c,
            Self::TerracottaPurple     => 0x7a4958,
            Self::TerracottaBlue       => 0x4c3e5c,
            Self::TerracottaBrown      => 0x4c3223,
            Self::TerracottaGreen      => 0x4c522a,
            Self::TerracottaRed        => 0x8e3c2e,
            Self::TerracottaBlack      => 0x251610,
            Self::CrimsonNylium        => 0xbd3031,
            Self::CrimsonStem          => 0x943f61,
            Self::CrimsonHyphae        => 0x5c191d,
            Self::WarpedNylium         => 0x167e86,
            Self::WarpedStem           => 0x3a8e8c,
            Self::WarpedHyphae         => 0x562c3e,
            Self::WarpedWartBlock      => 0x14b485,
            Self::Deepslate            => 0x646464,
            Self::RawIron              => 0xd8af93,
            Self::GlowLichen           => 0x7fa796,
        }
    }
}

///////////////////////////////////////////////////////////////////////////////////////////////////

/// What should happen when a piston tries to push the block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PushReaction {
    /// Pushed normally.
    Normal,
    /// Block is destroyed.
    Destroy,
    /// Piston doesn't extend, block can't be pushed.
    Block,
    /// Piston head passes through the block.
    Ignore,
    /// Block can be pushed normally, but not retracted.
    PushOnly,
}
