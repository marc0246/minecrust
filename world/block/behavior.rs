use crate::material::{Color, Material};
use crate::sound;

#[derive(Debug)]
pub struct Properties {
    pub material: Material,
    pub material_color: Color,
    pub has_collision: bool,
    pub requires_correct_tool_for_drops: bool,
    pub destroy_time: f32,
    pub explosion_resistance: f32,
    pub sound_type: sound::Type,
    pub is_randomly_ticking: bool,
    pub friction: f32,
    pub speed_factor: f32,
    pub jump_factor: f32,
    pub can_occlude: bool,
    pub dynamic_shape: bool,
    pub is_air: bool,
}

impl Properties {
    const fn new(material: Material, material_color: Color) -> Self {
        Properties {
            material,
            material_color,
            has_collision: true,
            requires_correct_tool_for_drops: false,
            destroy_time: 0.0,
            explosion_resistance: 0.0,
            sound_type: sound::Type::STONE,
            is_randomly_ticking: false,
            friction: 0.6,
            speed_factor: 1.0,
            jump_factor: 1.0,
            can_occlude: true,
            dynamic_shape: false,
            is_air: false,
        }
    }

    const fn color(mut self, color: Color) -> Self {
        self.material_color = color;

        self
    }

    const fn no_collision(mut self) -> Self {
        self.has_collision = false;

        self
    }

    const fn correct_tool(mut self) -> Self {
        self.requires_correct_tool_for_drops = true;

        self
    }

    const fn instabreak(self) -> Self {
        self.strength(0.0, 0.0)
    }

    const fn strength(self, time: f32, resistance: f32) -> Self {
        self.destroy_time(time).explosion_resistance(resistance)
    }

    const fn destroy_time(mut self, time: f32) -> Self {
        self.destroy_time = time;

        self
    }

    const fn explosion_resistance(mut self, resistance: f32) -> Self {
        self.explosion_resistance = resistance;

        self
    }

    const fn sound(mut self, sound_type: sound::Type) -> Self {
        self.sound_type = sound_type;

        self
    }

    const fn no_drops(self) -> Self {
        self
    }

    // const fn drops_like<B>(self) -> Self {
    //     self
    // }

    const fn randomly_ticking(mut self) -> Self {
        self.is_randomly_ticking = true;

        self
    }

    const fn friction(mut self, friction: f32) -> Self {
        self.friction = friction;

        self
    }

    const fn speed_factor(mut self, speed_factor: f32) -> Self {
        self.speed_factor = speed_factor;

        self
    }

    const fn jump_factor(mut self, jump_factor: f32) -> Self {
        self.jump_factor = jump_factor;

        self
    }

    const fn no_occlusion(mut self) -> Self {
        self.can_occlude = false;

        self
    }

    const fn dynamic_shape(mut self) -> Self {
        self.dynamic_shape = true;

        self
    }

    const fn air(mut self) -> Self {
        self.is_air = true;

        self
    }
}

pub const AIR: Properties = Properties::new(Material::AIR, Color::None)
    .no_collision()
    .no_drops()
    .air();

pub const ACACIA_BUTTON: Properties = OAK_BUTTON;

pub const ACACIA_DOOR: Properties = OAK_DOOR.color(Color::ColorOrange);

pub const ACACIA_FENCE: Properties = ACACIA_PLANKS;

pub const ACACIA_FENCE_GATE: Properties = ACACIA_PLANKS;

pub const ACACIA_LEAVES: Properties = OAK_LEAVES;

pub const ACACIA_LOG: Properties = OAK_LOG;

pub const ACACIA_PLANKS: Properties = OAK_PLANKS.color(Color::ColorOrange);

pub const ACACIA_PRESSURE_PLATE: Properties = OAK_PRESSURE_PLATE.color(Color::ColorOrange);

pub const ACACIA_SAPLING: Properties = OAK_SAPLING;

pub const ACACIA_SIGN: Properties = OAK_SIGN.color(Color::ColorOrange);

pub const ACACIA_SLAB: Properties = ACACIA_PLANKS;

pub const ACACIA_STAIRS: Properties = ACACIA_PLANKS;

pub const ACACIA_TRAPDOOR: Properties = OAK_DOOR.color(Color::ColorOrange);

pub const ACACIA_WALL_SIGN: Properties = ACACIA_SIGN/* .drops_like::<AcaciaSign>() */;

pub const ACACIA_WOOD: Properties = OAK_WOOD.color(Color::ColorGray);

pub const ACTIVATOR_RAIL: Properties = RAIL;

pub const ALLIUM: Properties = GRASS;

pub const AMETHYST_BLOCK: Properties = Properties::new(Material::AMETHYST, Color::ColorPurple)
    .correct_tool()
    .strength(1.5, 1.5)
    .sound(sound::Type::AMETHYST);

pub const AMETHYST_CLUSTER: Properties = Properties::new(Material::AMETHYST, Color::ColorPurple)
    .strength(1.5, 1.5)
    .sound(sound::Type::AMETHYST_CLUSTER)
    .randomly_ticking()
    .no_occlusion();

pub const ANCIENT_DEBRIS: Properties = Properties::new(Material::METAL, Color::ColorBlack)
    .correct_tool()
    .strength(30.0, 1200.0)
    .sound(sound::Type::ANCIENT_DEBRIS);

pub const ANDESITE: Properties = STONE;

pub const ANDESITE_SLAB: Properties = ANDESITE;

pub const ANDESITE_STAIRS: Properties = ANDESITE;

pub const ANDESITE_WALL: Properties = ANDESITE;

pub const ANVIL: Properties = Properties::new(Material::HEAVY_METAL, Color::Metal)
    .correct_tool()
    .strength(5.0, 1200.0)
    .sound(sound::Type::ANVIL);

pub const ATTACHED_MELON_STEM: Properties = ATTACHED_PUMPKIN_STEM;

pub const ATTACHED_PUMPKIN_STEM: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WOOD);

pub const AZALEA: Properties = Properties::new(Material::PLANT, Color::Plant)
    .instabreak()
    .sound(sound::Type::AZALEA)
    .no_occlusion();

pub const AZALEA_LEAVES: Properties = OAK_LEAVES.sound(sound::Type::AZALEA_LEAVES);

pub const AZURE_BLUET: Properties = GRASS;

pub const BAMBOO: Properties = Properties::new(Material::BAMBOO, Color::Plant)
    .strength(1.0, 1.0)
    .sound(sound::Type::BAMBOO)
    .randomly_ticking()
    .no_occlusion()
    .dynamic_shape();

pub const BAMBOO_SAPLING: Properties = Properties::new(Material::BAMBOO_SAPLING, Color::Wood)
    .no_collision()
    .strength(1.0, 1.0)
    .sound(sound::Type::BAMBOO_SAPLING)
    .randomly_ticking();

pub const BARREL: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const BARRIER: Properties = Properties::new(Material::BARRIER, Color::None)
    .strength(-1.0, 3_600_000.0)
    .no_drops()
    .no_occlusion();

pub const BASALT: Properties = Properties::new(Material::STONE, Color::ColorBlack)
    .correct_tool()
    .strength(1.25, 4.2)
    .sound(sound::Type::BASALT);

pub const BEACON: Properties = Properties::new(Material::GLASS, Color::Diamond)
    .strength(3.0, 3.0)
    .no_occlusion();

pub const BEDROCK: Properties = Properties::new(Material::STONE, Color::Stone)
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const BEE_NEST: Properties = Properties::new(Material::WOOD, Color::ColorYellow)
    .strength(0.3, 0.3)
    .sound(sound::Type::WOOD);

pub const BEEHIVE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(0.6, 0.6)
    .sound(sound::Type::WOOD);

pub const BEETROOTS: Properties = CARROTS;

pub const BELL: Properties = Properties::new(Material::METAL, Color::Gold)
    .correct_tool()
    .strength(5.0, 5.0)
    .sound(sound::Type::ANVIL);

pub const BIG_DRIPLEAF: Properties = Properties::new(Material::PLANT, Color::Plant)
    .strength(0.1, 0.1)
    .sound(sound::Type::BIG_DRIPLEAF);

pub const BIG_DRIPLEAF_STEM: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .strength(0.1, 0.1)
    .sound(sound::Type::BIG_DRIPLEAF);

pub const BIRCH_BUTTON: Properties = OAK_BUTTON;

pub const BIRCH_DOOR: Properties = OAK_DOOR.color(Color::Sand);

pub const BIRCH_FENCE: Properties = BIRCH_PLANKS;

pub const BIRCH_FENCE_GATE: Properties = BIRCH_PLANKS;

pub const BIRCH_LEAVES: Properties = OAK_LEAVES;

pub const BIRCH_LOG: Properties = OAK_LOG;

pub const BIRCH_PLANKS: Properties = OAK_PLANKS.color(Color::Sand);

pub const BIRCH_PRESSURE_PLATE: Properties = OAK_PRESSURE_PLATE.color(Color::Sand);

pub const BIRCH_SAPLING: Properties = OAK_SAPLING;

pub const BIRCH_SIGN: Properties = OAK_SIGN.color(Color::Sand);

pub const BIRCH_SLAB: Properties = BIRCH_PLANKS;

pub const BIRCH_STAIRS: Properties = BIRCH_PLANKS;

pub const BIRCH_TRAPDOOR: Properties = OAK_DOOR.color(Color::Sand);

pub const BIRCH_WALL_SIGN: Properties = BIRCH_SIGN/* .drops_like::<BirchSign>() */;

pub const BIRCH_WOOD: Properties = OAK_WOOD.color(Color::Sand);

pub const BLACK_BANNER: Properties = WHITE_BANNER;

pub const BLACK_BED: Properties = WHITE_BED;

pub const BLACK_CANDLE: Properties = CANDLE.color(Color::ColorBlack);

pub const BLACK_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const BLACK_CARPET: Properties = WHITE_CARPET.color(Color::ColorBlack);

pub const BLACK_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorBlack);

pub const BLACK_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorBlack);

pub const BLACK_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorBlack);

pub const BLACK_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorBlack);

pub const BLACK_STAINED_GLASS: Properties = GLASS.color(Color::ColorBlack);

pub const BLACK_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorBlack);

pub const BLACK_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaBlack);

pub const BLACK_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<BlackBanner>() */;

pub const BLACK_WOOL: Properties = WHITE_WOOL.color(Color::ColorBlack);

pub const BLACKSTONE: Properties = Properties::new(Material::STONE, Color::ColorBlack)
    .correct_tool()
    .strength(1.5, 6.0);

pub const BLACKSTONE_SLAB: Properties = BLACKSTONE.strength(2.0, 6.0);

pub const BLACKSTONE_STAIRS: Properties = BLACKSTONE;

pub const BLACKSTONE_WALL: Properties = BLACKSTONE;

pub const BLAST_FURNACE: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const BLUE_BANNER: Properties = WHITE_BANNER;

pub const BLUE_BED: Properties = WHITE_BED;

pub const BLUE_CANDLE: Properties = CANDLE.color(Color::ColorBlue);

pub const BLUE_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const BLUE_CARPET: Properties = WHITE_CARPET.color(Color::ColorBlue);

pub const BLUE_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorBlue);

pub const BLUE_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorBlue);

pub const BLUE_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorBlue);

pub const BLUE_ICE: Properties = Properties::new(Material::ICE_SOLID, Color::Ice)
    .strength(2.8, 2.8)
    .sound(sound::Type::GLASS)
    .friction(0.989);

pub const BLUE_ORCHID: Properties = GRASS;

pub const BLUE_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorBlue);

pub const BLUE_STAINED_GLASS: Properties = GLASS.color(Color::ColorBlue);

pub const BLUE_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorBlue);

pub const BLUE_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaBlue);

pub const BLUE_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<BlueBanner>() */;

pub const BLUE_WOOL: Properties = WHITE_WOOL.color(Color::ColorBlue);

pub const BONE_BLOCK: Properties = Properties::new(Material::STONE, Color::Sand)
    .correct_tool()
    .strength(2.0, 2.0)
    .sound(sound::Type::BONE_BLOCK);

pub const BOOKSHELF: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(1.5, 1.5)
    .sound(sound::Type::WOOD);

pub const BRAIN_CORAL: Properties = TUBE_CORAL.color(Color::ColorPink);

pub const BRAIN_CORAL_BLOCK: Properties = TUBE_CORAL_BLOCK.color(Color::ColorPink);

pub const BRAIN_CORAL_FAN: Properties = BRAIN_CORAL;

pub const BRAIN_CORAL_WALL_FAN: Properties = BRAIN_CORAL_FAN/* .drops_like::<BrainCoralFan>() */;

pub const BREWING_STAND: Properties = Properties::new(Material::METAL, Color::Metal)
    .correct_tool()
    .strength(0.5, 0.5)
    .no_occlusion();

pub const BRICK_SLAB: Properties = STONE_SLAB.color(Color::ColorRed);

pub const BRICK_STAIRS: Properties = BRICKS;

pub const BRICK_WALL: Properties = BRICKS;

pub const BRICKS: Properties = Properties::new(Material::STONE, Color::ColorRed)
    .correct_tool()
    .strength(2.0, 6.0);

pub const BROWN_BANNER: Properties = WHITE_BANNER;

pub const BROWN_BED: Properties = WHITE_BED;

pub const BROWN_CANDLE: Properties = CANDLE.color(Color::ColorBrown);

pub const BROWN_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const BROWN_CARPET: Properties = WHITE_CARPET.color(Color::ColorBrown);

pub const BROWN_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorBrown);

pub const BROWN_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorBrown);

pub const BROWN_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorBrown);

pub const BROWN_MUSHROOM: Properties = Properties::new(Material::PLANT, Color::ColorBrown)
    .no_collision()
    .instabreak()
    .sound(sound::Type::GRASS)
    .randomly_ticking();

pub const BROWN_MUSHROOM_BLOCK: Properties = Properties::new(Material::WOOD, Color::Dirt)
    .strength(0.2, 0.2)
    .sound(sound::Type::WOOD);

pub const BROWN_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorBrown);

pub const BROWN_STAINED_GLASS: Properties = GLASS.color(Color::ColorBrown);

pub const BROWN_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorBrown);

pub const BROWN_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaBrown);

pub const BROWN_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<BrownBanner>() */;

pub const BROWN_WOOL: Properties = WHITE_WOOL.color(Color::ColorBrown);

pub const BUBBLE_COLUMN: Properties = Properties::new(Material::BUBBLE_COLUMN, Color::Water)
    .no_collision()
    .no_drops();

pub const BUBBLE_CORAL: Properties = TUBE_CORAL.color(Color::ColorPurple);

pub const BUBBLE_CORAL_BLOCK: Properties = TUBE_CORAL_BLOCK.color(Color::ColorPurple);

pub const BUBBLE_CORAL_FAN: Properties = BUBBLE_CORAL;

pub const BUBBLE_CORAL_WALL_FAN: Properties = BUBBLE_CORAL_FAN/* .drops_like::<BubbleCoralFan>() */;

pub const BUDDING_AMETHYST: Properties = AMETHYST_BLOCK.randomly_ticking();

pub const CACTUS: Properties = Properties::new(Material::CACTUS, Color::Plant)
    .strength(0.4, 0.4)
    .sound(sound::Type::WOOL)
    .randomly_ticking();

pub const CAKE: Properties = Properties::new(Material::CAKE, Color::None)
    .strength(0.5, 0.5)
    .sound(sound::Type::WOOL);

pub const CALCITE: Properties = Properties::new(Material::STONE, Color::TerracottaWhite)
    .correct_tool()
    .strength(0.75, 0.75)
    .sound(sound::Type::CALCITE);

pub const CAMPFIRE: Properties = Properties::new(Material::WOOD, Color::Podzol)
    .strength(2.0, 2.0)
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const CANDLE: Properties = Properties::new(Material::DECORATION, Color::Sand)
    .no_occlusion()
    .strength(0.1, 0.1)
    .sound(sound::Type::CANDLE);

pub const CANDLE_CAKE: Properties = CAKE;

pub const CARROTS: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::CROP)
    .randomly_ticking();

pub const CARTOGRAPHY_TABLE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const CARVED_PUMPKIN: Properties = PUMPKIN;

pub const CAULDRON: Properties = Properties::new(Material::METAL, Color::Stone)
    .correct_tool()
    .strength(2.0, 2.0)
    .no_occlusion();

pub const CAVE_AIR: Properties = AIR;

pub const CAVE_VINES: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::CAVE_VINES)
    .randomly_ticking();

pub const CAVE_VINES_PLANT: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::CAVE_VINES);

pub const CHAIN: Properties = Properties::new(Material::METAL, Color::None)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::CHAIN)
    .no_occlusion();

pub const CHAIN_COMMAND_BLOCK: Properties = COMMAND_BLOCK.color(Color::ColorGreen);

pub const CHEST: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const CHIPPED_ANVIL: Properties = ANVIL;

pub const CHISELED_DEEPSLATE: Properties = COBBLED_DEEPSLATE.sound(sound::Type::DEEPSLATE_BRICKS);

pub const CHISELED_NETHER_BRICKS: Properties = NETHER_BRICKS;

pub const CHISELED_POLISHED_BLACKSTONE: Properties = POLISHED_BLACKSTONE;

pub const CHISELED_QUARTZ_BLOCK: Properties = QUARTZ_BLOCK;

pub const CHISELED_RED_SANDSTONE: Properties = RED_SANDSTONE;

pub const CHISELED_SANDSTONE: Properties = SANDSTONE;

pub const CHISELED_STONE_BRICKS: Properties = STONE;

pub const CHORUS_FLOWER: Properties = Properties::new(Material::PLANT, Color::ColorPurple)
    .strength(0.4, 0.4)
    .sound(sound::Type::WOOD)
    .randomly_ticking()
    .no_occlusion();

pub const CHORUS_PLANT: Properties = Properties::new(Material::PLANT, Color::ColorPurple)
    .strength(0.4, 0.4)
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const CLAY: Properties = Properties::new(Material::CLAY, Color::Clay)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRAVEL);

pub const COAL_BLOCK: Properties = Properties::new(Material::STONE, Color::ColorBlack)
    .correct_tool()
    .strength(5.0, 6.0);

pub const COAL_ORE: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.0, 3.0);

pub const COARSE_DIRT: Properties = DIRT;

pub const COBBLED_DEEPSLATE: Properties = DEEPSLATE.strength(3.5, 6.0);

pub const COBBLED_DEEPSLATE_SLAB: Properties = COBBLED_DEEPSLATE;

pub const COBBLED_DEEPSLATE_STAIRS: Properties = COBBLED_DEEPSLATE;

pub const COBBLED_DEEPSLATE_WALL: Properties = COBBLED_DEEPSLATE;

pub const COBBLESTONE: Properties = STONE.strength(2.0, 6.0);

pub const COBBLESTONE_SLAB: Properties = STONE_SLAB;

pub const COBBLESTONE_STAIRS: Properties = COBBLESTONE;

pub const COBBLESTONE_WALL: Properties = COBBLESTONE;

pub const COBWEB: Properties = Properties::new(Material::WEB, Color::Wool)
    .no_collision()
    .correct_tool()
    .strength(4.0, 4.0);

pub const COCOA: Properties = Properties::new(Material::PLANT, Color::Plant)
    .strength(0.2, 3.0)
    .sound(sound::Type::WOOD)
    .randomly_ticking()
    .no_occlusion();

pub const COMMAND_BLOCK: Properties = Properties::new(Material::METAL, Color::ColorBrown)
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const COMPARATOR: Properties = Properties::new(Material::DECORATION, Color::None)
    .instabreak()
    .sound(sound::Type::WOOD);

pub const COMPOSTER: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(0.6, 0.6)
    .sound(sound::Type::WOOD);

pub const CONDUIT: Properties = Properties::new(Material::GLASS, Color::Diamond)
    .strength(3.0, 3.0)
    .no_occlusion();

pub const COPPER_BLOCK: Properties = Properties::new(Material::METAL, Color::ColorOrange)
    .correct_tool()
    .strength(3.0, 6.0)
    .sound(sound::Type::COPPER);

pub const COPPER_ORE: Properties = COAL_ORE;

pub const CORNFLOWER: Properties = GRASS;

pub const CRACKED_DEEPSLATE_BRICKS: Properties = DEEPSLATE_BRICKS;

pub const CRACKED_DEEPSLATE_TILES: Properties = DEEPSLATE_TILES;

pub const CRACKED_NETHER_BRICKS: Properties = NETHER_BRICKS;

pub const CRACKED_POLISHED_BLACKSTONE_BRICKS: Properties = BLACKSTONE;

pub const CRACKED_STONE_BRICKS: Properties = STONE;

pub const CRAFTING_TABLE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const CREEPER_HEAD: Properties = SKELETON_SKULL;

pub const CREEPER_WALL_HEAD: Properties = SKELETON_SKULL/* .drops_like::<CreeperHead>() */;

pub const CRIMSON_BUTTON: Properties = OAK_BUTTON;

pub const CRIMSON_DOOR: Properties = Properties::new(Material::NETHER_WOOD, Color::CrimsonStem)
    .strength(3.0, 3.0)
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const CRIMSON_FENCE: Properties = CRIMSON_PLANKS;

pub const CRIMSON_FENCE_GATE: Properties = CRIMSON_PLANKS;

pub const CRIMSON_FUNGUS: Properties = WARPED_FUNGUS;

pub const CRIMSON_HYPHAE: Properties = Properties::new(Material::NETHER_WOOD, Color::CrimsonHyphae)
    .strength(2.0, 2.0)
    .sound(sound::Type::STEM);

pub const CRIMSON_NYLIUM: Properties = Properties::new(Material::STONE, Color::CrimsonNylium)
    .correct_tool()
    .strength(0.4, 0.4)
    .sound(sound::Type::NYLIUM)
    .randomly_ticking();

pub const CRIMSON_PLANKS: Properties = Properties::new(Material::NETHER_WOOD, Color::CrimsonStem)
    .strength(2.0, 3.0)
    .sound(sound::Type::WOOD);

pub const CRIMSON_PRESSURE_PLATE: Properties =
    Properties::new(Material::NETHER_WOOD, Color::CrimsonStem)
        .no_collision()
        .strength(0.5, 0.5)
        .sound(sound::Type::WOOD);

pub const CRIMSON_ROOTS: Properties = WARPED_ROOTS;

pub const CRIMSON_SIGN: Properties = Properties::new(Material::NETHER_WOOD, Color::CrimsonStem)
    .no_collision()
    .strength(1.0, 1.0)
    .sound(sound::Type::WOOD);

pub const CRIMSON_SLAB: Properties = CRIMSON_PLANKS;

pub const CRIMSON_STAIRS: Properties = CRIMSON_PLANKS;

pub const CRIMSON_STEM: Properties = Properties::new(Material::NETHER_WOOD, Color::CrimsonStem)
    .strength(2.0, 2.0)
    .sound(sound::Type::STEM);

pub const CRIMSON_TRAPDOOR: Properties = CRIMSON_DOOR;

pub const CRIMSON_WALL_SIGN: Properties = CRIMSON_SIGN/* .drops_like::<CrimsonSign>() */;

pub const CRYING_OBSIDIAN: Properties = OBSIDIAN;

pub const CUT_COPPER: Properties = COPPER_BLOCK;

pub const CUT_COPPER_SLAB: Properties = CUT_COPPER;

pub const CUT_COPPER_STAIRS: Properties = CUT_COPPER;

pub const CUT_RED_SANDSTONE: Properties = RED_SANDSTONE;

pub const CUT_RED_SANDSTONE_SLAB: Properties = STONE_SLAB.color(Color::ColorOrange);

pub const CUT_SANDSTONE: Properties = SANDSTONE;

pub const CUT_SANDSTONE_SLAB: Properties = STONE_SLAB.color(Color::Sand);

pub const CYAN_BANNER: Properties = WHITE_BANNER;

pub const CYAN_BED: Properties = WHITE_BED;

pub const CYAN_CANDLE: Properties = CANDLE.color(Color::ColorCyan);

pub const CYAN_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const CYAN_CARPET: Properties = WHITE_CARPET.color(Color::ColorCyan);

pub const CYAN_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorCyan);

pub const CYAN_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorCyan);

pub const CYAN_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorCyan);

pub const CYAN_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorCyan);

pub const CYAN_STAINED_GLASS: Properties = GLASS.color(Color::ColorCyan);

pub const CYAN_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorCyan);

pub const CYAN_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaCyan);

pub const CYAN_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<CyanBanner>() */;

pub const CYAN_WOOL: Properties = WHITE_WOOL.color(Color::ColorCyan);

pub const DAMAGED_ANVIL: Properties = ANVIL;

pub const DANDELION: Properties = GRASS;

pub const DARK_OAK_BUTTON: Properties = OAK_BUTTON;

pub const DARK_OAK_DOOR: Properties = OAK_DOOR.color(Color::ColorBrown);

pub const DARK_OAK_FENCE: Properties = DARK_OAK_PLANKS;

pub const DARK_OAK_FENCE_GATE: Properties = DARK_OAK_PLANKS;

pub const DARK_OAK_LEAVES: Properties = OAK_LEAVES;

pub const DARK_OAK_LOG: Properties = OAK_LOG;

pub const DARK_OAK_PLANKS: Properties = OAK_PLANKS.color(Color::ColorBrown);

pub const DARK_OAK_PRESSURE_PLATE: Properties = OAK_PRESSURE_PLATE.color(Color::ColorBrown);

pub const DARK_OAK_SAPLING: Properties = OAK_SAPLING;

pub const DARK_OAK_SIGN: Properties = OAK_SIGN.color(Color::ColorBrown);

pub const DARK_OAK_SLAB: Properties = DARK_OAK_PLANKS;

pub const DARK_OAK_STAIRS: Properties = DARK_OAK_PLANKS;

pub const DARK_OAK_TRAPDOOR: Properties = OAK_DOOR.color(Color::ColorBrown);

pub const DARK_OAK_WALL_SIGN: Properties = DARK_OAK_SIGN/* .drops_like::<DarkOakSign>() */;

pub const DARK_OAK_WOOD: Properties = OAK_WOOD.color(Color::ColorBrown);

pub const DARK_PRISMARINE: Properties = PRISMARINE.color(Color::Diamond);

pub const DARK_PRISMARINE_SLAB: Properties = DARK_PRISMARINE;

pub const DARK_PRISMARINE_STAIRS: Properties = DARK_PRISMARINE;

pub const DAYLIGHT_DETECTOR: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(0.2, 0.2)
    .sound(sound::Type::WOOD);

pub const DEAD_BRAIN_CORAL: Properties = DEAD_TUBE_CORAL;

pub const DEAD_BRAIN_CORAL_BLOCK: Properties = DEAD_TUBE_CORAL_BLOCK;

pub const DEAD_BRAIN_CORAL_FAN: Properties = DEAD_BRAIN_CORAL;

pub const DEAD_BRAIN_CORAL_WALL_FAN: Properties =
    DEAD_BRAIN_CORAL_FAN/* .drops_like::<DeadBrainCoralFan>() */;

pub const DEAD_BUBBLE_CORAL: Properties = DEAD_TUBE_CORAL;

pub const DEAD_BUBBLE_CORAL_BLOCK: Properties = DEAD_TUBE_CORAL_BLOCK;

pub const DEAD_BUBBLE_CORAL_FAN: Properties = DEAD_BUBBLE_CORAL;

pub const DEAD_BUBBLE_CORAL_WALL_FAN: Properties =
    DEAD_BUBBLE_CORAL_FAN/* .drops_like::<DeadBubbleCoralFan>() */;

pub const DEAD_BUSH: Properties = GRASS.color(Color::Wood);

pub const DEAD_FIRE_CORAL: Properties = DEAD_TUBE_CORAL;

pub const DEAD_FIRE_CORAL_BLOCK: Properties = DEAD_TUBE_CORAL_BLOCK;

pub const DEAD_FIRE_CORAL_FAN: Properties = DEAD_FIRE_CORAL;

pub const DEAD_FIRE_CORAL_WALL_FAN: Properties =
    DEAD_FIRE_CORAL_FAN/* .drops_like::<DeadFireCoralFan>() */;

pub const DEAD_HORN_CORAL: Properties = DEAD_TUBE_CORAL;

pub const DEAD_HORN_CORAL_BLOCK: Properties = DEAD_TUBE_CORAL_BLOCK;

pub const DEAD_HORN_CORAL_FAN: Properties = DEAD_HORN_CORAL;

pub const DEAD_HORN_CORAL_WALL_FAN: Properties =
    DEAD_HORN_CORAL_FAN/* .drops_like::<DeadHornCoralFan>() */;

pub const DEAD_TUBE_CORAL: Properties = Properties::new(Material::STONE, Color::ColorGray)
    .no_collision()
    .correct_tool()
    .instabreak();

pub const DEAD_TUBE_CORAL_BLOCK: Properties = Properties::new(Material::STONE, Color::ColorGray)
    .correct_tool()
    .strength(1.5, 6.0);

pub const DEAD_TUBE_CORAL_FAN: Properties = DEAD_TUBE_CORAL;

pub const DEAD_TUBE_CORAL_WALL_FAN: Properties =
    DEAD_TUBE_CORAL_FAN/* .drops_like::<DeadTubeCoralFan>() */;

pub const DEEPSLATE: Properties = Properties::new(Material::STONE, Color::Deepslate)
    .correct_tool()
    .strength(3.0, 6.0)
    .sound(sound::Type::DEEPSLATE);

pub const DEEPSLATE_BRICK_SLAB: Properties = DEEPSLATE_BRICKS;

pub const DEEPSLATE_BRICK_STAIRS: Properties = DEEPSLATE_BRICKS;

pub const DEEPSLATE_BRICK_WALL: Properties = DEEPSLATE_BRICKS;

pub const DEEPSLATE_BRICKS: Properties = COBBLED_DEEPSLATE.sound(sound::Type::DEEPSLATE_BRICKS);

pub const DEEPSLATE_COAL_ORE: Properties = Properties::new(Material::STONE, Color::Deepslate)
    .correct_tool()
    .strength(4.5, 3.0)
    .sound(sound::Type::DEEPSLATE);

pub const DEEPSLATE_COPPER_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_DIAMOND_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_EMERALD_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_GOLD_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_IRON_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_LAPIS_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_REDSTONE_ORE: Properties = DEEPSLATE_COAL_ORE;

pub const DEEPSLATE_TILE_SLAB: Properties = DEEPSLATE_TILES;

pub const DEEPSLATE_TILE_STAIRS: Properties = DEEPSLATE_TILES;

pub const DEEPSLATE_TILE_WALL: Properties = DEEPSLATE_TILES;

pub const DEEPSLATE_TILES: Properties = COBBLED_DEEPSLATE.sound(sound::Type::DEEPSLATE_TILES);

pub const DETECTOR_RAIL: Properties = RAIL;

pub const DIAMOND_BLOCK: Properties = Properties::new(Material::METAL, Color::Diamond)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::METAL);

pub const DIAMOND_ORE: Properties = COAL_ORE;

pub const DIORITE: Properties = STONE.color(Color::Quartz);

pub const DIORITE_SLAB: Properties = DIORITE;

pub const DIORITE_STAIRS: Properties = DIORITE;

pub const DIORITE_WALL: Properties = DIORITE;

pub const DIRT: Properties = Properties::new(Material::DIRT, Color::Dirt)
    .strength(0.5, 0.5)
    .sound(sound::Type::GRAVEL);

pub const DIRT_PATH: Properties = Properties::new(Material::DIRT, Color::Dirt)
    .strength(0.65, 0.65)
    .sound(sound::Type::GRASS);

pub const DISPENSER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const DRAGON_EGG: Properties = Properties::new(Material::EGG, Color::ColorBlack)
    .strength(3.0, 9.0)
    .no_occlusion();

pub const DRAGON_HEAD: Properties = SKELETON_SKULL;

pub const DRAGON_WALL_HEAD: Properties = SKELETON_SKULL/* .drops_like::<DragonHead>() */;

pub const DRIED_KELP_BLOCK: Properties = Properties::new(Material::GRASS, Color::ColorGreen)
    .strength(0.5, 2.5)
    .sound(sound::Type::GRASS);

pub const DRIPSTONE_BLOCK: Properties = Properties::new(Material::STONE, Color::TerracottaBrown)
    .correct_tool()
    .strength(1.5, 1.0)
    .sound(sound::Type::DRIPSTONE_BLOCK);

pub const DROPPER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const EMERALD_BLOCK: Properties = Properties::new(Material::METAL, Color::Emerald)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::METAL);

pub const EMERALD_ORE: Properties = COAL_ORE;

pub const ENCHANTING_TABLE: Properties = Properties::new(Material::STONE, Color::ColorRed)
    .correct_tool()
    .strength(5.0, 1200.0);

pub const END_GATEWAY: Properties = Properties::new(Material::PORTAL, Color::ColorBlack)
    .no_collision()
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const END_PORTAL: Properties = Properties::new(Material::PORTAL, Color::ColorBlack)
    .no_collision()
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const END_PORTAL_FRAME: Properties = Properties::new(Material::STONE, Color::ColorGreen)
    .strength(-1.0, 3_600_000.0)
    .sound(sound::Type::GLASS)
    .no_drops();

pub const END_ROD: Properties = Properties::new(Material::DECORATION, Color::None)
    .instabreak()
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const END_STONE: Properties = Properties::new(Material::STONE, Color::Sand)
    .correct_tool()
    .strength(3.0, 9.0);

pub const END_STONE_BRICK_SLAB: Properties = END_STONE_BRICKS;

pub const END_STONE_BRICK_STAIRS: Properties = END_STONE_BRICKS;

pub const END_STONE_BRICK_WALL: Properties = END_STONE_BRICKS;

pub const END_STONE_BRICKS: Properties = END_STONE;

pub const ENDER_CHEST: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(22.5, 600.0);

pub const EXPOSED_COPPER: Properties = COPPER_BLOCK.color(Color::TerracottaLightGray);

pub const EXPOSED_CUT_COPPER: Properties = EXPOSED_COPPER;

pub const EXPOSED_CUT_COPPER_SLAB: Properties = EXPOSED_CUT_COPPER;

pub const EXPOSED_CUT_COPPER_STAIRS: Properties = EXPOSED_CUT_COPPER;

pub const FARMLAND: Properties = Properties::new(Material::DIRT, Color::Dirt)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRAVEL)
    .randomly_ticking();

pub const FERN: Properties = GRASS;

pub const FIRE: Properties = Properties::new(Material::FIRE, Color::Fire)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WOOL);

pub const FIRE_CORAL: Properties = TUBE_CORAL.color(Color::ColorRed);

pub const FIRE_CORAL_BLOCK: Properties = TUBE_CORAL_BLOCK.color(Color::ColorRed);

pub const FIRE_CORAL_FAN: Properties = FIRE_CORAL;

pub const FIRE_CORAL_WALL_FAN: Properties = FIRE_CORAL_FAN/* .drops_like::<FireCoralFan>() */;

pub const FLETCHING_TABLE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const FLOWER_POT: Properties = Properties::new(Material::DECORATION, Color::None)
    .instabreak()
    .no_occlusion();

pub const FLOWERING_AZALEA: Properties = AZALEA.sound(sound::Type::FLOWERING_AZALEA);

pub const FLOWERING_AZALEA_LEAVES: Properties = AZALEA_LEAVES;

pub const FROSTED_ICE: Properties = ICE;

pub const FURNACE: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const GILDED_BLACKSTONE: Properties = BLACKSTONE.sound(sound::Type::GILDED_BLACKSTONE);

pub const GLASS: Properties = Properties::new(Material::GLASS, Color::None)
    .strength(0.3, 0.3)
    .sound(sound::Type::GLASS)
    .no_occlusion();

pub const GLASS_PANE: Properties = GLASS;

pub const GLOW_LICHEN: Properties = Properties::new(Material::REPLACEABLE_PLANT, Color::GlowLichen)
    .no_collision()
    .strength(0.2, 0.2)
    .sound(sound::Type::GLOW_LICHEN);

pub const GLOWSTONE: Properties = Properties::new(Material::GLASS, Color::Sand)
    .strength(0.3, 0.3)
    .sound(sound::Type::GLASS);

pub const GOLD_BLOCK: Properties = Properties::new(Material::METAL, Color::Gold)
    .correct_tool()
    .strength(3.0, 6.0)
    .sound(sound::Type::METAL);

pub const GOLD_ORE: Properties = COAL_ORE;

pub const GRANITE: Properties = STONE.color(Color::Dirt);

pub const GRANITE_SLAB: Properties = GRANITE;

pub const GRANITE_STAIRS: Properties = GRANITE;

pub const GRANITE_WALL: Properties = GRANITE;

pub const GRASS: Properties = Properties::new(Material::REPLACEABLE_PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::GRASS);

pub const GRASS_BLOCK: Properties = Properties::new(Material::GRASS, Color::Grass)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRASS)
    .randomly_ticking();

pub const GRAVEL: Properties = Properties::new(Material::SAND, Color::Stone)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRAVEL);

pub const GRAY_BANNER: Properties = WHITE_BANNER;

pub const GRAY_BED: Properties = WHITE_BED;

pub const GRAY_CANDLE: Properties = CANDLE.color(Color::ColorGray);

pub const GRAY_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const GRAY_CARPET: Properties = WHITE_CARPET.color(Color::ColorGray);

pub const GRAY_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorGray);

pub const GRAY_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorGray);

pub const GRAY_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorGray);

pub const GRAY_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorGray);

pub const GRAY_STAINED_GLASS: Properties = GLASS.color(Color::ColorGray);

pub const GRAY_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorGray);

pub const GRAY_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaGray);

pub const GRAY_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<GrayBanner>() */;

pub const GRAY_WOOL: Properties = WHITE_WOOL.color(Color::ColorGray);

pub const GREEN_BANNER: Properties = WHITE_BANNER;

pub const GREEN_BED: Properties = WHITE_BED;

pub const GREEN_CANDLE: Properties = CANDLE.color(Color::ColorGreen);

pub const GREEN_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const GREEN_CARPET: Properties = WHITE_CARPET.color(Color::ColorGreen);

pub const GREEN_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorGreen);

pub const GREEN_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorGreen);

pub const GREEN_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorGreen);

pub const GREEN_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorGreen);

pub const GREEN_STAINED_GLASS: Properties = GLASS.color(Color::ColorGreen);

pub const GREEN_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorGreen);

pub const GREEN_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaGreen);

pub const GREEN_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<GreenBanner>() */;

pub const GREEN_WOOL: Properties = WHITE_WOOL.color(Color::ColorGreen);

pub const GRINDSTONE: Properties = Properties::new(Material::HEAVY_METAL, Color::Metal)
    .correct_tool()
    .strength(2.0, 6.0)
    .sound(sound::Type::STONE);

pub const HANGING_ROOTS: Properties = Properties::new(Material::REPLACEABLE_PLANT, Color::Dirt)
    .no_collision()
    .instabreak()
    .sound(sound::Type::HANGING_ROOTS);

pub const HAY_BLOCK: Properties = Properties::new(Material::GRASS, Color::ColorYellow)
    .strength(0.5, 0.5)
    .sound(sound::Type::GRASS);

pub const HEAVY_WEIGHTED_PRESSURE_PLATE: Properties = LIGHT_WEIGHTED_PRESSURE_PLATE;

pub const HONEY_BLOCK: Properties = Properties::new(Material::CLAY, Color::ColorOrange)
    .sound(sound::Type::HONEY_BLOCK)
    .speed_factor(0.4)
    .jump_factor(0.5)
    .no_occlusion();

pub const HONEYCOMB_BLOCK: Properties = Properties::new(Material::CLAY, Color::ColorOrange)
    .strength(0.6, 0.6)
    .sound(sound::Type::CORAL_BLOCK);

pub const HOPPER: Properties = Properties::new(Material::METAL, Color::Stone)
    .correct_tool()
    .strength(3.0, 4.8)
    .sound(sound::Type::METAL)
    .no_occlusion();

pub const HORN_CORAL: Properties = TUBE_CORAL.color(Color::ColorYellow);

pub const HORN_CORAL_BLOCK: Properties = TUBE_CORAL_BLOCK.color(Color::ColorYellow);

pub const HORN_CORAL_FAN: Properties = HORN_CORAL;

pub const HORN_CORAL_WALL_FAN: Properties = HORN_CORAL_FAN/* .drops_like::<HornCoralFan>() */;

pub const ICE: Properties = Properties::new(Material::ICE, Color::Ice)
    .strength(0.5, 0.5)
    .sound(sound::Type::GLASS)
    .randomly_ticking()
    .friction(0.98)
    .no_occlusion();

pub const INFESTED_CHISELED_STONE_BRICKS: Properties = INFESTED_STONE;

pub const INFESTED_COBBLESTONE: Properties = Properties::new(Material::CLAY, Color::Clay)
    .correct_tool()
    .strength(1.0, 0.75);

pub const INFESTED_CRACKED_STONE_BRICKS: Properties = INFESTED_STONE;

pub const INFESTED_DEEPSLATE: Properties =
    Properties::new(Material::CLAY, Color::Deepslate).sound(sound::Type::DEEPSLATE);

pub const INFESTED_MOSSY_STONE_BRICKS: Properties = INFESTED_STONE;

pub const INFESTED_STONE: Properties = Properties::new(Material::CLAY, Color::Clay)
    .correct_tool()
    .strength(0.75, 0.75);

pub const INFESTED_STONE_BRICKS: Properties = INFESTED_STONE;

pub const IRON_BARS: Properties = Properties::new(Material::METAL, Color::None)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::METAL)
    .no_occlusion();

pub const IRON_BLOCK: Properties = Properties::new(Material::METAL, Color::Metal)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::METAL);

pub const IRON_DOOR: Properties = Properties::new(Material::METAL, Color::Metal)
    .correct_tool()
    .strength(5.0, 5.0)
    .sound(sound::Type::METAL)
    .no_occlusion();

pub const IRON_ORE: Properties = COAL_ORE;

pub const IRON_TRAPDOOR: Properties = IRON_DOOR;

pub const JACK_O_LANTERN: Properties = PUMPKIN;

pub const JIGSAW: Properties = Properties::new(Material::METAL, Color::ColorLightGray)
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const JUKEBOX: Properties = Properties::new(Material::WOOD, Color::Dirt).strength(2.0, 6.0);

pub const JUNGLE_BUTTON: Properties = OAK_BUTTON;

pub const JUNGLE_DOOR: Properties = OAK_DOOR.color(Color::Dirt);

pub const JUNGLE_FENCE: Properties = JUNGLE_PLANKS;

pub const JUNGLE_FENCE_GATE: Properties = JUNGLE_PLANKS;

pub const JUNGLE_LEAVES: Properties = OAK_LEAVES;

pub const JUNGLE_LOG: Properties = OAK_LOG;

pub const JUNGLE_PLANKS: Properties = OAK_PLANKS.color(Color::Dirt);

pub const JUNGLE_PRESSURE_PLATE: Properties = OAK_PRESSURE_PLATE.color(Color::Dirt);

pub const JUNGLE_SAPLING: Properties = OAK_SAPLING;

pub const JUNGLE_SIGN: Properties = OAK_SIGN.color(Color::Podzol);

pub const JUNGLE_SLAB: Properties = JUNGLE_PLANKS;

pub const JUNGLE_STAIRS: Properties = JUNGLE_PLANKS;

pub const JUNGLE_TRAPDOOR: Properties = OAK_DOOR.color(Color::Dirt);

pub const JUNGLE_WALL_SIGN: Properties = JUNGLE_SIGN/* .drops_like::<JungleSign>() */;

pub const JUNGLE_WOOD: Properties = OAK_WOOD.color(Color::Dirt);

pub const KELP: Properties = KELP_PLANT.randomly_ticking();

pub const KELP_PLANT: Properties = Properties::new(Material::WATER_PLANT, Color::Water)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WET_GRASS);

pub const LADDER: Properties = Properties::new(Material::DECORATION, Color::None)
    .strength(0.4, 0.4)
    .sound(sound::Type::LADDER)
    .no_occlusion();

pub const LANTERN: Properties = Properties::new(Material::METAL, Color::Metal)
    .correct_tool()
    .strength(3.5, 3.5)
    .sound(sound::Type::LANTERN)
    .no_occlusion();

pub const LAPIS_BLOCK: Properties = Properties::new(Material::METAL, Color::Lapis)
    .correct_tool()
    .strength(3.0, 3.0);

pub const LAPIS_ORE: Properties = COAL_ORE;

pub const LARGE_AMETHYST_BUD: Properties = AMETHYST_CLUSTER.sound(sound::Type::MEDIUM_AMETHYST_BUD);

pub const LARGE_FERN: Properties = GRASS;

pub const LAVA: Properties = Properties::new(Material::LAVA, Color::Fire)
    .no_collision()
    .strength(100.0, 100.0)
    .no_drops()
    .randomly_ticking();

pub const LAVA_CAULDRON: Properties = CAULDRON;

pub const LECTERN: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const LEVER: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .strength(0.5, 0.5)
    .sound(sound::Type::WOOD);

pub const LIGHT: Properties = Properties::new(Material::AIR, Color::None)
    .strength(-1.0, 3_600_000.0)
    .no_drops()
    .no_occlusion();

pub const LIGHT_BLUE_BANNER: Properties = WHITE_BANNER;

pub const LIGHT_BLUE_BED: Properties = WHITE_BED;

pub const LIGHT_BLUE_CANDLE: Properties = CANDLE.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const LIGHT_BLUE_CARPET: Properties = WHITE_CARPET.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_CONCRETE_POWDER: Properties =
    WHITE_CONCRETE_POWDER.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_GLAZED_TERRACOTTA: Properties =
    WHITE_GLAZED_TERRACOTTA.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_STAINED_GLASS: Properties = GLASS.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorLightBlue);

pub const LIGHT_BLUE_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaLightBlue);

pub const LIGHT_BLUE_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<LightBlueBanner>() */;

pub const LIGHT_BLUE_WOOL: Properties = WHITE_WOOL.color(Color::ColorLightBlue);

pub const LIGHT_GRAY_BANNER: Properties = WHITE_BANNER;

pub const LIGHT_GRAY_BED: Properties = WHITE_BED;

pub const LIGHT_GRAY_CANDLE: Properties = CANDLE.color(Color::ColorLightGray);

pub const LIGHT_GRAY_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const LIGHT_GRAY_CARPET: Properties = WHITE_CARPET.color(Color::ColorLightGray);

pub const LIGHT_GRAY_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorLightGray);

pub const LIGHT_GRAY_CONCRETE_POWDER: Properties =
    WHITE_CONCRETE_POWDER.color(Color::ColorLightGray);

pub const LIGHT_GRAY_GLAZED_TERRACOTTA: Properties =
    WHITE_GLAZED_TERRACOTTA.color(Color::ColorLightGray);

pub const LIGHT_GRAY_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorLightGray);

pub const LIGHT_GRAY_STAINED_GLASS: Properties = GLASS.color(Color::ColorLightGray);

pub const LIGHT_GRAY_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorLightGray);

pub const LIGHT_GRAY_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaLightGray);

pub const LIGHT_GRAY_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<LightGrayBanner>() */;

pub const LIGHT_GRAY_WOOL: Properties = WHITE_WOOL.color(Color::ColorLightGray);

pub const LIGHT_WEIGHTED_PRESSURE_PLATE: Properties = Properties::new(Material::METAL, Color::Gold)
    .no_collision()
    .correct_tool()
    .strength(0.5, 0.5)
    .sound(sound::Type::WOOD);

pub const LIGHTNING_ROD: Properties = COPPER_BLOCK.no_occlusion();

pub const LILAC: Properties = GRASS;

pub const LILY_OF_THE_VALLEY: Properties = GRASS;

pub const LILY_PAD: Properties = Properties::new(Material::PLANT, Color::Plant)
    .instabreak()
    .sound(sound::Type::LILY_PAD)
    .no_occlusion();

pub const LIME_BANNER: Properties = WHITE_BANNER;

pub const LIME_BED: Properties = WHITE_BED;

pub const LIME_CANDLE: Properties = CANDLE.color(Color::ColorLightGreen);

pub const LIME_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const LIME_CARPET: Properties = WHITE_CARPET.color(Color::ColorLightGreen);

pub const LIME_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorLightGreen);

pub const LIME_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorLightGreen);

pub const LIME_GLAZED_TERRACOTTA: Properties =
    WHITE_GLAZED_TERRACOTTA.color(Color::ColorLightGreen);

pub const LIME_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorLightGreen);

pub const LIME_STAINED_GLASS: Properties = GLASS.color(Color::ColorLightGreen);

pub const LIME_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorLightGreen);

pub const LIME_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaLightGreen);

pub const LIME_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<LimeBanner>() */;

pub const LIME_WOOL: Properties = WHITE_WOOL.color(Color::ColorLightGreen);

pub const LODESTONE: Properties = Properties::new(Material::HEAVY_METAL, Color::Metal)
    .correct_tool()
    .strength(3.5, 3.5)
    .sound(sound::Type::LODESTONE);

pub const LOOM: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const MAGENTA_BANNER: Properties = WHITE_BANNER;

pub const MAGENTA_BED: Properties = WHITE_BED;

pub const MAGENTA_CANDLE: Properties = CANDLE.color(Color::ColorMagenta);

pub const MAGENTA_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const MAGENTA_CARPET: Properties = WHITE_CARPET.color(Color::ColorMagenta);

pub const MAGENTA_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorMagenta);

pub const MAGENTA_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorMagenta);

pub const MAGENTA_GLAZED_TERRACOTTA: Properties =
    WHITE_GLAZED_TERRACOTTA.color(Color::ColorMagenta);

pub const MAGENTA_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorMagenta);

pub const MAGENTA_STAINED_GLASS: Properties = GLASS.color(Color::ColorMagenta);

pub const MAGENTA_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorMagenta);

pub const MAGENTA_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaMagenta);

pub const MAGENTA_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<MagentaBanner>() */;

pub const MAGENTA_WOOL: Properties = WHITE_WOOL.color(Color::ColorMagenta);

pub const MAGMA_BLOCK: Properties = Properties::new(Material::STONE, Color::Nether)
    .correct_tool()
    .strength(0.5, 0.5)
    .randomly_ticking();

pub const MEDIUM_AMETHYST_BUD: Properties = AMETHYST_CLUSTER.sound(sound::Type::LARGE_AMETHYST_BUD);

pub const MELON: Properties = Properties::new(Material::VEGETABLE, Color::ColorLightGreen)
    .strength(1.0, 1.0)
    .sound(sound::Type::WOOD);

pub const MELON_STEM: Properties = PUMPKIN_STEM;

pub const MOSS_BLOCK: Properties = Properties::new(Material::MOSS, Color::ColorGreen)
    .strength(0.1, 0.1)
    .sound(sound::Type::MOSS);

pub const MOSS_CARPET: Properties = Properties::new(Material::PLANT, Color::ColorGreen)
    .strength(0.1, 0.1)
    .sound(sound::Type::MOSS_CARPET);

pub const MOSSY_COBBLESTONE: Properties = COBBLESTONE;

pub const MOSSY_COBBLESTONE_SLAB: Properties = MOSSY_COBBLESTONE;

pub const MOSSY_COBBLESTONE_STAIRS: Properties = MOSSY_COBBLESTONE;

pub const MOSSY_COBBLESTONE_WALL: Properties = COBBLESTONE;

pub const MOSSY_STONE_BRICK_SLAB: Properties = MOSSY_STONE_BRICKS;

pub const MOSSY_STONE_BRICK_STAIRS: Properties = MOSSY_STONE_BRICKS;

pub const MOSSY_STONE_BRICK_WALL: Properties = MOSSY_STONE_BRICKS;

pub const MOSSY_STONE_BRICKS: Properties = STONE;

pub const MOVING_PISTON: Properties = Properties::new(Material::PISTON, Color::Stone)
    .strength(-1.0, -1.0)
    .no_drops()
    .no_occlusion()
    .dynamic_shape();

pub const MUSHROOM_STEM: Properties = BROWN_MUSHROOM_BLOCK.color(Color::Wool);

pub const MYCELIUM: Properties = Properties::new(Material::GRASS, Color::ColorPurple)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRASS)
    .randomly_ticking();

pub const NETHER_BRICK_FENCE: Properties = NETHER_BRICKS;

pub const NETHER_BRICK_SLAB: Properties = STONE_SLAB.color(Color::Nether);

pub const NETHER_BRICK_STAIRS: Properties = NETHER_BRICKS;

pub const NETHER_BRICK_WALL: Properties = NETHER_BRICKS;

pub const NETHER_BRICKS: Properties = Properties::new(Material::STONE, Color::Nether)
    .correct_tool()
    .strength(2.0, 6.0)
    .sound(sound::Type::NETHER_BRICKS);

pub const NETHER_GOLD_ORE: Properties = Properties::new(Material::STONE, Color::Nether)
    .correct_tool()
    .strength(3.0, 3.0)
    .sound(sound::Type::NETHER_GOLD_ORE);

pub const NETHER_PORTAL: Properties = Properties::new(Material::PORTAL, Color::None)
    .no_collision()
    .strength(-1.0, -1.0)
    .randomly_ticking()
    .sound(sound::Type::GLASS);

pub const NETHER_QUARTZ_ORE: Properties = NETHER_GOLD_ORE.sound(sound::Type::NETHER_ORE);

pub const NETHER_SPROUTS: Properties = WARPED_ROOTS.sound(sound::Type::NETHER_SPROUTS);

pub const NETHER_WART: Properties = Properties::new(Material::PLANT, Color::ColorRed)
    .no_collision()
    .sound(sound::Type::NETHER_WART)
    .randomly_ticking();

pub const NETHER_WART_BLOCK: Properties = Properties::new(Material::GRASS, Color::ColorRed)
    .strength(1.0, 1.0)
    .sound(sound::Type::WART_BLOCK);

pub const NETHERITE_BLOCK: Properties = Properties::new(Material::METAL, Color::ColorBlack)
    .correct_tool()
    .strength(50.0, 1200.0)
    .sound(sound::Type::NETHERITE_BLOCK);

pub const NETHERRACK: Properties = Properties::new(Material::STONE, Color::Nether)
    .correct_tool()
    .strength(0.4, 0.4)
    .sound(sound::Type::NETHERRACK);

pub const NOTE_BLOCK: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(0.8, 0.8)
    .sound(sound::Type::WOOD);

pub const OAK_BUTTON: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .strength(0.5, 0.5)
    .sound(sound::Type::WOOD);

pub const OAK_DOOR: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(3.0, 3.0)
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const OAK_FENCE: Properties = OAK_PLANKS;

pub const OAK_FENCE_GATE: Properties = OAK_PLANKS;

pub const OAK_LEAVES: Properties = Properties::new(Material::LEAVES, Color::Plant)
    .strength(0.2, 0.2)
    .sound(sound::Type::GRASS)
    .randomly_ticking()
    .no_occlusion();

pub const OAK_LOG: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.0, 2.0)
    .sound(sound::Type::WOOD);

pub const OAK_PLANKS: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.0, 3.0)
    .sound(sound::Type::WOOD);

pub const OAK_PRESSURE_PLATE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .no_collision()
    .strength(0.5, 0.5)
    .sound(sound::Type::WOOD);

pub const OAK_SAPLING: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::GRASS)
    .randomly_ticking();

pub const OAK_SIGN: Properties = Properties::new(Material::WOOD, Color::Wood)
    .no_collision()
    .strength(1.0, 1.0)
    .sound(sound::Type::WOOD);

pub const OAK_SLAB: Properties = OAK_PLANKS;

pub const OAK_STAIRS: Properties = OAK_PLANKS;

pub const OAK_TRAPDOOR: Properties = OAK_DOOR;

pub const OAK_WALL_SIGN: Properties = OAK_SIGN/* .drops_like::<OakSign>() */;

pub const OAK_WOOD: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.0, 2.0)
    .sound(sound::Type::WOOD);

pub const OBSERVER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.0, 3.0);

pub const OBSIDIAN: Properties = Properties::new(Material::STONE, Color::ColorBlack)
    .correct_tool()
    .strength(50.0, 1200.0);

pub const ORANGE_BANNER: Properties = WHITE_BANNER;

pub const ORANGE_BED: Properties = WHITE_BED;

pub const ORANGE_CANDLE: Properties = CANDLE.color(Color::ColorOrange);

pub const ORANGE_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const ORANGE_CARPET: Properties = WHITE_CARPET.color(Color::ColorOrange);

pub const ORANGE_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorOrange);

pub const ORANGE_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorOrange);

pub const ORANGE_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorOrange);

pub const ORANGE_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorOrange);

pub const ORANGE_STAINED_GLASS: Properties = GLASS.color(Color::ColorOrange);

pub const ORANGE_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorOrange);

pub const ORANGE_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaOrange);

pub const ORANGE_TULIP: Properties = GRASS;

pub const ORANGE_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<OrangeBanner>() */;

pub const ORANGE_WOOL: Properties = WHITE_WOOL.color(Color::ColorOrange);

pub const OXEYE_DAISY: Properties = GRASS;

pub const OXIDIZED_COPPER: Properties = COPPER_BLOCK.color(Color::WarpedNylium);

pub const OXIDIZED_CUT_COPPER: Properties = OXIDIZED_COPPER;

pub const OXIDIZED_CUT_COPPER_SLAB: Properties = OXIDIZED_CUT_COPPER;

pub const OXIDIZED_CUT_COPPER_STAIRS: Properties = OXIDIZED_CUT_COPPER;

pub const PACKED_ICE: Properties = Properties::new(Material::ICE_SOLID, Color::Ice)
    .strength(0.5, 0.5)
    .sound(sound::Type::GLASS)
    .friction(0.98);

pub const PEONY: Properties = GRASS;

pub const PETRIFIED_OAK_SLAB: Properties = STONE_SLAB.color(Color::Wood);

pub const PINK_BANNER: Properties = WHITE_BANNER;

pub const PINK_BED: Properties = WHITE_BED;

pub const PINK_CANDLE: Properties = CANDLE.color(Color::ColorPink);

pub const PINK_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const PINK_CARPET: Properties = WHITE_CARPET.color(Color::ColorPink);

pub const PINK_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorPink);

pub const PINK_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorPink);

pub const PINK_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorPink);

pub const PINK_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorPink);

pub const PINK_STAINED_GLASS: Properties = GLASS.color(Color::ColorPink);

pub const PINK_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorPink);

pub const PINK_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaPink);

pub const PINK_TULIP: Properties = GRASS;

pub const PINK_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<PinkBanner>() */;

pub const PINK_WOOL: Properties = WHITE_WOOL.color(Color::ColorPink);

pub const PISTON: Properties = Properties::new(Material::PISTON, Color::Stone).strength(1.5, 1.5);

pub const PISTON_HEAD: Properties = PISTON.no_drops();

pub const PLAYER_HEAD: Properties = SKELETON_SKULL;

pub const PLAYER_WALL_HEAD: Properties = SKELETON_SKULL/* .drops_like::<PlayerHead>() */;

pub const PODZOL: Properties = DIRT.color(Color::Podzol);

pub const POINTED_DRIPSTONE: Properties = Properties::new(Material::STONE, Color::TerracottaBrown)
    .strength(1.5, 3.0)
    .sound(sound::Type::POINTED_DRIPSTONE)
    .randomly_ticking()
    .no_occlusion()
    .dynamic_shape();

pub const POLISHED_ANDESITE: Properties = ANDESITE;

pub const POLISHED_ANDESITE_SLAB: Properties = POLISHED_ANDESITE;

pub const POLISHED_ANDESITE_STAIRS: Properties = POLISHED_ANDESITE;

pub const POLISHED_BASALT: Properties = BASALT;

pub const POLISHED_BLACKSTONE: Properties = BLACKSTONE.strength(2.0, 6.0);

pub const POLISHED_BLACKSTONE_BRICK_SLAB: Properties = BLACKSTONE.strength(2.0, 6.0);

pub const POLISHED_BLACKSTONE_BRICK_STAIRS: Properties = BLACKSTONE;

pub const POLISHED_BLACKSTONE_BRICK_WALL: Properties = BLACKSTONE;

pub const POLISHED_BLACKSTONE_BRICKS: Properties = BLACKSTONE;

pub const POLISHED_BLACKSTONE_BUTTON: Properties = STONE_BUTTON;

pub const POLISHED_BLACKSTONE_PRESSURE_PLATE: Properties =
    STONE_PRESSURE_PLATE.color(Color::ColorBlack);

pub const POLISHED_BLACKSTONE_SLAB: Properties = POLISHED_BLACKSTONE;

pub const POLISHED_BLACKSTONE_STAIRS: Properties = POLISHED_BLACKSTONE;

pub const POLISHED_BLACKSTONE_WALL: Properties = POLISHED_BLACKSTONE;

pub const POLISHED_DEEPSLATE: Properties = COBBLED_DEEPSLATE.sound(sound::Type::POLISHED_DEEPSLATE);

pub const POLISHED_DEEPSLATE_SLAB: Properties = POLISHED_DEEPSLATE;

pub const POLISHED_DEEPSLATE_STAIRS: Properties = POLISHED_DEEPSLATE;

pub const POLISHED_DEEPSLATE_WALL: Properties = POLISHED_DEEPSLATE;

pub const POLISHED_DIORITE: Properties = DIORITE;

pub const POLISHED_DIORITE_SLAB: Properties = POLISHED_DIORITE;

pub const POLISHED_DIORITE_STAIRS: Properties = POLISHED_DIORITE;

pub const POLISHED_GRANITE: Properties = GRANITE;

pub const POLISHED_GRANITE_SLAB: Properties = POLISHED_GRANITE;

pub const POLISHED_GRANITE_STAIRS: Properties = POLISHED_GRANITE;

pub const POPPY: Properties = GRASS;

pub const POTATOES: Properties = CARROTS;

pub const POTTED_ACACIA_SAPLING: Properties = FLOWER_POT;

pub const POTTED_ALLIUM: Properties = FLOWER_POT;

pub const POTTED_AZALEA_BUSH: Properties = FLOWER_POT;

pub const POTTED_AZURE_BLUET: Properties = FLOWER_POT;

pub const POTTED_BAMBOO: Properties = FLOWER_POT;

pub const POTTED_BIRCH_SAPLING: Properties = FLOWER_POT;

pub const POTTED_BLUE_ORCHID: Properties = FLOWER_POT;

pub const POTTED_BROWN_MUSHROOM: Properties = FLOWER_POT;

pub const POTTED_CACTUS: Properties = FLOWER_POT;

pub const POTTED_CORNFLOWER: Properties = FLOWER_POT;

pub const POTTED_CRIMSON_FUNGUS: Properties = FLOWER_POT;

pub const POTTED_CRIMSON_ROOTS: Properties = FLOWER_POT;

pub const POTTED_DANDELION: Properties = FLOWER_POT;

pub const POTTED_DARK_OAK_SAPLING: Properties = FLOWER_POT;

pub const POTTED_DEAD_BUSH: Properties = FLOWER_POT;

pub const POTTED_FERN: Properties = FLOWER_POT;

pub const POTTED_FLOWERING_AZALEA_BUSH: Properties = FLOWER_POT;

pub const POTTED_JUNGLE_SAPLING: Properties = FLOWER_POT;

pub const POTTED_LILY_OF_THE_VALLEY: Properties = FLOWER_POT;

pub const POTTED_OAK_SAPLING: Properties = FLOWER_POT;

pub const POTTED_ORANGE_TULIP: Properties = FLOWER_POT;

pub const POTTED_OXEYE_DAISY: Properties = FLOWER_POT;

pub const POTTED_PINK_TULIP: Properties = FLOWER_POT;

pub const POTTED_POPPY: Properties = FLOWER_POT;

pub const POTTED_RED_MUSHROOM: Properties = FLOWER_POT;

pub const POTTED_RED_TULIP: Properties = FLOWER_POT;

pub const POTTED_SPRUCE_SAPLING: Properties = FLOWER_POT;

pub const POTTED_WARPED_FUNGUS: Properties = FLOWER_POT;

pub const POTTED_WARPED_ROOTS: Properties = FLOWER_POT;

pub const POTTED_WHITE_TULIP: Properties = FLOWER_POT;

pub const POTTED_WITHER_ROSE: Properties = FLOWER_POT;

pub const POWDER_SNOW: Properties = Properties::new(Material::POWDER_SNOW, Color::Snow)
    .strength(0.25, 0.25)
    .sound(sound::Type::POWDER_SNOW)
    .dynamic_shape();

pub const POWDER_SNOW_CAULDRON: Properties = CAULDRON;

pub const POWERED_RAIL: Properties = RAIL;

pub const PRISMARINE: Properties = Properties::new(Material::STONE, Color::ColorCyan)
    .correct_tool()
    .strength(1.5, 6.0);

pub const PRISMARINE_BRICK_SLAB: Properties = PRISMARINE_BRICKS;

pub const PRISMARINE_BRICK_STAIRS: Properties = PRISMARINE_BRICKS;

pub const PRISMARINE_BRICKS: Properties = PRISMARINE.color(Color::Diamond);

pub const PRISMARINE_SLAB: Properties = PRISMARINE;

pub const PRISMARINE_STAIRS: Properties = PRISMARINE;

pub const PRISMARINE_WALL: Properties = PRISMARINE;

pub const PUMPKIN: Properties = Properties::new(Material::VEGETABLE, Color::ColorOrange)
    .strength(1.0, 1.0)
    .sound(sound::Type::WOOD);

pub const PUMPKIN_STEM: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::HARD_CROP)
    .randomly_ticking();

pub const PURPLE_BANNER: Properties = WHITE_BANNER;

pub const PURPLE_BED: Properties = WHITE_BED;

pub const PURPLE_CANDLE: Properties = CANDLE.color(Color::ColorPurple);

pub const PURPLE_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const PURPLE_CARPET: Properties = WHITE_CARPET.color(Color::ColorPurple);

pub const PURPLE_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorPurple);

pub const PURPLE_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorPurple);

pub const PURPLE_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorPurple);

pub const PURPLE_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::TerracottaPurple);

pub const PURPLE_STAINED_GLASS: Properties = GLASS.color(Color::ColorPurple);

pub const PURPLE_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorPurple);

pub const PURPLE_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaPurple);

pub const PURPLE_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<PurpleBanner>() */;

pub const PURPLE_WOOL: Properties = WHITE_WOOL.color(Color::ColorPurple);

pub const PURPUR_BLOCK: Properties = Properties::new(Material::STONE, Color::ColorMagenta)
    .correct_tool()
    .strength(1.5, 6.0);

pub const PURPUR_PILLAR: Properties = PURPUR_BLOCK;

pub const PURPUR_SLAB: Properties = STONE_SLAB.color(Color::ColorMagenta);

pub const PURPUR_STAIRS: Properties = PURPUR_BLOCK;

pub const QUARTZ_BLOCK: Properties = SANDSTONE.color(Color::Quartz);

pub const QUARTZ_BRICKS: Properties = QUARTZ_BLOCK;

pub const QUARTZ_PILLAR: Properties = QUARTZ_BLOCK;

pub const QUARTZ_SLAB: Properties = STONE_SLAB.color(Color::Quartz);

pub const QUARTZ_STAIRS: Properties = QUARTZ_BLOCK;

pub const RAIL: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .strength(0.7, 0.7)
    .sound(sound::Type::METAL);

pub const RAW_COPPER_BLOCK: Properties = Properties::new(Material::STONE, Color::ColorOrange)
    .correct_tool()
    .strength(5.0, 6.0);

pub const RAW_GOLD_BLOCK: Properties = Properties::new(Material::STONE, Color::Gold)
    .correct_tool()
    .strength(5.0, 6.0);

pub const RAW_IRON_BLOCK: Properties = Properties::new(Material::STONE, Color::RawIron)
    .correct_tool()
    .strength(5.0, 6.0);

pub const RED_BANNER: Properties = WHITE_BANNER;

pub const RED_BED: Properties = WHITE_BED;

pub const RED_CANDLE: Properties = CANDLE.color(Color::ColorRed);

pub const RED_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const RED_CARPET: Properties = WHITE_CARPET.color(Color::ColorRed);

pub const RED_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorRed);

pub const RED_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorRed);

pub const RED_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorRed);

pub const RED_MUSHROOM: Properties = BROWN_MUSHROOM.color(Color::ColorRed);

pub const RED_MUSHROOM_BLOCK: Properties = BROWN_MUSHROOM_BLOCK.color(Color::ColorRed);

pub const RED_NETHER_BRICK_SLAB: Properties = RED_NETHER_BRICKS;

pub const RED_NETHER_BRICK_STAIRS: Properties = RED_NETHER_BRICKS;

pub const RED_NETHER_BRICK_WALL: Properties = RED_NETHER_BRICKS;

pub const RED_NETHER_BRICKS: Properties = NETHER_BRICKS;

pub const RED_SAND: Properties = SAND.color(Color::ColorOrange);

pub const RED_SANDSTONE: Properties = SANDSTONE.color(Color::ColorOrange);

pub const RED_SANDSTONE_SLAB: Properties = STONE_SLAB.color(Color::ColorOrange);

pub const RED_SANDSTONE_STAIRS: Properties = RED_SANDSTONE;

pub const RED_SANDSTONE_WALL: Properties = RED_SANDSTONE;

pub const RED_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorRed);

pub const RED_STAINED_GLASS: Properties = GLASS.color(Color::ColorRed);

pub const RED_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorRed);

pub const RED_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaRed);

pub const RED_TULIP: Properties = GRASS;

pub const RED_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<RedBanner>() */;

pub const RED_WOOL: Properties = WHITE_WOOL.color(Color::ColorRed);

pub const REDSTONE_BLOCK: Properties = Properties::new(Material::METAL, Color::Fire)
    .correct_tool()
    .strength(5.0, 6.0)
    .sound(sound::Type::METAL);

pub const REDSTONE_LAMP: Properties = Properties::new(Material::BUILDABLE_GLASS, Color::None)
    .strength(0.3, 0.3)
    .sound(sound::Type::GLASS);

pub const REDSTONE_ORE: Properties = COAL_ORE.randomly_ticking();

pub const REDSTONE_TORCH: Properties = TORCH;

pub const REDSTONE_WALL_TORCH: Properties = TORCH/* .drops_like::<RedstoneTorch>() */;

pub const REDSTONE_WIRE: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .instabreak();

pub const REPEATER: Properties = Properties::new(Material::DECORATION, Color::None)
    .instabreak()
    .sound(sound::Type::WOOD);

pub const REPEATING_COMMAND_BLOCK: Properties = COMMAND_BLOCK.color(Color::ColorPurple);

pub const RESPAWN_ANCHOR: Properties = OBSIDIAN;

pub const ROOTED_DIRT: Properties = DIRT.sound(sound::Type::ROOTED_DIRT);

pub const ROSE_BUSH: Properties = GRASS;

pub const SAND: Properties = Properties::new(Material::SAND, Color::Sand)
    .strength(0.5, 0.5)
    .sound(sound::Type::SAND);

pub const SANDSTONE: Properties = Properties::new(Material::STONE, Color::Sand)
    .correct_tool()
    .strength(0.8, 0.8);

pub const SANDSTONE_SLAB: Properties = STONE_SLAB.color(Color::Sand);

pub const SANDSTONE_STAIRS: Properties = SANDSTONE;

pub const SANDSTONE_WALL: Properties = SANDSTONE;

pub const SCAFFOLDING: Properties = Properties::new(Material::DECORATION, Color::Sand)
    .no_collision()
    .sound(sound::Type::SCAFFOLDING)
    .dynamic_shape();

pub const SCULK_SENSOR: Properties = Properties::new(Material::SCULK, Color::ColorCyan)
    .strength(1.5, 1.5)
    .sound(sound::Type::SCULK_SENSOR);

pub const SEA_LANTERN: Properties = Properties::new(Material::GLASS, Color::Quartz)
    .strength(0.3, 0.3)
    .sound(sound::Type::GLASS);

pub const SEA_PICKLE: Properties = Properties::new(Material::WATER_PLANT, Color::ColorGreen)
    .sound(sound::Type::SLIME_BLOCK)
    .no_occlusion();

pub const SEAGRASS: Properties = Properties::new(Material::REPLACEABLE_WATER_PLANT, Color::Water)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WET_GRASS);

pub const SHROOMLIGHT: Properties = Properties::new(Material::GRASS, Color::ColorRed)
    .strength(1.0, 1.0)
    .sound(sound::Type::SHROOMLIGHT);

pub const SHULKER_BOX: Properties = Properties::new(Material::SHULKER_SHELL, Color::ColorPurple)
    .strength(2.0, 2.0)
    .no_occlusion()
    .dynamic_shape();

pub const SKELETON_SKULL: Properties =
    Properties::new(Material::DECORATION, Color::None).strength(1.0, 1.0);

pub const SKELETON_WALL_SKULL: Properties = SKELETON_SKULL/* .drops_like::<SkeletonSkull>() */;

pub const SLIME_BLOCK: Properties = Properties::new(Material::CLAY, Color::Grass)
    .sound(sound::Type::SLIME_BLOCK)
    .friction(0.8)
    .no_occlusion();

pub const SMALL_AMETHYST_BUD: Properties = AMETHYST_CLUSTER.sound(sound::Type::SMALL_AMETHYST_BUD);

pub const SMALL_DRIPLEAF: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::SMALL_DRIPLEAF);

pub const SMITHING_TABLE: Properties = Properties::new(Material::WOOD, Color::Wood)
    .strength(2.5, 2.5)
    .sound(sound::Type::WOOD);

pub const SMOKER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const SMOOTH_BASALT: Properties = BASALT;

pub const SMOOTH_QUARTZ: Properties = SMOOTH_STONE.color(Color::Quartz);

pub const SMOOTH_QUARTZ_SLAB: Properties = SMOOTH_QUARTZ;

pub const SMOOTH_QUARTZ_STAIRS: Properties = SMOOTH_QUARTZ;

pub const SMOOTH_RED_SANDSTONE: Properties = SMOOTH_STONE.color(Color::ColorOrange);

pub const SMOOTH_RED_SANDSTONE_SLAB: Properties = SMOOTH_RED_SANDSTONE;

pub const SMOOTH_RED_SANDSTONE_STAIRS: Properties = SMOOTH_RED_SANDSTONE;

pub const SMOOTH_SANDSTONE: Properties = SMOOTH_STONE.color(Color::Sand);

pub const SMOOTH_SANDSTONE_SLAB: Properties = SMOOTH_SANDSTONE;

pub const SMOOTH_SANDSTONE_STAIRS: Properties = SMOOTH_SANDSTONE;

pub const SMOOTH_STONE: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(2.0, 6.0);

pub const SMOOTH_STONE_SLAB: Properties = STONE_SLAB;

pub const SNOW: Properties = Properties::new(Material::TOP_SNOW, Color::Snow)
    .correct_tool()
    .strength(0.1, 0.1)
    .sound(sound::Type::SNOW)
    .randomly_ticking();

pub const SNOW_BLOCK: Properties = Properties::new(Material::SNOW, Color::Snow)
    .correct_tool()
    .strength(0.2, 0.2)
    .sound(sound::Type::SNOW);

pub const SOUL_CAMPFIRE: Properties = CAMPFIRE;

pub const SOUL_FIRE: Properties = FIRE.color(Color::ColorLightBlue);

pub const SOUL_LANTERN: Properties = LANTERN;

pub const SOUL_SAND: Properties = Properties::new(Material::SAND, Color::ColorBrown)
    .strength(0.5, 0.5)
    .sound(sound::Type::SOUL_SAND)
    .speed_factor(0.4);

pub const SOUL_SOIL: Properties = Properties::new(Material::DIRT, Color::ColorBrown)
    .strength(0.5, 0.5)
    .sound(sound::Type::SOUL_SOIL);

pub const SOUL_TORCH: Properties = TORCH;

pub const SOUL_WALL_TORCH: Properties = SOUL_TORCH/* .drops_like::<SoulTorch>() */;

pub const SPAWNER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(5.0, 5.0)
    .sound(sound::Type::METAL)
    .no_occlusion();

pub const SPONGE: Properties = Properties::new(Material::SPONGE, Color::ColorYellow)
    .strength(0.6, 0.6)
    .sound(sound::Type::GRASS);

pub const SPORE_BLOSSOM: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::SPORE_BLOSSOM);

pub const SPRUCE_BUTTON: Properties = OAK_BUTTON;

pub const SPRUCE_DOOR: Properties = OAK_DOOR.color(Color::Podzol);

pub const SPRUCE_FENCE: Properties = SPRUCE_PLANKS;

pub const SPRUCE_FENCE_GATE: Properties = SPRUCE_PLANKS;

pub const SPRUCE_LEAVES: Properties = OAK_LEAVES;

pub const SPRUCE_LOG: Properties = OAK_LOG;

pub const SPRUCE_PLANKS: Properties = OAK_PLANKS.color(Color::Podzol);

pub const SPRUCE_PRESSURE_PLATE: Properties = OAK_PRESSURE_PLATE.color(Color::Podzol);

pub const SPRUCE_SAPLING: Properties = OAK_SAPLING;

pub const SPRUCE_SIGN: Properties = OAK_SIGN.color(Color::ColorBrown);

pub const SPRUCE_SLAB: Properties = SPRUCE_PLANKS;

pub const SPRUCE_STAIRS: Properties = SPRUCE_PLANKS;

pub const SPRUCE_TRAPDOOR: Properties = OAK_DOOR.color(Color::Podzol);

pub const SPRUCE_WALL_SIGN: Properties = SPRUCE_SIGN/* .drops_like::<SpruceSign>() */;

pub const SPRUCE_WOOD: Properties = OAK_WOOD.color(Color::Podzol);

pub const STICKY_PISTON: Properties = PISTON;

pub const STONE: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(1.5, 6.0);

pub const STONE_BRICK_SLAB: Properties = STONE_SLAB;

pub const STONE_BRICK_STAIRS: Properties = STONE_BRICKS;

pub const STONE_BRICK_WALL: Properties = STONE_BRICKS;

pub const STONE_BRICKS: Properties = STONE;

pub const STONE_BUTTON: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .strength(0.5, 0.5);

pub const STONE_PRESSURE_PLATE: Properties = Properties::new(Material::STONE, Color::Stone)
    .no_collision()
    .correct_tool()
    .strength(0.5, 0.5);

pub const STONE_SLAB: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(2.0, 6.0);

pub const STONE_STAIRS: Properties = STONE;

pub const STONECUTTER: Properties = Properties::new(Material::STONE, Color::Stone)
    .correct_tool()
    .strength(3.5, 3.5);

pub const STRIPPED_ACACIA_LOG: Properties = OAK_LOG;

pub const STRIPPED_ACACIA_WOOD: Properties = OAK_WOOD.color(Color::ColorOrange);

pub const STRIPPED_BIRCH_LOG: Properties = OAK_LOG;

pub const STRIPPED_BIRCH_WOOD: Properties = OAK_WOOD.color(Color::Sand);

pub const STRIPPED_CRIMSON_HYPHAE: Properties = CRIMSON_HYPHAE;

pub const STRIPPED_CRIMSON_STEM: Properties = CRIMSON_STEM;

pub const STRIPPED_DARK_OAK_LOG: Properties = OAK_LOG;

pub const STRIPPED_DARK_OAK_WOOD: Properties = OAK_WOOD.color(Color::ColorBrown);

pub const STRIPPED_JUNGLE_LOG: Properties = OAK_LOG;

pub const STRIPPED_JUNGLE_WOOD: Properties = OAK_WOOD.color(Color::Dirt);

pub const STRIPPED_OAK_LOG: Properties = OAK_LOG;

pub const STRIPPED_OAK_WOOD: Properties = OAK_WOOD;

pub const STRIPPED_SPRUCE_LOG: Properties = OAK_LOG;

pub const STRIPPED_SPRUCE_WOOD: Properties = OAK_WOOD.color(Color::Podzol);

pub const STRIPPED_WARPED_HYPHAE: Properties = WARPED_HYPHAE;

pub const STRIPPED_WARPED_STEM: Properties = WARPED_STEM;

pub const STRUCTURE_BLOCK: Properties = Properties::new(Material::METAL, Color::ColorLightGray)
    .strength(-1.0, 3_600_000.0)
    .no_drops();

pub const STRUCTURE_VOID: Properties = Properties::new(Material::STRUCTURAL_AIR, Color::None)
    .no_collision()
    .no_drops();

pub const SUGAR_CANE: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::GRASS)
    .randomly_ticking();

pub const SUNFLOWER: Properties = GRASS;

pub const SWEET_BERRY_BUSH: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .sound(sound::Type::SWEET_BERRY_BUSH)
    .randomly_ticking();

pub const TALL_GRASS: Properties = GRASS;

pub const TALL_SEAGRASS: Properties = SEAGRASS;

pub const TARGET: Properties = Properties::new(Material::GRASS, Color::Quartz)
    .strength(0.5, 0.5)
    .sound(sound::Type::GRASS);

pub const TERRACOTTA: Properties = Properties::new(Material::STONE, Color::ColorOrange)
    .correct_tool()
    .strength(1.25, 4.2);

pub const TINTED_GLASS: Properties = GLASS.color(Color::ColorGray);

pub const TNT: Properties = Properties::new(Material::EXPLOSIVE, Color::Fire)
    .instabreak()
    .sound(sound::Type::GRASS);

pub const TORCH: Properties = Properties::new(Material::DECORATION, Color::None)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WOOD);

pub const TRAPPED_CHEST: Properties = CHEST;

pub const TRIPWIRE: Properties = Properties::new(Material::DECORATION, Color::None).no_collision();

pub const TRIPWIRE_HOOK: Properties =
    Properties::new(Material::DECORATION, Color::None).no_collision();

pub const TUBE_CORAL: Properties = Properties::new(Material::WATER_PLANT, Color::ColorBlue)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WET_GRASS);

pub const TUBE_CORAL_BLOCK: Properties = Properties::new(Material::STONE, Color::ColorBlue)
    .correct_tool()
    .strength(1.5, 6.0)
    .sound(sound::Type::CORAL_BLOCK);

pub const TUBE_CORAL_FAN: Properties = TUBE_CORAL;

pub const TUBE_CORAL_WALL_FAN: Properties = TUBE_CORAL_FAN/* .drops_like::<TubeCoralFan>() */;

pub const TUFF: Properties = Properties::new(Material::STONE, Color::TerracottaGray)
    .correct_tool()
    .strength(1.5, 6.0)
    .sound(sound::Type::TUFF);

pub const TURTLE_EGG: Properties = Properties::new(Material::EGG, Color::Sand)
    .strength(0.5, 0.5)
    .sound(sound::Type::METAL)
    .randomly_ticking()
    .no_occlusion();

pub const TWISTING_VINES: Properties = TWISTING_VINES_PLANT;

pub const TWISTING_VINES_PLANT: Properties = Properties::new(Material::PLANT, Color::ColorCyan)
    .no_collision()
    .instabreak()
    .sound(sound::Type::TWISTING_VINES);

pub const VINE: Properties = Properties::new(Material::REPLACEABLE_PLANT, Color::Plant)
    .no_collision()
    .strength(0.2, 0.2)
    .randomly_ticking()
    .sound(sound::Type::VINE);

pub const VOID_AIR: Properties = AIR;

pub const WALL_TORCH: Properties = TORCH/* .drops_like::<Torch>() */;

pub const WARPED_BUTTON: Properties = OAK_BUTTON;

pub const WARPED_DOOR: Properties = CRIMSON_DOOR.color(Color::WarpedStem);

pub const WARPED_FENCE: Properties = WARPED_PLANKS;

pub const WARPED_FENCE_GATE: Properties = WARPED_PLANKS;

pub const WARPED_FUNGUS: Properties = Properties::new(Material::PLANT, Color::ColorCyan)
    .no_collision()
    .instabreak()
    .sound(sound::Type::FUNGUS);

pub const WARPED_HYPHAE: Properties = CRIMSON_HYPHAE.color(Color::WarpedHyphae);

pub const WARPED_NYLIUM: Properties = CRIMSON_NYLIUM.color(Color::WarpedNylium);

pub const WARPED_PLANKS: Properties = CRIMSON_PLANKS.color(Color::WarpedStem);

pub const WARPED_PRESSURE_PLATE: Properties = CRIMSON_PRESSURE_PLATE.color(Color::WarpedStem);

pub const WARPED_ROOTS: Properties =
    Properties::new(Material::REPLACEABLE_FIREPROOF_PLANT, Color::ColorCyan)
        .no_collision()
        .instabreak()
        .sound(sound::Type::ROOTS);

pub const WARPED_SIGN: Properties = CRIMSON_SIGN.color(Color::WarpedStem);

pub const WARPED_SLAB: Properties = WARPED_PLANKS;

pub const WARPED_STAIRS: Properties = WARPED_PLANKS;

pub const WARPED_STEM: Properties = CRIMSON_STEM.color(Color::WarpedStem);

pub const WARPED_TRAPDOOR: Properties = WARPED_DOOR;

pub const WARPED_WALL_SIGN: Properties = WARPED_SIGN/* .drops_like::<WarpedSign>() */;

pub const WARPED_WART_BLOCK: Properties = Properties::new(Material::GRASS, Color::WarpedWartBlock)
    .strength(1.0, 1.0)
    .sound(sound::Type::WART_BLOCK);

pub const WATER: Properties = Properties::new(Material::WATER, Color::Water)
    .no_collision()
    .strength(100.0, 100.0)
    .no_drops();

pub const WATER_CAULDRON: Properties = CAULDRON;

pub const WAXED_COPPER_BLOCK: Properties = COPPER_BLOCK;

pub const WAXED_CUT_COPPER: Properties = CUT_COPPER;

pub const WAXED_CUT_COPPER_SLAB: Properties = CUT_COPPER;

pub const WAXED_CUT_COPPER_STAIRS: Properties = CUT_COPPER;

pub const WAXED_EXPOSED_COPPER: Properties = EXPOSED_COPPER;

pub const WAXED_EXPOSED_CUT_COPPER: Properties = EXPOSED_CUT_COPPER;

pub const WAXED_EXPOSED_CUT_COPPER_SLAB: Properties = EXPOSED_CUT_COPPER;

pub const WAXED_EXPOSED_CUT_COPPER_STAIRS: Properties = EXPOSED_CUT_COPPER;

pub const WAXED_OXIDIZED_COPPER: Properties = OXIDIZED_COPPER;

pub const WAXED_OXIDIZED_CUT_COPPER: Properties = OXIDIZED_CUT_COPPER;

pub const WAXED_OXIDIZED_CUT_COPPER_SLAB: Properties = OXIDIZED_CUT_COPPER;

pub const WAXED_OXIDIZED_CUT_COPPER_STAIRS: Properties = OXIDIZED_CUT_COPPER;

pub const WAXED_WEATHERED_COPPER: Properties = WEATHERED_COPPER;

pub const WAXED_WEATHERED_CUT_COPPER: Properties = WEATHERED_CUT_COPPER;

pub const WAXED_WEATHERED_CUT_COPPER_SLAB: Properties = WEATHERED_CUT_COPPER;

pub const WAXED_WEATHERED_CUT_COPPER_STAIRS: Properties = WEATHERED_CUT_COPPER;

pub const WEATHERED_COPPER: Properties = COPPER_BLOCK.color(Color::WarpedStem);

pub const WEATHERED_CUT_COPPER: Properties = WEATHERED_COPPER;

pub const WEATHERED_CUT_COPPER_SLAB: Properties = WEATHERED_CUT_COPPER;

pub const WEATHERED_CUT_COPPER_STAIRS: Properties = WEATHERED_CUT_COPPER;

pub const WEEPING_VINES: Properties = WEEPING_VINES_PLANT.randomly_ticking();

pub const WEEPING_VINES_PLANT: Properties = Properties::new(Material::PLANT, Color::Nether)
    .no_collision()
    .instabreak()
    .sound(sound::Type::WEEPING_VINES);

pub const WET_SPONGE: Properties = SPONGE;

pub const WHEAT: Properties = Properties::new(Material::PLANT, Color::Plant)
    .no_collision()
    .instabreak()
    .sound(sound::Type::CROP)
    .randomly_ticking();

pub const WHITE_BANNER: Properties = Properties::new(Material::WOOD, Color::Wood)
    .no_collision()
    .strength(1.0, 1.0)
    .sound(sound::Type::WOOD);

pub const WHITE_BED: Properties = Properties::new(Material::WOOL, Color::Wool)
    .strength(0.2, 0.2)
    .sound(sound::Type::WOOD)
    .no_occlusion();

pub const WHITE_CANDLE: Properties = CANDLE.color(Color::Wool);

pub const WHITE_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const WHITE_CARPET: Properties = Properties::new(Material::CLOTH_DECORATION, Color::Snow)
    .strength(0.1, 0.1)
    .sound(sound::Type::WOOL);

pub const WHITE_CONCRETE: Properties = Properties::new(Material::STONE, Color::Snow)
    .correct_tool()
    .strength(1.8, 1.8);

pub const WHITE_CONCRETE_POWDER: Properties = Properties::new(Material::SAND, Color::Snow)
    .strength(0.5, 0.5)
    .sound(sound::Type::SAND);

pub const WHITE_GLAZED_TERRACOTTA: Properties = Properties::new(Material::STONE, Color::Snow)
    .correct_tool()
    .strength(1.4, 1.4);

pub const WHITE_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::Snow);

pub const WHITE_STAINED_GLASS: Properties = GLASS.color(Color::Snow);

pub const WHITE_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::Snow);

pub const WHITE_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaWhite);

pub const WHITE_TULIP: Properties = GRASS;

pub const WHITE_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<WhiteBanner>() */;

pub const WHITE_WOOL: Properties = Properties::new(Material::WOOL, Color::Snow)
    .strength(0.8, 0.8)
    .sound(sound::Type::WOOL);

pub const WITHER_ROSE: Properties = GRASS;

pub const WITHER_SKELETON_SKULL: Properties = SKELETON_SKULL;

pub const WITHER_SKELETON_WALL_SKULL: Properties =
    SKELETON_SKULL/* .drops_like::<WitherSkeletonSkull>() */;

pub const YELLOW_BANNER: Properties = WHITE_BANNER;

pub const YELLOW_BED: Properties = WHITE_BED;

pub const YELLOW_CANDLE: Properties = CANDLE.color(Color::ColorYellow);

pub const YELLOW_CANDLE_CAKE: Properties = CANDLE_CAKE;

pub const YELLOW_CARPET: Properties = WHITE_CARPET.color(Color::ColorYellow);

pub const YELLOW_CONCRETE: Properties = WHITE_CONCRETE.color(Color::ColorYellow);

pub const YELLOW_CONCRETE_POWDER: Properties = WHITE_CONCRETE_POWDER.color(Color::ColorYellow);

pub const YELLOW_GLAZED_TERRACOTTA: Properties = WHITE_GLAZED_TERRACOTTA.color(Color::ColorYellow);

pub const YELLOW_SHULKER_BOX: Properties = SHULKER_BOX.color(Color::ColorYellow);

pub const YELLOW_STAINED_GLASS: Properties = GLASS.color(Color::ColorYellow);

pub const YELLOW_STAINED_GLASS_PANE: Properties = GLASS_PANE.color(Color::ColorYellow);

pub const YELLOW_TERRACOTTA: Properties = TERRACOTTA.color(Color::TerracottaYellow);

pub const YELLOW_WALL_BANNER: Properties = WHITE_BANNER/* .drops_like::<YellowBanner>() */;

pub const YELLOW_WOOL: Properties = WHITE_WOOL.color(Color::ColorYellow);

pub const ZOMBIE_HEAD: Properties = SKELETON_SKULL;

pub const ZOMBIE_WALL_HEAD: Properties = SKELETON_SKULL/* .drops_like::<ZombieHead>() */;
