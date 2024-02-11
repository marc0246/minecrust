#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::IntoStaticStr)]
pub enum Event {
    #[strum(serialize = "ambient.cave")]
    AmbientCave,
    #[strum(serialize = "ambient.basalt_deltas.additions")]
    AmbientBasaltDeltasAdditions,
    #[strum(serialize = "ambient.basalt_deltas.loop")]
    AmbientBasaltDeltasLoop,
    #[strum(serialize = "ambient.basalt_deltas.mood")]
    AmbientBasaltDeltasMood,
    #[strum(serialize = "ambient.crimson_forest.additions")]
    AmbientCrimsonForestAdditions,
    #[strum(serialize = "ambient.crimson_forest.loop")]
    AmbientCrimsonForestLoop,
    #[strum(serialize = "ambient.crimson_forest.mood")]
    AmbientCrimsonForestMood,
    #[strum(serialize = "ambient.nether_wastes.additions")]
    AmbientNetherWastesAdditions,
    #[strum(serialize = "ambient.nether_wastes.loop")]
    AmbientNetherWastesLoop,
    #[strum(serialize = "ambient.nether_wastes.mood")]
    AmbientNetherWastesMood,
    #[strum(serialize = "ambient.soul_sand_valley.additions")]
    AmbientSoulSandValleyAdditions,
    #[strum(serialize = "ambient.soul_sand_valley.loop")]
    AmbientSoulSandValleyLoop,
    #[strum(serialize = "ambient.soul_sand_valley.mood")]
    AmbientSoulSandValleyMood,
    #[strum(serialize = "ambient.warped_forest.additions")]
    AmbientWarpedForestAdditions,
    #[strum(serialize = "ambient.warped_forest.loop")]
    AmbientWarpedForestLoop,
    #[strum(serialize = "ambient.warped_forest.mood")]
    AmbientWarpedForestMood,
    #[strum(serialize = "ambient.underwater.enter")]
    AmbientUnderwaterEnter,
    #[strum(serialize = "ambient.underwater.exit")]
    AmbientUnderwaterExit,
    #[strum(serialize = "ambient.underwater.loop")]
    AmbientUnderwaterLoop,
    #[strum(serialize = "ambient.underwater.loop.additions")]
    AmbientUnderwaterLoopAdditions,
    #[strum(serialize = "ambient.underwater.loop.additions.rare")]
    AmbientUnderwaterLoopAdditionsRare,
    #[strum(serialize = "ambient.underwater.loop.additions.ultra_rare")]
    AmbientUnderwaterLoopAdditionsUltraRare,
    #[strum(serialize = "block.amethyst_block.break")]
    AmethystBlockBreak,
    #[strum(serialize = "block.amethyst_block.chime")]
    AmethystBlockChime,
    #[strum(serialize = "block.amethyst_block.fall")]
    AmethystBlockFall,
    #[strum(serialize = "block.amethyst_block.hit")]
    AmethystBlockHit,
    #[strum(serialize = "block.amethyst_block.place")]
    AmethystBlockPlace,
    #[strum(serialize = "block.amethyst_block.step")]
    AmethystBlockStep,
    #[strum(serialize = "block.amethyst_cluster.break")]
    AmethystClusterBreak,
    #[strum(serialize = "block.amethyst_cluster.fall")]
    AmethystClusterFall,
    #[strum(serialize = "block.amethyst_cluster.hit")]
    AmethystClusterHit,
    #[strum(serialize = "block.amethyst_cluster.place")]
    AmethystClusterPlace,
    #[strum(serialize = "block.amethyst_cluster.step")]
    AmethystClusterStep,
    #[strum(serialize = "block.ancient_debris.break")]
    AncientDebrisBreak,
    #[strum(serialize = "block.ancient_debris.step")]
    AncientDebrisStep,
    #[strum(serialize = "block.ancient_debris.place")]
    AncientDebrisPlace,
    #[strum(serialize = "block.ancient_debris.hit")]
    AncientDebrisHit,
    #[strum(serialize = "block.ancient_debris.fall")]
    AncientDebrisFall,
    #[strum(serialize = "block.anvil.break")]
    AnvilBreak,
    #[strum(serialize = "block.anvil.destroy")]
    AnvilDestroy,
    #[strum(serialize = "block.anvil.fall")]
    AnvilFall,
    #[strum(serialize = "block.anvil.hit")]
    AnvilHit,
    #[strum(serialize = "block.anvil.land")]
    AnvilLand,
    #[strum(serialize = "block.anvil.place")]
    AnvilPlace,
    #[strum(serialize = "block.anvil.step")]
    AnvilStep,
    #[strum(serialize = "block.anvil.use")]
    AnvilUse,
    #[strum(serialize = "item.armor.equip_chain")]
    ArmorEquipChain,
    #[strum(serialize = "item.armor.equip_diamond")]
    ArmorEquipDiamond,
    #[strum(serialize = "item.armor.equip_elytra")]
    ArmorEquipElytra,
    #[strum(serialize = "item.armor.equip_generic")]
    ArmorEquipGeneric,
    #[strum(serialize = "item.armor.equip_gold")]
    ArmorEquipGold,
    #[strum(serialize = "item.armor.equip_iron")]
    ArmorEquipIron,
    #[strum(serialize = "item.armor.equip_leather")]
    ArmorEquipLeather,
    #[strum(serialize = "item.armor.equip_netherite")]
    ArmorEquipNetherite,
    #[strum(serialize = "item.armor.equip_turtle")]
    ArmorEquipTurtle,
    #[strum(serialize = "entity.armor_stand.break")]
    ArmorStandBreak,
    #[strum(serialize = "entity.armor_stand.fall")]
    ArmorStandFall,
    #[strum(serialize = "entity.armor_stand.hit")]
    ArmorStandHit,
    #[strum(serialize = "entity.armor_stand.place")]
    ArmorStandPlace,
    #[strum(serialize = "entity.arrow.hit")]
    ArrowHit,
    #[strum(serialize = "entity.arrow.hit_player")]
    ArrowHitPlayer,
    #[strum(serialize = "entity.arrow.shoot")]
    ArrowShoot,
    #[strum(serialize = "item.axe.strip")]
    AxeStrip,
    #[strum(serialize = "item.axe.scrape")]
    AxeScrape,
    #[strum(serialize = "item.axe.wax_off")]
    AxeWaxOff,
    #[strum(serialize = "entity.axolotl.attack")]
    AxolotlAttack,
    #[strum(serialize = "entity.axolotl.death")]
    AxolotlDeath,
    #[strum(serialize = "entity.axolotl.hurt")]
    AxolotlHurt,
    #[strum(serialize = "entity.axolotl.idle_air")]
    AxolotlIdleAir,
    #[strum(serialize = "entity.axolotl.idle_water")]
    AxolotlIdleWater,
    #[strum(serialize = "entity.axolotl.splash")]
    AxolotlSplash,
    #[strum(serialize = "entity.axolotl.swim")]
    AxolotlSwim,
    #[strum(serialize = "block.azalea.break")]
    AzaleaBreak,
    #[strum(serialize = "block.azalea.fall")]
    AzaleaFall,
    #[strum(serialize = "block.azalea.hit")]
    AzaleaHit,
    #[strum(serialize = "block.azalea.place")]
    AzaleaPlace,
    #[strum(serialize = "block.azalea.step")]
    AzaleaStep,
    #[strum(serialize = "block.azalea_leaves.break")]
    AzaleaLeavesBreak,
    #[strum(serialize = "block.azalea_leaves.fall")]
    AzaleaLeavesFall,
    #[strum(serialize = "block.azalea_leaves.hit")]
    AzaleaLeavesHit,
    #[strum(serialize = "block.azalea_leaves.place")]
    AzaleaLeavesPlace,
    #[strum(serialize = "block.azalea_leaves.step")]
    AzaleaLeavesStep,
    #[strum(serialize = "block.bamboo.break")]
    BambooBreak,
    #[strum(serialize = "block.bamboo.fall")]
    BambooFall,
    #[strum(serialize = "block.bamboo.hit")]
    BambooHit,
    #[strum(serialize = "block.bamboo.place")]
    BambooPlace,
    #[strum(serialize = "block.bamboo.step")]
    BambooStep,
    #[strum(serialize = "block.bamboo_sapling.break")]
    BambooSaplingBreak,
    #[strum(serialize = "block.bamboo_sapling.hit")]
    BambooSaplingHit,
    #[strum(serialize = "block.bamboo_sapling.place")]
    BambooSaplingPlace,
    #[strum(serialize = "block.barrel.close")]
    BarrelClose,
    #[strum(serialize = "block.barrel.open")]
    BarrelOpen,
    #[strum(serialize = "block.basalt.break")]
    BasaltBreak,
    #[strum(serialize = "block.basalt.step")]
    BasaltStep,
    #[strum(serialize = "block.basalt.place")]
    BasaltPlace,
    #[strum(serialize = "block.basalt.hit")]
    BasaltHit,
    #[strum(serialize = "block.basalt.fall")]
    BasaltFall,
    #[strum(serialize = "entity.bat.ambient")]
    BatAmbient,
    #[strum(serialize = "entity.bat.death")]
    BatDeath,
    #[strum(serialize = "entity.bat.hurt")]
    BatHurt,
    #[strum(serialize = "entity.bat.loop")]
    BatLoop,
    #[strum(serialize = "entity.bat.takeoff")]
    BatTakeoff,
    #[strum(serialize = "block.beacon.activate")]
    BeaconActivate,
    #[strum(serialize = "block.beacon.ambient")]
    BeaconAmbient,
    #[strum(serialize = "block.beacon.deactivate")]
    BeaconDeactivate,
    #[strum(serialize = "block.beacon.power_select")]
    BeaconPowerSelect,
    #[strum(serialize = "entity.bee.death")]
    BeeDeath,
    #[strum(serialize = "entity.bee.hurt")]
    BeeHurt,
    #[strum(serialize = "entity.bee.loop_aggressive")]
    BeeLoopAggressive,
    #[strum(serialize = "entity.bee.loop")]
    BeeLoop,
    #[strum(serialize = "entity.bee.sting")]
    BeeSting,
    #[strum(serialize = "entity.bee.pollinate")]
    BeePollinate,
    #[strum(serialize = "block.beehive.drip")]
    BeehiveDrip,
    #[strum(serialize = "block.beehive.enter")]
    BeehiveEnter,
    #[strum(serialize = "block.beehive.exit")]
    BeehiveExit,
    #[strum(serialize = "block.beehive.shear")]
    BeehiveShear,
    #[strum(serialize = "block.beehive.work")]
    BeehiveWork,
    #[strum(serialize = "block.bell.use")]
    BellBlock,
    #[strum(serialize = "block.bell.resonate")]
    BellResonate,
    #[strum(serialize = "block.big_dripleaf.break")]
    BigDripleafBreak,
    #[strum(serialize = "block.big_dripleaf.fall")]
    BigDripleafFall,
    #[strum(serialize = "block.big_dripleaf.hit")]
    BigDripleafHit,
    #[strum(serialize = "block.big_dripleaf.place")]
    BigDripleafPlace,
    #[strum(serialize = "block.big_dripleaf.step")]
    BigDripleafStep,
    #[strum(serialize = "entity.blaze.ambient")]
    BlazeAmbient,
    #[strum(serialize = "entity.blaze.burn")]
    BlazeBurn,
    #[strum(serialize = "entity.blaze.death")]
    BlazeDeath,
    #[strum(serialize = "entity.blaze.hurt")]
    BlazeHurt,
    #[strum(serialize = "entity.blaze.shoot")]
    BlazeShoot,
    #[strum(serialize = "entity.boat.paddle_land")]
    BoatPaddleLand,
    #[strum(serialize = "entity.boat.paddle_water")]
    BoatPaddleWater,
    #[strum(serialize = "block.bone_block.break")]
    BoneBlockBreak,
    #[strum(serialize = "block.bone_block.fall")]
    BoneBlockFall,
    #[strum(serialize = "block.bone_block.hit")]
    BoneBlockHit,
    #[strum(serialize = "block.bone_block.place")]
    BoneBlockPlace,
    #[strum(serialize = "block.bone_block.step")]
    BoneBlockStep,
    #[strum(serialize = "item.bone_meal.use")]
    BoneMealUse,
    #[strum(serialize = "item.book.page_turn")]
    BookPageTurn,
    #[strum(serialize = "item.book.put")]
    BookPut,
    #[strum(serialize = "block.blastfurnace.fire_crackle")]
    BlastfurnaceFireCrackle,
    #[strum(serialize = "item.bottle.empty")]
    BottleEmpty,
    #[strum(serialize = "item.bottle.fill")]
    BottleFill,
    #[strum(serialize = "item.bottle.fill_dragonbreath")]
    BottleFillDragonbreath,
    #[strum(serialize = "block.brewing_stand.brew")]
    BrewingStandBrew,
    #[strum(serialize = "block.bubble_column.bubble_pop")]
    BubbleColumnBubblePop,
    #[strum(serialize = "block.bubble_column.upwards_ambient")]
    BubbleColumnUpwardsAmbient,
    #[strum(serialize = "block.bubble_column.upwards_inside")]
    BubbleColumnUpwardsInside,
    #[strum(serialize = "block.bubble_column.whirlpool_ambient")]
    BubbleColumnWhirlpoolAmbient,
    #[strum(serialize = "block.bubble_column.whirlpool_inside")]
    BubbleColumnWhirlpoolInside,
    #[strum(serialize = "item.bucket.empty")]
    BucketEmpty,
    #[strum(serialize = "item.bucket.empty_axolotl")]
    BucketEmptyAxolotl,
    #[strum(serialize = "item.bucket.empty_fish")]
    BucketEmptyFish,
    #[strum(serialize = "item.bucket.empty_lava")]
    BucketEmptyLava,
    #[strum(serialize = "item.bucket.empty_powder_snow")]
    BucketEmptyPowderSnow,
    #[strum(serialize = "item.bucket.fill")]
    BucketFill,
    #[strum(serialize = "item.bucket.fill_axolotl")]
    BucketFillAxolotl,
    #[strum(serialize = "item.bucket.fill_fish")]
    BucketFillFish,
    #[strum(serialize = "item.bucket.fill_lava")]
    BucketFillLava,
    #[strum(serialize = "item.bucket.fill_powder_snow")]
    BucketFillPowderSnow,
    #[strum(serialize = "block.cake.add_candle")]
    CakeAddCandle,
    #[strum(serialize = "block.calcite.break")]
    CalciteBreak,
    #[strum(serialize = "block.calcite.step")]
    CalciteStep,
    #[strum(serialize = "block.calcite.place")]
    CalcitePlace,
    #[strum(serialize = "block.calcite.hit")]
    CalciteHit,
    #[strum(serialize = "block.calcite.fall")]
    CalciteFall,
    #[strum(serialize = "block.campfire.crackle")]
    CampfireCrackle,
    #[strum(serialize = "block.candle.ambient")]
    CandleAmbient,
    #[strum(serialize = "block.candle.break")]
    CandleBreak,
    #[strum(serialize = "block.candle.extinguish")]
    CandleExtinguish,
    #[strum(serialize = "block.candle.fall")]
    CandleFall,
    #[strum(serialize = "block.candle.hit")]
    CandleHit,
    #[strum(serialize = "block.candle.place")]
    CandlePlace,
    #[strum(serialize = "block.candle.step")]
    CandleStep,
    #[strum(serialize = "entity.cat.ambient")]
    CatAmbient,
    #[strum(serialize = "entity.cat.stray_ambient")]
    CatStrayAmbient,
    #[strum(serialize = "entity.cat.death")]
    CatDeath,
    #[strum(serialize = "entity.cat.eat")]
    CatEat,
    #[strum(serialize = "entity.cat.hiss")]
    CatHiss,
    #[strum(serialize = "entity.cat.beg_for_food")]
    CatBegForFood,
    #[strum(serialize = "entity.cat.hurt")]
    CatHurt,
    #[strum(serialize = "entity.cat.purr")]
    CatPurr,
    #[strum(serialize = "entity.cat.purreow")]
    CatPurreow,
    #[strum(serialize = "block.cave_vines.break")]
    CaveVinesBreak,
    #[strum(serialize = "block.cave_vines.fall")]
    CaveVinesFall,
    #[strum(serialize = "block.cave_vines.hit")]
    CaveVinesHit,
    #[strum(serialize = "block.cave_vines.place")]
    CaveVinesPlace,
    #[strum(serialize = "block.cave_vines.step")]
    CaveVinesStep,
    #[strum(serialize = "block.cave_vines.pick_berries")]
    CaveVinesPickBerries,
    #[strum(serialize = "block.chain.break")]
    ChainBreak,
    #[strum(serialize = "block.chain.fall")]
    ChainFall,
    #[strum(serialize = "block.chain.hit")]
    ChainHit,
    #[strum(serialize = "block.chain.place")]
    ChainPlace,
    #[strum(serialize = "block.chain.step")]
    ChainStep,
    #[strum(serialize = "block.chest.close")]
    ChestClose,
    #[strum(serialize = "block.chest.locked")]
    ChestLocked,
    #[strum(serialize = "block.chest.open")]
    ChestOpen,
    #[strum(serialize = "entity.chicken.ambient")]
    ChickenAmbient,
    #[strum(serialize = "entity.chicken.death")]
    ChickenDeath,
    #[strum(serialize = "entity.chicken.egg")]
    ChickenEgg,
    #[strum(serialize = "entity.chicken.hurt")]
    ChickenHurt,
    #[strum(serialize = "entity.chicken.step")]
    ChickenStep,
    #[strum(serialize = "block.chorus_flower.death")]
    ChorusFlowerDeath,
    #[strum(serialize = "block.chorus_flower.grow")]
    ChorusFlowerGrow,
    #[strum(serialize = "item.chorus_fruit.teleport")]
    ChorusFruitTeleport,
    #[strum(serialize = "entity.cod.ambient")]
    CodAmbient,
    #[strum(serialize = "entity.cod.death")]
    CodDeath,
    #[strum(serialize = "entity.cod.flop")]
    CodFlop,
    #[strum(serialize = "entity.cod.hurt")]
    CodHurt,
    #[strum(serialize = "block.comparator.click")]
    ComparatorClick,
    #[strum(serialize = "block.composter.empty")]
    ComposterEmpty,
    #[strum(serialize = "block.composter.fill")]
    ComposterFill,
    #[strum(serialize = "block.composter.fill_success")]
    ComposterFillSuccess,
    #[strum(serialize = "block.composter.ready")]
    ComposterReady,
    #[strum(serialize = "block.conduit.activate")]
    ConduitActivate,
    #[strum(serialize = "block.conduit.ambient")]
    ConduitAmbient,
    #[strum(serialize = "block.conduit.ambient.short")]
    ConduitAmbientShort,
    #[strum(serialize = "block.conduit.attack.target")]
    ConduitAttackTarget,
    #[strum(serialize = "block.conduit.deactivate")]
    ConduitDeactivate,
    #[strum(serialize = "block.copper.break")]
    CopperBreak,
    #[strum(serialize = "block.copper.step")]
    CopperStep,
    #[strum(serialize = "block.copper.place")]
    CopperPlace,
    #[strum(serialize = "block.copper.hit")]
    CopperHit,
    #[strum(serialize = "block.copper.fall")]
    CopperFall,
    #[strum(serialize = "block.coral_block.break")]
    CoralBlockBreak,
    #[strum(serialize = "block.coral_block.fall")]
    CoralBlockFall,
    #[strum(serialize = "block.coral_block.hit")]
    CoralBlockHit,
    #[strum(serialize = "block.coral_block.place")]
    CoralBlockPlace,
    #[strum(serialize = "block.coral_block.step")]
    CoralBlockStep,
    #[strum(serialize = "entity.cow.ambient")]
    CowAmbient,
    #[strum(serialize = "entity.cow.death")]
    CowDeath,
    #[strum(serialize = "entity.cow.hurt")]
    CowHurt,
    #[strum(serialize = "entity.cow.milk")]
    CowMilk,
    #[strum(serialize = "entity.cow.step")]
    CowStep,
    #[strum(serialize = "entity.creeper.death")]
    CreeperDeath,
    #[strum(serialize = "entity.creeper.hurt")]
    CreeperHurt,
    #[strum(serialize = "entity.creeper.primed")]
    CreeperPrimed,
    #[strum(serialize = "block.crop.break")]
    CropBreak,
    #[strum(serialize = "item.crop.plant")]
    CropPlanted,
    #[strum(serialize = "item.crossbow.hit")]
    CrossbowHit,
    #[strum(serialize = "item.crossbow.loading_end")]
    CrossbowLoadingEnd,
    #[strum(serialize = "item.crossbow.loading_middle")]
    CrossbowLoadingMiddle,
    #[strum(serialize = "item.crossbow.loading_start")]
    CrossbowLoadingStart,
    #[strum(serialize = "item.crossbow.quick_charge_1")]
    CrossbowQuickCharge1,
    #[strum(serialize = "item.crossbow.quick_charge_2")]
    CrossbowQuickCharge2,
    #[strum(serialize = "item.crossbow.quick_charge_3")]
    CrossbowQuickCharge3,
    #[strum(serialize = "item.crossbow.shoot")]
    CrossbowShoot,
    #[strum(serialize = "block.deepslate_bricks.break")]
    DeepslateBricksBreak,
    #[strum(serialize = "block.deepslate_bricks.fall")]
    DeepslateBricksFall,
    #[strum(serialize = "block.deepslate_bricks.hit")]
    DeepslateBricksHit,
    #[strum(serialize = "block.deepslate_bricks.place")]
    DeepslateBricksPlace,
    #[strum(serialize = "block.deepslate_bricks.step")]
    DeepslateBricksStep,
    #[strum(serialize = "block.deepslate.break")]
    DeepslateBreak,
    #[strum(serialize = "block.deepslate.fall")]
    DeepslateFall,
    #[strum(serialize = "block.deepslate.hit")]
    DeepslateHit,
    #[strum(serialize = "block.deepslate.place")]
    DeepslatePlace,
    #[strum(serialize = "block.deepslate.step")]
    DeepslateStep,
    #[strum(serialize = "block.deepslate_tiles.break")]
    DeepslateTilesBreak,
    #[strum(serialize = "block.deepslate_tiles.fall")]
    DeepslateTilesFall,
    #[strum(serialize = "block.deepslate_tiles.hit")]
    DeepslateTilesHit,
    #[strum(serialize = "block.deepslate_tiles.place")]
    DeepslateTilesPlace,
    #[strum(serialize = "block.deepslate_tiles.step")]
    DeepslateTilesStep,
    #[strum(serialize = "block.dispenser.dispense")]
    DispenserDispense,
    #[strum(serialize = "block.dispenser.fail")]
    DispenserFail,
    #[strum(serialize = "block.dispenser.launch")]
    DispenserLaunch,
    #[strum(serialize = "entity.dolphin.ambient")]
    DolphinAmbient,
    #[strum(serialize = "entity.dolphin.ambient_water")]
    DolphinAmbientWater,
    #[strum(serialize = "entity.dolphin.attack")]
    DolphinAttack,
    #[strum(serialize = "entity.dolphin.death")]
    DolphinDeath,
    #[strum(serialize = "entity.dolphin.eat")]
    DolphinEat,
    #[strum(serialize = "entity.dolphin.hurt")]
    DolphinHurt,
    #[strum(serialize = "entity.dolphin.jump")]
    DolphinJump,
    #[strum(serialize = "entity.dolphin.play")]
    DolphinPlay,
    #[strum(serialize = "entity.dolphin.splash")]
    DolphinSplash,
    #[strum(serialize = "entity.dolphin.swim")]
    DolphinSwim,
    #[strum(serialize = "entity.donkey.ambient")]
    DonkeyAmbient,
    #[strum(serialize = "entity.donkey.angry")]
    DonkeyAngry,
    #[strum(serialize = "entity.donkey.chest")]
    DonkeyChest,
    #[strum(serialize = "entity.donkey.death")]
    DonkeyDeath,
    #[strum(serialize = "entity.donkey.eat")]
    DonkeyEat,
    #[strum(serialize = "entity.donkey.hurt")]
    DonkeyHurt,
    #[strum(serialize = "block.dripstone_block.break")]
    DripstoneBlockBreak,
    #[strum(serialize = "block.dripstone_block.step")]
    DripstoneBlockStep,
    #[strum(serialize = "block.dripstone_block.place")]
    DripstoneBlockPlace,
    #[strum(serialize = "block.dripstone_block.hit")]
    DripstoneBlockHit,
    #[strum(serialize = "block.dripstone_block.fall")]
    DripstoneBlockFall,
    #[strum(serialize = "block.pointed_dripstone.break")]
    PointedDripstoneBreak,
    #[strum(serialize = "block.pointed_dripstone.step")]
    PointedDripstoneStep,
    #[strum(serialize = "block.pointed_dripstone.place")]
    PointedDripstonePlace,
    #[strum(serialize = "block.pointed_dripstone.hit")]
    PointedDripstoneHit,
    #[strum(serialize = "block.pointed_dripstone.fall")]
    PointedDripstoneFall,
    #[strum(serialize = "block.pointed_dripstone.land")]
    PointedDripstoneLand,
    #[strum(serialize = "block.pointed_dripstone.drip_lava")]
    PointedDripstoneDripLava,
    #[strum(serialize = "block.pointed_dripstone.drip_water")]
    PointedDripstoneDripWater,
    #[strum(serialize = "block.pointed_dripstone.drip_lava_into_cauldron")]
    PointedDripstoneDripLavaIntoCauldron,
    #[strum(serialize = "block.pointed_dripstone.drip_water_into_cauldron")]
    PointedDripstoneDripWaterIntoCauldron,
    #[strum(serialize = "block.big_dripleaf.tilt_down")]
    BigDripleafTiltDown,
    #[strum(serialize = "block.big_dripleaf.tilt_up")]
    BigDripleafTiltUp,
    #[strum(serialize = "entity.drowned.ambient")]
    DrownedAmbient,
    #[strum(serialize = "entity.drowned.ambient_water")]
    DrownedAmbientWater,
    #[strum(serialize = "entity.drowned.death")]
    DrownedDeath,
    #[strum(serialize = "entity.drowned.death_water")]
    DrownedDeathWater,
    #[strum(serialize = "entity.drowned.hurt")]
    DrownedHurt,
    #[strum(serialize = "entity.drowned.hurt_water")]
    DrownedHurtWater,
    #[strum(serialize = "entity.drowned.shoot")]
    DrownedShoot,
    #[strum(serialize = "entity.drowned.step")]
    DrownedStep,
    #[strum(serialize = "entity.drowned.swim")]
    DrownedSwim,
    #[strum(serialize = "item.dye.use")]
    DyeUse,
    #[strum(serialize = "entity.egg.throw")]
    EggThrow,
    #[strum(serialize = "entity.elder_guardian.ambient")]
    ElderGuardianAmbient,
    #[strum(serialize = "entity.elder_guardian.ambient_land")]
    ElderGuardianAmbientLand,
    #[strum(serialize = "entity.elder_guardian.curse")]
    ElderGuardianCurse,
    #[strum(serialize = "entity.elder_guardian.death")]
    ElderGuardianDeath,
    #[strum(serialize = "entity.elder_guardian.death_land")]
    ElderGuardianDeathLand,
    #[strum(serialize = "entity.elder_guardian.flop")]
    ElderGuardianFlop,
    #[strum(serialize = "entity.elder_guardian.hurt")]
    ElderGuardianHurt,
    #[strum(serialize = "entity.elder_guardian.hurt_land")]
    ElderGuardianHurtLand,
    #[strum(serialize = "item.elytra.flying")]
    ElytraFlying,
    #[strum(serialize = "block.enchantment_table.use")]
    EnchantmentTableUse,
    #[strum(serialize = "block.ender_chest.close")]
    EnderChestClose,
    #[strum(serialize = "block.ender_chest.open")]
    EnderChestOpen,
    #[strum(serialize = "entity.ender_dragon.ambient")]
    EnderDragonAmbient,
    #[strum(serialize = "entity.ender_dragon.death")]
    EnderDragonDeath,
    #[strum(serialize = "entity.dragon_fireball.explode")]
    DragonFireballExplode,
    #[strum(serialize = "entity.ender_dragon.flap")]
    EnderDragonFlap,
    #[strum(serialize = "entity.ender_dragon.growl")]
    EnderDragonGrowl,
    #[strum(serialize = "entity.ender_dragon.hurt")]
    EnderDragonHurt,
    #[strum(serialize = "entity.ender_dragon.shoot")]
    EnderDragonShoot,
    #[strum(serialize = "entity.ender_eye.death")]
    EnderEyeDeath,
    #[strum(serialize = "entity.ender_eye.launch")]
    EnderEyeLaunch,
    #[strum(serialize = "entity.enderman.ambient")]
    EndermanAmbient,
    #[strum(serialize = "entity.enderman.death")]
    EndermanDeath,
    #[strum(serialize = "entity.enderman.hurt")]
    EndermanHurt,
    #[strum(serialize = "entity.enderman.scream")]
    EndermanScream,
    #[strum(serialize = "entity.enderman.stare")]
    EndermanStare,
    #[strum(serialize = "entity.enderman.teleport")]
    EndermanTeleport,
    #[strum(serialize = "entity.endermite.ambient")]
    EndermiteAmbient,
    #[strum(serialize = "entity.endermite.death")]
    EndermiteDeath,
    #[strum(serialize = "entity.endermite.hurt")]
    EndermiteHurt,
    #[strum(serialize = "entity.endermite.step")]
    EndermiteStep,
    #[strum(serialize = "entity.ender_pearl.throw")]
    EnderPearlThrow,
    #[strum(serialize = "block.end_gateway.spawn")]
    EndGatewaySpawn,
    #[strum(serialize = "block.end_portal_frame.fill")]
    EndPortalFrameFill,
    #[strum(serialize = "block.end_portal.spawn")]
    EndPortalSpawn,
    #[strum(serialize = "entity.evoker.ambient")]
    EvokerAmbient,
    #[strum(serialize = "entity.evoker.cast_spell")]
    EvokerCastSpell,
    #[strum(serialize = "entity.evoker.celebrate")]
    EvokerCelebrate,
    #[strum(serialize = "entity.evoker.death")]
    EvokerDeath,
    #[strum(serialize = "entity.evoker_fangs.attack")]
    EvokerFangsAttack,
    #[strum(serialize = "entity.evoker.hurt")]
    EvokerHurt,
    #[strum(serialize = "entity.evoker.prepare_attack")]
    EvokerPrepareAttack,
    #[strum(serialize = "entity.evoker.prepare_summon")]
    EvokerPrepareSummon,
    #[strum(serialize = "entity.evoker.prepare_wololo")]
    EvokerPrepareWololo,
    #[strum(serialize = "entity.experience_bottle.throw")]
    ExperienceBottleThrow,
    #[strum(serialize = "entity.experience_orb.pickup")]
    ExperienceOrbPickup,
    #[strum(serialize = "block.fence_gate.close")]
    FenceGateClose,
    #[strum(serialize = "block.fence_gate.open")]
    FenceGateOpen,
    #[strum(serialize = "item.firecharge.use")]
    FirechargeUse,
    #[strum(serialize = "entity.firework_rocket.blast")]
    FireworkRocketBlast,
    #[strum(serialize = "entity.firework_rocket.blast_far")]
    FireworkRocketBlastFar,
    #[strum(serialize = "entity.firework_rocket.large_blast")]
    FireworkRocketLargeBlast,
    #[strum(serialize = "entity.firework_rocket.large_blast_far")]
    FireworkRocketLargeBlastFar,
    #[strum(serialize = "entity.firework_rocket.launch")]
    FireworkRocketLaunch,
    #[strum(serialize = "entity.firework_rocket.shoot")]
    FireworkRocketShoot,
    #[strum(serialize = "entity.firework_rocket.twinkle")]
    FireworkRocketTwinkle,
    #[strum(serialize = "entity.firework_rocket.twinkle_far")]
    FireworkRocketTwinkleFar,
    #[strum(serialize = "block.fire.ambient")]
    FireAmbient,
    #[strum(serialize = "block.fire.extinguish")]
    FireExtinguish,
    #[strum(serialize = "entity.fish.swim")]
    FishSwim,
    #[strum(serialize = "entity.fishing_bobber.retrieve")]
    FishingBobberRetrieve,
    #[strum(serialize = "entity.fishing_bobber.splash")]
    FishingBobberSplash,
    #[strum(serialize = "entity.fishing_bobber.throw")]
    FishingBobberThrow,
    #[strum(serialize = "item.flintandsteel.use")]
    FlintandsteelUse,
    #[strum(serialize = "block.flowering_azalea.break")]
    FloweringAzaleaBreak,
    #[strum(serialize = "block.flowering_azalea.fall")]
    FloweringAzaleaFall,
    #[strum(serialize = "block.flowering_azalea.hit")]
    FloweringAzaleaHit,
    #[strum(serialize = "block.flowering_azalea.place")]
    FloweringAzaleaPlace,
    #[strum(serialize = "block.flowering_azalea.step")]
    FloweringAzaleaStep,
    #[strum(serialize = "entity.fox.aggro")]
    FoxAggro,
    #[strum(serialize = "entity.fox.ambient")]
    FoxAmbient,
    #[strum(serialize = "entity.fox.bite")]
    FoxBite,
    #[strum(serialize = "entity.fox.death")]
    FoxDeath,
    #[strum(serialize = "entity.fox.eat")]
    FoxEat,
    #[strum(serialize = "entity.fox.hurt")]
    FoxHurt,
    #[strum(serialize = "entity.fox.screech")]
    FoxScreech,
    #[strum(serialize = "entity.fox.sleep")]
    FoxSleep,
    #[strum(serialize = "entity.fox.sniff")]
    FoxSniff,
    #[strum(serialize = "entity.fox.spit")]
    FoxSpit,
    #[strum(serialize = "entity.fox.teleport")]
    FoxTeleport,
    #[strum(serialize = "block.roots.break")]
    RootsBreak,
    #[strum(serialize = "block.roots.step")]
    RootsStep,
    #[strum(serialize = "block.roots.place")]
    RootsPlace,
    #[strum(serialize = "block.roots.hit")]
    RootsHit,
    #[strum(serialize = "block.roots.fall")]
    RootsFall,
    #[strum(serialize = "block.furnace.fire_crackle")]
    FurnaceFireCrackle,
    #[strum(serialize = "entity.generic.big_fall")]
    GenericBigFall,
    #[strum(serialize = "entity.generic.burn")]
    GenericBurn,
    #[strum(serialize = "entity.generic.death")]
    GenericDeath,
    #[strum(serialize = "entity.generic.drink")]
    GenericDrink,
    #[strum(serialize = "entity.generic.eat")]
    GenericEat,
    #[strum(serialize = "entity.generic.explode")]
    GenericExplode,
    #[strum(serialize = "entity.generic.extinguish_fire")]
    GenericExtinguishFire,
    #[strum(serialize = "entity.generic.hurt")]
    GenericHurt,
    #[strum(serialize = "entity.generic.small_fall")]
    GenericSmallFall,
    #[strum(serialize = "entity.generic.splash")]
    GenericSplash,
    #[strum(serialize = "entity.generic.swim")]
    GenericSwim,
    #[strum(serialize = "entity.ghast.ambient")]
    GhastAmbient,
    #[strum(serialize = "entity.ghast.death")]
    GhastDeath,
    #[strum(serialize = "entity.ghast.hurt")]
    GhastHurt,
    #[strum(serialize = "entity.ghast.scream")]
    GhastScream,
    #[strum(serialize = "entity.ghast.shoot")]
    GhastShoot,
    #[strum(serialize = "entity.ghast.warn")]
    GhastWarn,
    #[strum(serialize = "block.gilded_blackstone.break")]
    GildedBlackstoneBreak,
    #[strum(serialize = "block.gilded_blackstone.fall")]
    GildedBlackstoneFall,
    #[strum(serialize = "block.gilded_blackstone.hit")]
    GildedBlackstoneHit,
    #[strum(serialize = "block.gilded_blackstone.place")]
    GildedBlackstonePlace,
    #[strum(serialize = "block.gilded_blackstone.step")]
    GildedBlackstoneStep,
    #[strum(serialize = "block.glass.break")]
    GlassBreak,
    #[strum(serialize = "block.glass.fall")]
    GlassFall,
    #[strum(serialize = "block.glass.hit")]
    GlassHit,
    #[strum(serialize = "block.glass.place")]
    GlassPlace,
    #[strum(serialize = "block.glass.step")]
    GlassStep,
    #[strum(serialize = "item.glow_ink_sac.use")]
    GlowInkSacUse,
    #[strum(serialize = "entity.glow_item_frame.add_item")]
    GlowItemFrameAddItem,
    #[strum(serialize = "entity.glow_item_frame.break")]
    GlowItemFrameBreak,
    #[strum(serialize = "entity.glow_item_frame.place")]
    GlowItemFramePlace,
    #[strum(serialize = "entity.glow_item_frame.remove_item")]
    GlowItemFrameRemoveItem,
    #[strum(serialize = "entity.glow_item_frame.rotate_item")]
    GlowItemFrameRotateItem,
    #[strum(serialize = "entity.glow_squid.ambient")]
    GlowSquidAmbient,
    #[strum(serialize = "entity.glow_squid.death")]
    GlowSquidDeath,
    #[strum(serialize = "entity.glow_squid.hurt")]
    GlowSquidHurt,
    #[strum(serialize = "entity.glow_squid.squirt")]
    GlowSquidSquirt,
    #[strum(serialize = "entity.goat.ambient")]
    GoatAmbient,
    #[strum(serialize = "entity.goat.death")]
    GoatDeath,
    #[strum(serialize = "entity.goat.eat")]
    GoatEat,
    #[strum(serialize = "entity.goat.hurt")]
    GoatHurt,
    #[strum(serialize = "entity.goat.long_jump")]
    GoatLongJump,
    #[strum(serialize = "entity.goat.milk")]
    GoatMilk,
    #[strum(serialize = "entity.goat.prepare_ram")]
    GoatPrepareRam,
    #[strum(serialize = "entity.goat.ram_impact")]
    GoatRamImpact,
    #[strum(serialize = "entity.goat.screaming.ambient")]
    GoatScreamingAmbient,
    #[strum(serialize = "entity.goat.screaming.death")]
    GoatScreamingDeath,
    #[strum(serialize = "entity.goat.screaming.eat")]
    GoatScreamingEat,
    #[strum(serialize = "entity.goat.screaming.hurt")]
    GoatScreamingHurt,
    #[strum(serialize = "entity.goat.screaming.long_jump")]
    GoatScreamingLongJump,
    #[strum(serialize = "entity.goat.screaming.milk")]
    GoatScreamingMilk,
    #[strum(serialize = "entity.goat.screaming.prepare_ram")]
    GoatScreamingPrepareRam,
    #[strum(serialize = "entity.goat.screaming.ram_impact")]
    GoatScreamingRamImpact,
    #[strum(serialize = "entity.goat.step")]
    GoatStep,
    #[strum(serialize = "block.grass.break")]
    GrassBreak,
    #[strum(serialize = "block.grass.fall")]
    GrassFall,
    #[strum(serialize = "block.grass.hit")]
    GrassHit,
    #[strum(serialize = "block.grass.place")]
    GrassPlace,
    #[strum(serialize = "block.grass.step")]
    GrassStep,
    #[strum(serialize = "block.gravel.break")]
    GravelBreak,
    #[strum(serialize = "block.gravel.fall")]
    GravelFall,
    #[strum(serialize = "block.gravel.hit")]
    GravelHit,
    #[strum(serialize = "block.gravel.place")]
    GravelPlace,
    #[strum(serialize = "block.gravel.step")]
    GravelStep,
    #[strum(serialize = "block.grindstone.use")]
    GrindstoneUse,
    #[strum(serialize = "entity.guardian.ambient")]
    GuardianAmbient,
    #[strum(serialize = "entity.guardian.ambient_land")]
    GuardianAmbientLand,
    #[strum(serialize = "entity.guardian.attack")]
    GuardianAttack,
    #[strum(serialize = "entity.guardian.death")]
    GuardianDeath,
    #[strum(serialize = "entity.guardian.death_land")]
    GuardianDeathLand,
    #[strum(serialize = "entity.guardian.flop")]
    GuardianFlop,
    #[strum(serialize = "entity.guardian.hurt")]
    GuardianHurt,
    #[strum(serialize = "entity.guardian.hurt_land")]
    GuardianHurtLand,
    #[strum(serialize = "block.hanging_roots.break")]
    HangingRootsBreak,
    #[strum(serialize = "block.hanging_roots.fall")]
    HangingRootsFall,
    #[strum(serialize = "block.hanging_roots.hit")]
    HangingRootsHit,
    #[strum(serialize = "block.hanging_roots.place")]
    HangingRootsPlace,
    #[strum(serialize = "block.hanging_roots.step")]
    HangingRootsStep,
    #[strum(serialize = "item.hoe.till")]
    HoeTill,
    #[strum(serialize = "entity.hoglin.ambient")]
    HoglinAmbient,
    #[strum(serialize = "entity.hoglin.angry")]
    HoglinAngry,
    #[strum(serialize = "entity.hoglin.attack")]
    HoglinAttack,
    #[strum(serialize = "entity.hoglin.converted_to_zombified")]
    HoglinConvertedToZombified,
    #[strum(serialize = "entity.hoglin.death")]
    HoglinDeath,
    #[strum(serialize = "entity.hoglin.hurt")]
    HoglinHurt,
    #[strum(serialize = "entity.hoglin.retreat")]
    HoglinRetreat,
    #[strum(serialize = "entity.hoglin.step")]
    HoglinStep,
    #[strum(serialize = "block.honey_block.break")]
    HoneyBlockBreak,
    #[strum(serialize = "block.honey_block.fall")]
    HoneyBlockFall,
    #[strum(serialize = "block.honey_block.hit")]
    HoneyBlockHit,
    #[strum(serialize = "block.honey_block.place")]
    HoneyBlockPlace,
    #[strum(serialize = "block.honey_block.slide")]
    HoneyBlockSlide,
    #[strum(serialize = "block.honey_block.step")]
    HoneyBlockStep,
    #[strum(serialize = "item.honeycomb.wax_on")]
    HoneycombWaxOn,
    #[strum(serialize = "item.honey_bottle.drink")]
    HoneyDrink,
    #[strum(serialize = "entity.horse.ambient")]
    HorseAmbient,
    #[strum(serialize = "entity.horse.angry")]
    HorseAngry,
    #[strum(serialize = "entity.horse.armor")]
    HorseArmor,
    #[strum(serialize = "entity.horse.breathe")]
    HorseBreathe,
    #[strum(serialize = "entity.horse.death")]
    HorseDeath,
    #[strum(serialize = "entity.horse.eat")]
    HorseEat,
    #[strum(serialize = "entity.horse.gallop")]
    HorseGallop,
    #[strum(serialize = "entity.horse.hurt")]
    HorseHurt,
    #[strum(serialize = "entity.horse.jump")]
    HorseJump,
    #[strum(serialize = "entity.horse.land")]
    HorseLand,
    #[strum(serialize = "entity.horse.saddle")]
    HorseSaddle,
    #[strum(serialize = "entity.horse.step")]
    HorseStep,
    #[strum(serialize = "entity.horse.step_wood")]
    HorseStepWood,
    #[strum(serialize = "entity.hostile.big_fall")]
    HostileBigFall,
    #[strum(serialize = "entity.hostile.death")]
    HostileDeath,
    #[strum(serialize = "entity.hostile.hurt")]
    HostileHurt,
    #[strum(serialize = "entity.hostile.small_fall")]
    HostileSmallFall,
    #[strum(serialize = "entity.hostile.splash")]
    HostileSplash,
    #[strum(serialize = "entity.hostile.swim")]
    HostileSwim,
    #[strum(serialize = "entity.husk.ambient")]
    HuskAmbient,
    #[strum(serialize = "entity.husk.converted_to_zombie")]
    HuskConvertedToZombie,
    #[strum(serialize = "entity.husk.death")]
    HuskDeath,
    #[strum(serialize = "entity.husk.hurt")]
    HuskHurt,
    #[strum(serialize = "entity.husk.step")]
    HuskStep,
    #[strum(serialize = "entity.illusioner.ambient")]
    IllusionerAmbient,
    #[strum(serialize = "entity.illusioner.cast_spell")]
    IllusionerCastSpell,
    #[strum(serialize = "entity.illusioner.death")]
    IllusionerDeath,
    #[strum(serialize = "entity.illusioner.hurt")]
    IllusionerHurt,
    #[strum(serialize = "entity.illusioner.mirror_move")]
    IllusionerMirrorMove,
    #[strum(serialize = "entity.illusioner.prepare_blindness")]
    IllusionerPrepareBlindness,
    #[strum(serialize = "entity.illusioner.prepare_mirror")]
    IllusionerPrepareMirror,
    #[strum(serialize = "item.ink_sac.use")]
    InkSacUse,
    #[strum(serialize = "block.iron_door.close")]
    IronDoorClose,
    #[strum(serialize = "block.iron_door.open")]
    IronDoorOpen,
    #[strum(serialize = "entity.iron_golem.attack")]
    IronGolemAttack,
    #[strum(serialize = "entity.iron_golem.damage")]
    IronGolemDamage,
    #[strum(serialize = "entity.iron_golem.death")]
    IronGolemDeath,
    #[strum(serialize = "entity.iron_golem.hurt")]
    IronGolemHurt,
    #[strum(serialize = "entity.iron_golem.repair")]
    IronGolemRepair,
    #[strum(serialize = "entity.iron_golem.step")]
    IronGolemStep,
    #[strum(serialize = "block.iron_trapdoor.close")]
    IronTrapdoorClose,
    #[strum(serialize = "block.iron_trapdoor.open")]
    IronTrapdoorOpen,
    #[strum(serialize = "entity.item_frame.add_item")]
    ItemFrameAddItem,
    #[strum(serialize = "entity.item_frame.break")]
    ItemFrameBreak,
    #[strum(serialize = "entity.item_frame.place")]
    ItemFramePlace,
    #[strum(serialize = "entity.item_frame.remove_item")]
    ItemFrameRemoveItem,
    #[strum(serialize = "entity.item_frame.rotate_item")]
    ItemFrameRotateItem,
    #[strum(serialize = "entity.item.break")]
    ItemBreak,
    #[strum(serialize = "entity.item.pickup")]
    ItemPickup,
    #[strum(serialize = "block.ladder.break")]
    LadderBreak,
    #[strum(serialize = "block.ladder.fall")]
    LadderFall,
    #[strum(serialize = "block.ladder.hit")]
    LadderHit,
    #[strum(serialize = "block.ladder.place")]
    LadderPlace,
    #[strum(serialize = "block.ladder.step")]
    LadderStep,
    #[strum(serialize = "block.lantern.break")]
    LanternBreak,
    #[strum(serialize = "block.lantern.fall")]
    LanternFall,
    #[strum(serialize = "block.lantern.hit")]
    LanternHit,
    #[strum(serialize = "block.lantern.place")]
    LanternPlace,
    #[strum(serialize = "block.lantern.step")]
    LanternStep,
    #[strum(serialize = "block.large_amethyst_bud.break")]
    LargeAmethystBudBreak,
    #[strum(serialize = "block.large_amethyst_bud.place")]
    LargeAmethystBudPlace,
    #[strum(serialize = "block.lava.ambient")]
    LavaAmbient,
    #[strum(serialize = "block.lava.extinguish")]
    LavaExtinguish,
    #[strum(serialize = "block.lava.pop")]
    LavaPop,
    #[strum(serialize = "entity.leash_knot.break")]
    LeashKnotBreak,
    #[strum(serialize = "entity.leash_knot.place")]
    LeashKnotPlace,
    #[strum(serialize = "block.lever.click")]
    LeverClick,
    #[strum(serialize = "entity.lightning_bolt.impact")]
    LightningBoltImpact,
    #[strum(serialize = "entity.lightning_bolt.thunder")]
    LightningBoltThunder,
    #[strum(serialize = "entity.lingering_potion.throw")]
    LingeringPotionThrow,
    #[strum(serialize = "entity.llama.ambient")]
    LlamaAmbient,
    #[strum(serialize = "entity.llama.angry")]
    LlamaAngry,
    #[strum(serialize = "entity.llama.chest")]
    LlamaChest,
    #[strum(serialize = "entity.llama.death")]
    LlamaDeath,
    #[strum(serialize = "entity.llama.eat")]
    LlamaEat,
    #[strum(serialize = "entity.llama.hurt")]
    LlamaHurt,
    #[strum(serialize = "entity.llama.spit")]
    LlamaSpit,
    #[strum(serialize = "entity.llama.step")]
    LlamaStep,
    #[strum(serialize = "entity.llama.swag")]
    LlamaSwag,
    #[strum(serialize = "entity.magma_cube.death_small")]
    MagmaCubeDeathSmall,
    #[strum(serialize = "block.lodestone.break")]
    LodestoneBreak,
    #[strum(serialize = "block.lodestone.step")]
    LodestoneStep,
    #[strum(serialize = "block.lodestone.place")]
    LodestonePlace,
    #[strum(serialize = "block.lodestone.hit")]
    LodestoneHit,
    #[strum(serialize = "block.lodestone.fall")]
    LodestoneFall,
    #[strum(serialize = "item.lodestone_compass.lock")]
    LodestoneCompassLock,
    #[strum(serialize = "entity.magma_cube.death")]
    MagmaCubeDeath,
    #[strum(serialize = "entity.magma_cube.hurt")]
    MagmaCubeHurt,
    #[strum(serialize = "entity.magma_cube.hurt_small")]
    MagmaCubeHurtSmall,
    #[strum(serialize = "entity.magma_cube.jump")]
    MagmaCubeJump,
    #[strum(serialize = "entity.magma_cube.squish")]
    MagmaCubeSquish,
    #[strum(serialize = "entity.magma_cube.squish_small")]
    MagmaCubeSquishSmall,
    #[strum(serialize = "block.medium_amethyst_bud.break")]
    MediumAmethystBudBreak,
    #[strum(serialize = "block.medium_amethyst_bud.place")]
    MediumAmethystBudPlace,
    #[strum(serialize = "block.metal.break")]
    MetalBreak,
    #[strum(serialize = "block.metal.fall")]
    MetalFall,
    #[strum(serialize = "block.metal.hit")]
    MetalHit,
    #[strum(serialize = "block.metal.place")]
    MetalPlace,
    #[strum(serialize = "block.metal_pressure_plate.click_off")]
    MetalPressurePlateClickOff,
    #[strum(serialize = "block.metal_pressure_plate.click_on")]
    MetalPressurePlateClickOn,
    #[strum(serialize = "block.metal.step")]
    MetalStep,
    #[strum(serialize = "entity.minecart.inside.underwater")]
    MinecartInsideUnderwater,
    #[strum(serialize = "entity.minecart.inside")]
    MinecartInside,
    #[strum(serialize = "entity.minecart.riding")]
    MinecartRiding,
    #[strum(serialize = "entity.mooshroom.convert")]
    MooshroomConvert,
    #[strum(serialize = "entity.mooshroom.eat")]
    MooshroomEat,
    #[strum(serialize = "entity.mooshroom.milk")]
    MooshroomMilk,
    #[strum(serialize = "entity.mooshroom.suspicious_milk")]
    MooshroomMilkSuspiciously,
    #[strum(serialize = "entity.mooshroom.shear")]
    MooshroomShear,
    #[strum(serialize = "block.moss_carpet.break")]
    MossCarpetBreak,
    #[strum(serialize = "block.moss_carpet.fall")]
    MossCarpetFall,
    #[strum(serialize = "block.moss_carpet.hit")]
    MossCarpetHit,
    #[strum(serialize = "block.moss_carpet.place")]
    MossCarpetPlace,
    #[strum(serialize = "block.moss_carpet.step")]
    MossCarpetStep,
    #[strum(serialize = "block.moss.break")]
    MossBreak,
    #[strum(serialize = "block.moss.fall")]
    MossFall,
    #[strum(serialize = "block.moss.hit")]
    MossHit,
    #[strum(serialize = "block.moss.place")]
    MossPlace,
    #[strum(serialize = "block.moss.step")]
    MossStep,
    #[strum(serialize = "entity.mule.ambient")]
    MuleAmbient,
    #[strum(serialize = "entity.mule.angry")]
    MuleAngry,
    #[strum(serialize = "entity.mule.chest")]
    MuleChest,
    #[strum(serialize = "entity.mule.death")]
    MuleDeath,
    #[strum(serialize = "entity.mule.eat")]
    MuleEat,
    #[strum(serialize = "entity.mule.hurt")]
    MuleHurt,
    #[strum(serialize = "music.creative")]
    MusicCreative,
    #[strum(serialize = "music.credits")]
    MusicCredits,
    #[strum(serialize = "music_disc.11")]
    MusicDisc11,
    #[strum(serialize = "music_disc.13")]
    MusicDisc13,
    #[strum(serialize = "music_disc.blocks")]
    MusicDiscBlocks,
    #[strum(serialize = "music_disc.cat")]
    MusicDiscCat,
    #[strum(serialize = "music_disc.chirp")]
    MusicDiscChirp,
    #[strum(serialize = "music_disc.far")]
    MusicDiscFar,
    #[strum(serialize = "music_disc.mall")]
    MusicDiscMall,
    #[strum(serialize = "music_disc.mellohi")]
    MusicDiscMellohi,
    #[strum(serialize = "music_disc.pigstep")]
    MusicDiscPigstep,
    #[strum(serialize = "music_disc.stal")]
    MusicDiscStal,
    #[strum(serialize = "music_disc.strad")]
    MusicDiscStrad,
    #[strum(serialize = "music_disc.wait")]
    MusicDiscWait,
    #[strum(serialize = "music_disc.ward")]
    MusicDiscWard,
    #[strum(serialize = "music.dragon")]
    MusicDragon,
    #[strum(serialize = "music.end")]
    MusicEnd,
    #[strum(serialize = "music.game")]
    MusicGame,
    #[strum(serialize = "music.menu")]
    MusicMenu,
    #[strum(serialize = "music.nether.basalt_deltas")]
    MusicBiomeBasaltDeltas,
    #[strum(serialize = "music.nether.nether_wastes")]
    MusicBiomeNetherWastes,
    #[strum(serialize = "music.nether.soul_sand_valley")]
    MusicBiomeSoulSandValley,
    #[strum(serialize = "music.nether.crimson_forest")]
    MusicBiomeCrimsonForest,
    #[strum(serialize = "music.nether.warped_forest")]
    MusicBiomeWarpedForest,
    #[strum(serialize = "music.under_water")]
    MusicUnderWater,
    #[strum(serialize = "block.nether_bricks.break")]
    NetherBricksBreak,
    #[strum(serialize = "block.nether_bricks.step")]
    NetherBricksStep,
    #[strum(serialize = "block.nether_bricks.place")]
    NetherBricksPlace,
    #[strum(serialize = "block.nether_bricks.hit")]
    NetherBricksHit,
    #[strum(serialize = "block.nether_bricks.fall")]
    NetherBricksFall,
    #[strum(serialize = "block.nether_wart.break")]
    NetherWartBreak,
    #[strum(serialize = "item.nether_wart.plant")]
    NetherWartPlanted,
    #[strum(serialize = "block.stem.break")]
    StemBreak,
    #[strum(serialize = "block.stem.step")]
    StemStep,
    #[strum(serialize = "block.stem.place")]
    StemPlace,
    #[strum(serialize = "block.stem.hit")]
    StemHit,
    #[strum(serialize = "block.stem.fall")]
    StemFall,
    #[strum(serialize = "block.nylium.break")]
    NyliumBreak,
    #[strum(serialize = "block.nylium.step")]
    NyliumStep,
    #[strum(serialize = "block.nylium.place")]
    NyliumPlace,
    #[strum(serialize = "block.nylium.hit")]
    NyliumHit,
    #[strum(serialize = "block.nylium.fall")]
    NyliumFall,
    #[strum(serialize = "block.nether_sprouts.break")]
    NetherSproutsBreak,
    #[strum(serialize = "block.nether_sprouts.step")]
    NetherSproutsStep,
    #[strum(serialize = "block.nether_sprouts.place")]
    NetherSproutsPlace,
    #[strum(serialize = "block.nether_sprouts.hit")]
    NetherSproutsHit,
    #[strum(serialize = "block.nether_sprouts.fall")]
    NetherSproutsFall,
    #[strum(serialize = "block.fungus.break")]
    FungusBreak,
    #[strum(serialize = "block.fungus.step")]
    FungusStep,
    #[strum(serialize = "block.fungus.place")]
    FungusPlace,
    #[strum(serialize = "block.fungus.hit")]
    FungusHit,
    #[strum(serialize = "block.fungus.fall")]
    FungusFall,
    #[strum(serialize = "block.weeping_vines.break")]
    WeepingVinesBreak,
    #[strum(serialize = "block.weeping_vines.step")]
    WeepingVinesStep,
    #[strum(serialize = "block.weeping_vines.place")]
    WeepingVinesPlace,
    #[strum(serialize = "block.weeping_vines.hit")]
    WeepingVinesHit,
    #[strum(serialize = "block.weeping_vines.fall")]
    WeepingVinesFall,
    #[strum(serialize = "block.wart_block.break")]
    WartBlockBreak,
    #[strum(serialize = "block.wart_block.step")]
    WartBlockStep,
    #[strum(serialize = "block.wart_block.place")]
    WartBlockPlace,
    #[strum(serialize = "block.wart_block.hit")]
    WartBlockHit,
    #[strum(serialize = "block.wart_block.fall")]
    WartBlockFall,
    #[strum(serialize = "block.netherite_block.break")]
    NetheriteBlockBreak,
    #[strum(serialize = "block.netherite_block.step")]
    NetheriteBlockStep,
    #[strum(serialize = "block.netherite_block.place")]
    NetheriteBlockPlace,
    #[strum(serialize = "block.netherite_block.hit")]
    NetheriteBlockHit,
    #[strum(serialize = "block.netherite_block.fall")]
    NetheriteBlockFall,
    #[strum(serialize = "block.netherrack.break")]
    NetherrackBreak,
    #[strum(serialize = "block.netherrack.step")]
    NetherrackStep,
    #[strum(serialize = "block.netherrack.place")]
    NetherrackPlace,
    #[strum(serialize = "block.netherrack.hit")]
    NetherrackHit,
    #[strum(serialize = "block.netherrack.fall")]
    NetherrackFall,
    #[strum(serialize = "block.note_block.basedrum")]
    NoteBlockBasedrum,
    #[strum(serialize = "block.note_block.bass")]
    NoteBlockBass,
    #[strum(serialize = "block.note_block.bell")]
    NoteBlockBell,
    #[strum(serialize = "block.note_block.chime")]
    NoteBlockChime,
    #[strum(serialize = "block.note_block.flute")]
    NoteBlockFlute,
    #[strum(serialize = "block.note_block.guitar")]
    NoteBlockGuitar,
    #[strum(serialize = "block.note_block.harp")]
    NoteBlockHarp,
    #[strum(serialize = "block.note_block.hat")]
    NoteBlockHat,
    #[strum(serialize = "block.note_block.pling")]
    NoteBlockPling,
    #[strum(serialize = "block.note_block.snare")]
    NoteBlockSnare,
    #[strum(serialize = "block.note_block.xylophone")]
    NoteBlockXylophone,
    #[strum(serialize = "block.note_block.iron_xylophone")]
    NoteBlockIronXylophone,
    #[strum(serialize = "block.note_block.cow_bell")]
    NoteBlockCowBell,
    #[strum(serialize = "block.note_block.didgeridoo")]
    NoteBlockDidgeridoo,
    #[strum(serialize = "block.note_block.bit")]
    NoteBlockBit,
    #[strum(serialize = "block.note_block.banjo")]
    NoteBlockBanjo,
    #[strum(serialize = "entity.ocelot.hurt")]
    OcelotHurt,
    #[strum(serialize = "entity.ocelot.ambient")]
    OcelotAmbient,
    #[strum(serialize = "entity.ocelot.death")]
    OcelotDeath,
    #[strum(serialize = "entity.painting.break")]
    PaintingBreak,
    #[strum(serialize = "entity.painting.place")]
    PaintingPlace,
    #[strum(serialize = "entity.panda.pre_sneeze")]
    PandaPreSneeze,
    #[strum(serialize = "entity.panda.sneeze")]
    PandaSneeze,
    #[strum(serialize = "entity.panda.ambient")]
    PandaAmbient,
    #[strum(serialize = "entity.panda.death")]
    PandaDeath,
    #[strum(serialize = "entity.panda.eat")]
    PandaEat,
    #[strum(serialize = "entity.panda.step")]
    PandaStep,
    #[strum(serialize = "entity.panda.cant_breed")]
    PandaCantBreed,
    #[strum(serialize = "entity.panda.aggressive_ambient")]
    PandaAggressiveAmbient,
    #[strum(serialize = "entity.panda.worried_ambient")]
    PandaWorriedAmbient,
    #[strum(serialize = "entity.panda.hurt")]
    PandaHurt,
    #[strum(serialize = "entity.panda.bite")]
    PandaBite,
    #[strum(serialize = "entity.parrot.ambient")]
    ParrotAmbient,
    #[strum(serialize = "entity.parrot.death")]
    ParrotDeath,
    #[strum(serialize = "entity.parrot.eat")]
    ParrotEat,
    #[strum(serialize = "entity.parrot.fly")]
    ParrotFly,
    #[strum(serialize = "entity.parrot.hurt")]
    ParrotHurt,
    #[strum(serialize = "entity.parrot.imitate.blaze")]
    ParrotImitateBlaze,
    #[strum(serialize = "entity.parrot.imitate.creeper")]
    ParrotImitateCreeper,
    #[strum(serialize = "entity.parrot.imitate.drowned")]
    ParrotImitateDrowned,
    #[strum(serialize = "entity.parrot.imitate.elder_guardian")]
    ParrotImitateElderGuardian,
    #[strum(serialize = "entity.parrot.imitate.ender_dragon")]
    ParrotImitateEnderDragon,
    #[strum(serialize = "entity.parrot.imitate.endermite")]
    ParrotImitateEndermite,
    #[strum(serialize = "entity.parrot.imitate.evoker")]
    ParrotImitateEvoker,
    #[strum(serialize = "entity.parrot.imitate.ghast")]
    ParrotImitateGhast,
    #[strum(serialize = "entity.parrot.imitate.guardian")]
    ParrotImitateGuardian,
    #[strum(serialize = "entity.parrot.imitate.hoglin")]
    ParrotImitateHoglin,
    #[strum(serialize = "entity.parrot.imitate.husk")]
    ParrotImitateHusk,
    #[strum(serialize = "entity.parrot.imitate.illusioner")]
    ParrotImitateIllusioner,
    #[strum(serialize = "entity.parrot.imitate.magma_cube")]
    ParrotImitateMagmaCube,
    #[strum(serialize = "entity.parrot.imitate.phantom")]
    ParrotImitatePhantom,
    #[strum(serialize = "entity.parrot.imitate.piglin")]
    ParrotImitatePiglin,
    #[strum(serialize = "entity.parrot.imitate.piglin_brute")]
    ParrotImitatePiglinBrute,
    #[strum(serialize = "entity.parrot.imitate.pillager")]
    ParrotImitatePillager,
    #[strum(serialize = "entity.parrot.imitate.ravager")]
    ParrotImitateRavager,
    #[strum(serialize = "entity.parrot.imitate.shulker")]
    ParrotImitateShulker,
    #[strum(serialize = "entity.parrot.imitate.silverfish")]
    ParrotImitateSilverfish,
    #[strum(serialize = "entity.parrot.imitate.skeleton")]
    ParrotImitateSkeleton,
    #[strum(serialize = "entity.parrot.imitate.slime")]
    ParrotImitateSlime,
    #[strum(serialize = "entity.parrot.imitate.spider")]
    ParrotImitateSpider,
    #[strum(serialize = "entity.parrot.imitate.stray")]
    ParrotImitateStray,
    #[strum(serialize = "entity.parrot.imitate.vex")]
    ParrotImitateVex,
    #[strum(serialize = "entity.parrot.imitate.vindicator")]
    ParrotImitateVindicator,
    #[strum(serialize = "entity.parrot.imitate.witch")]
    ParrotImitateWitch,
    #[strum(serialize = "entity.parrot.imitate.wither")]
    ParrotImitateWither,
    #[strum(serialize = "entity.parrot.imitate.wither_skeleton")]
    ParrotImitateWitherSkeleton,
    #[strum(serialize = "entity.parrot.imitate.zoglin")]
    ParrotImitateZoglin,
    #[strum(serialize = "entity.parrot.imitate.zombie")]
    ParrotImitateZombie,
    #[strum(serialize = "entity.parrot.imitate.zombie_villager")]
    ParrotImitateZombieVillager,
    #[strum(serialize = "entity.parrot.step")]
    ParrotStep,
    #[strum(serialize = "entity.phantom.ambient")]
    PhantomAmbient,
    #[strum(serialize = "entity.phantom.bite")]
    PhantomBite,
    #[strum(serialize = "entity.phantom.death")]
    PhantomDeath,
    #[strum(serialize = "entity.phantom.flap")]
    PhantomFlap,
    #[strum(serialize = "entity.phantom.hurt")]
    PhantomHurt,
    #[strum(serialize = "entity.phantom.swoop")]
    PhantomSwoop,
    #[strum(serialize = "entity.pig.ambient")]
    PigAmbient,
    #[strum(serialize = "entity.pig.death")]
    PigDeath,
    #[strum(serialize = "entity.pig.hurt")]
    PigHurt,
    #[strum(serialize = "entity.pig.saddle")]
    PigSaddle,
    #[strum(serialize = "entity.pig.step")]
    PigStep,
    #[strum(serialize = "entity.piglin.admiring_item")]
    PiglinAdmiringItem,
    #[strum(serialize = "entity.piglin.ambient")]
    PiglinAmbient,
    #[strum(serialize = "entity.piglin.angry")]
    PiglinAngry,
    #[strum(serialize = "entity.piglin.celebrate")]
    PiglinCelebrate,
    #[strum(serialize = "entity.piglin.death")]
    PiglinDeath,
    #[strum(serialize = "entity.piglin.jealous")]
    PiglinJealous,
    #[strum(serialize = "entity.piglin.hurt")]
    PiglinHurt,
    #[strum(serialize = "entity.piglin.retreat")]
    PiglinRetreat,
    #[strum(serialize = "entity.piglin.step")]
    PiglinStep,
    #[strum(serialize = "entity.piglin.converted_to_zombified")]
    PiglinConvertedToZombified,
    #[strum(serialize = "entity.piglin_brute.ambient")]
    PiglinBruteAmbient,
    #[strum(serialize = "entity.piglin_brute.angry")]
    PiglinBruteAngry,
    #[strum(serialize = "entity.piglin_brute.death")]
    PiglinBruteDeath,
    #[strum(serialize = "entity.piglin_brute.hurt")]
    PiglinBruteHurt,
    #[strum(serialize = "entity.piglin_brute.step")]
    PiglinBruteStep,
    #[strum(serialize = "entity.piglin_brute.converted_to_zombified")]
    PiglinBruteConvertedToZombified,
    #[strum(serialize = "entity.pillager.ambient")]
    PillagerAmbient,
    #[strum(serialize = "entity.pillager.celebrate")]
    PillagerCelebrate,
    #[strum(serialize = "entity.pillager.death")]
    PillagerDeath,
    #[strum(serialize = "entity.pillager.hurt")]
    PillagerHurt,
    #[strum(serialize = "block.piston.contract")]
    PistonContract,
    #[strum(serialize = "block.piston.extend")]
    PistonExtend,
    #[strum(serialize = "entity.player.attack.crit")]
    PlayerAttackCrit,
    #[strum(serialize = "entity.player.attack.knockback")]
    PlayerAttackKnockback,
    #[strum(serialize = "entity.player.attack.nodamage")]
    PlayerAttackNodamage,
    #[strum(serialize = "entity.player.attack.strong")]
    PlayerAttackStrong,
    #[strum(serialize = "entity.player.attack.sweep")]
    PlayerAttackSweep,
    #[strum(serialize = "entity.player.attack.weak")]
    PlayerAttackWeak,
    #[strum(serialize = "entity.player.big_fall")]
    PlayerBigFall,
    #[strum(serialize = "entity.player.breath")]
    PlayerBreath,
    #[strum(serialize = "entity.player.burp")]
    PlayerBurp,
    #[strum(serialize = "entity.player.death")]
    PlayerDeath,
    #[strum(serialize = "entity.player.hurt")]
    PlayerHurt,
    #[strum(serialize = "entity.player.hurt_drown")]
    PlayerHurtDrown,
    #[strum(serialize = "entity.player.hurt_freeze")]
    PlayerHurtFreeze,
    #[strum(serialize = "entity.player.hurt_on_fire")]
    PlayerHurtOnFire,
    #[strum(serialize = "entity.player.hurt_sweet_berry_bush")]
    PlayerHurtSweetBerryBush,
    #[strum(serialize = "entity.player.levelup")]
    PlayerLevelup,
    #[strum(serialize = "entity.player.small_fall")]
    PlayerSmallFall,
    #[strum(serialize = "entity.player.splash")]
    PlayerSplash,
    #[strum(serialize = "entity.player.splash.high_speed")]
    PlayerSplashHighSpeed,
    #[strum(serialize = "entity.player.swim")]
    PlayerSwim,
    #[strum(serialize = "entity.polar_bear.ambient")]
    PolarBearAmbient,
    #[strum(serialize = "entity.polar_bear.ambient_baby")]
    PolarBearAmbientBaby,
    #[strum(serialize = "entity.polar_bear.death")]
    PolarBearDeath,
    #[strum(serialize = "entity.polar_bear.hurt")]
    PolarBearHurt,
    #[strum(serialize = "entity.polar_bear.step")]
    PolarBearStep,
    #[strum(serialize = "entity.polar_bear.warning")]
    PolarBearWarning,
    #[strum(serialize = "block.polished_deepslate.break")]
    PolishedDeepslateBreak,
    #[strum(serialize = "block.polished_deepslate.fall")]
    PolishedDeepslateFall,
    #[strum(serialize = "block.polished_deepslate.hit")]
    PolishedDeepslateHit,
    #[strum(serialize = "block.polished_deepslate.place")]
    PolishedDeepslatePlace,
    #[strum(serialize = "block.polished_deepslate.step")]
    PolishedDeepslateStep,
    #[strum(serialize = "block.portal.ambient")]
    PortalAmbient,
    #[strum(serialize = "block.portal.travel")]
    PortalTravel,
    #[strum(serialize = "block.portal.trigger")]
    PortalTrigger,
    #[strum(serialize = "block.powder_snow.break")]
    PowderSnowBreak,
    #[strum(serialize = "block.powder_snow.fall")]
    PowderSnowFall,
    #[strum(serialize = "block.powder_snow.hit")]
    PowderSnowHit,
    #[strum(serialize = "block.powder_snow.place")]
    PowderSnowPlace,
    #[strum(serialize = "block.powder_snow.step")]
    PowderSnowStep,
    #[strum(serialize = "entity.puffer_fish.ambient")]
    PufferFishAmbient,
    #[strum(serialize = "entity.puffer_fish.blow_out")]
    PufferFishBlowOut,
    #[strum(serialize = "entity.puffer_fish.blow_up")]
    PufferFishBlowUp,
    #[strum(serialize = "entity.puffer_fish.death")]
    PufferFishDeath,
    #[strum(serialize = "entity.puffer_fish.flop")]
    PufferFishFlop,
    #[strum(serialize = "entity.puffer_fish.hurt")]
    PufferFishHurt,
    #[strum(serialize = "entity.puffer_fish.sting")]
    PufferFishSting,
    #[strum(serialize = "block.pumpkin.carve")]
    PumpkinCarve,
    #[strum(serialize = "entity.rabbit.ambient")]
    RabbitAmbient,
    #[strum(serialize = "entity.rabbit.attack")]
    RabbitAttack,
    #[strum(serialize = "entity.rabbit.death")]
    RabbitDeath,
    #[strum(serialize = "entity.rabbit.hurt")]
    RabbitHurt,
    #[strum(serialize = "entity.rabbit.jump")]
    RabbitJump,
    #[strum(serialize = "event.raid.horn")]
    RaidHorn,
    #[strum(serialize = "entity.ravager.ambient")]
    RavagerAmbient,
    #[strum(serialize = "entity.ravager.attack")]
    RavagerAttack,
    #[strum(serialize = "entity.ravager.celebrate")]
    RavagerCelebrate,
    #[strum(serialize = "entity.ravager.death")]
    RavagerDeath,
    #[strum(serialize = "entity.ravager.hurt")]
    RavagerHurt,
    #[strum(serialize = "entity.ravager.step")]
    RavagerStep,
    #[strum(serialize = "entity.ravager.stunned")]
    RavagerStunned,
    #[strum(serialize = "entity.ravager.roar")]
    RavagerRoar,
    #[strum(serialize = "block.nether_gold_ore.break")]
    NetherGoldOreBreak,
    #[strum(serialize = "block.nether_gold_ore.fall")]
    NetherGoldOreFall,
    #[strum(serialize = "block.nether_gold_ore.hit")]
    NetherGoldOreHit,
    #[strum(serialize = "block.nether_gold_ore.place")]
    NetherGoldOrePlace,
    #[strum(serialize = "block.nether_gold_ore.step")]
    NetherGoldOreStep,
    #[strum(serialize = "block.nether_ore.break")]
    NetherOreBreak,
    #[strum(serialize = "block.nether_ore.fall")]
    NetherOreFall,
    #[strum(serialize = "block.nether_ore.hit")]
    NetherOreHit,
    #[strum(serialize = "block.nether_ore.place")]
    NetherOrePlace,
    #[strum(serialize = "block.nether_ore.step")]
    NetherOreStep,
    #[strum(serialize = "block.redstone_torch.burnout")]
    RedstoneTorchBurnout,
    #[strum(serialize = "block.respawn_anchor.ambient")]
    RespawnAnchorAmbient,
    #[strum(serialize = "block.respawn_anchor.charge")]
    RespawnAnchorCharge,
    #[strum(serialize = "block.respawn_anchor.deplete")]
    RespawnAnchorDeplete,
    #[strum(serialize = "block.respawn_anchor.set_spawn")]
    RespawnAnchorSetSpawn,
    #[strum(serialize = "block.rooted_dirt.break")]
    RootedDirtBreak,
    #[strum(serialize = "block.rooted_dirt.fall")]
    RootedDirtFall,
    #[strum(serialize = "block.rooted_dirt.hit")]
    RootedDirtHit,
    #[strum(serialize = "block.rooted_dirt.place")]
    RootedDirtPlace,
    #[strum(serialize = "block.rooted_dirt.step")]
    RootedDirtStep,
    #[strum(serialize = "entity.salmon.ambient")]
    SalmonAmbient,
    #[strum(serialize = "entity.salmon.death")]
    SalmonDeath,
    #[strum(serialize = "entity.salmon.flop")]
    SalmonFlop,
    #[strum(serialize = "entity.salmon.hurt")]
    SalmonHurt,
    #[strum(serialize = "block.sand.break")]
    SandBreak,
    #[strum(serialize = "block.sand.fall")]
    SandFall,
    #[strum(serialize = "block.sand.hit")]
    SandHit,
    #[strum(serialize = "block.sand.place")]
    SandPlace,
    #[strum(serialize = "block.sand.step")]
    SandStep,
    #[strum(serialize = "block.scaffolding.break")]
    ScaffoldingBreak,
    #[strum(serialize = "block.scaffolding.fall")]
    ScaffoldingFall,
    #[strum(serialize = "block.scaffolding.hit")]
    ScaffoldingHit,
    #[strum(serialize = "block.scaffolding.place")]
    ScaffoldingPlace,
    #[strum(serialize = "block.scaffolding.step")]
    ScaffoldingStep,
    #[strum(serialize = "block.sculk_sensor.clicking")]
    SculkClicking,
    #[strum(serialize = "block.sculk_sensor.clicking_stop")]
    SculkClickingStop,
    #[strum(serialize = "block.sculk_sensor.break")]
    SculkSensorBreak,
    #[strum(serialize = "block.sculk_sensor.fall")]
    SculkSensorFall,
    #[strum(serialize = "block.sculk_sensor.hit")]
    SculkSensorHit,
    #[strum(serialize = "block.sculk_sensor.place")]
    SculkSensorPlace,
    #[strum(serialize = "block.sculk_sensor.step")]
    SculkSensorStep,
    #[strum(serialize = "entity.sheep.ambient")]
    SheepAmbient,
    #[strum(serialize = "entity.sheep.death")]
    SheepDeath,
    #[strum(serialize = "entity.sheep.hurt")]
    SheepHurt,
    #[strum(serialize = "entity.sheep.shear")]
    SheepShear,
    #[strum(serialize = "entity.sheep.step")]
    SheepStep,
    #[strum(serialize = "item.shield.block")]
    ShieldBlock,
    #[strum(serialize = "item.shield.break")]
    ShieldBreak,
    #[strum(serialize = "block.shroomlight.break")]
    ShroomlightBreak,
    #[strum(serialize = "block.shroomlight.step")]
    ShroomlightStep,
    #[strum(serialize = "block.shroomlight.place")]
    ShroomlightPlace,
    #[strum(serialize = "block.shroomlight.hit")]
    ShroomlightHit,
    #[strum(serialize = "block.shroomlight.fall")]
    ShroomlightFall,
    #[strum(serialize = "item.shovel.flatten")]
    ShovelFlatten,
    #[strum(serialize = "entity.shulker.ambient")]
    ShulkerAmbient,
    #[strum(serialize = "block.shulker_box.close")]
    ShulkerBoxClose,
    #[strum(serialize = "block.shulker_box.open")]
    ShulkerBoxOpen,
    #[strum(serialize = "entity.shulker_bullet.hit")]
    ShulkerBulletHit,
    #[strum(serialize = "entity.shulker_bullet.hurt")]
    ShulkerBulletHurt,
    #[strum(serialize = "entity.shulker.close")]
    ShulkerClose,
    #[strum(serialize = "entity.shulker.death")]
    ShulkerDeath,
    #[strum(serialize = "entity.shulker.hurt")]
    ShulkerHurt,
    #[strum(serialize = "entity.shulker.hurt_closed")]
    ShulkerHurtClosed,
    #[strum(serialize = "entity.shulker.open")]
    ShulkerOpen,
    #[strum(serialize = "entity.shulker.shoot")]
    ShulkerShoot,
    #[strum(serialize = "entity.shulker.teleport")]
    ShulkerTeleport,
    #[strum(serialize = "entity.silverfish.ambient")]
    SilverfishAmbient,
    #[strum(serialize = "entity.silverfish.death")]
    SilverfishDeath,
    #[strum(serialize = "entity.silverfish.hurt")]
    SilverfishHurt,
    #[strum(serialize = "entity.silverfish.step")]
    SilverfishStep,
    #[strum(serialize = "entity.skeleton.ambient")]
    SkeletonAmbient,
    #[strum(serialize = "entity.skeleton.converted_to_stray")]
    SkeletonConvertedToStray,
    #[strum(serialize = "entity.skeleton.death")]
    SkeletonDeath,
    #[strum(serialize = "entity.skeleton_horse.ambient")]
    SkeletonHorseAmbient,
    #[strum(serialize = "entity.skeleton_horse.death")]
    SkeletonHorseDeath,
    #[strum(serialize = "entity.skeleton_horse.hurt")]
    SkeletonHorseHurt,
    #[strum(serialize = "entity.skeleton_horse.swim")]
    SkeletonHorseSwim,
    #[strum(serialize = "entity.skeleton_horse.ambient_water")]
    SkeletonHorseAmbientWater,
    #[strum(serialize = "entity.skeleton_horse.gallop_water")]
    SkeletonHorseGallopWater,
    #[strum(serialize = "entity.skeleton_horse.jump_water")]
    SkeletonHorseJumpWater,
    #[strum(serialize = "entity.skeleton_horse.step_water")]
    SkeletonHorseStepWater,
    #[strum(serialize = "entity.skeleton.hurt")]
    SkeletonHurt,
    #[strum(serialize = "entity.skeleton.shoot")]
    SkeletonShoot,
    #[strum(serialize = "entity.skeleton.step")]
    SkeletonStep,
    #[strum(serialize = "entity.slime.attack")]
    SlimeAttack,
    #[strum(serialize = "entity.slime.death")]
    SlimeDeath,
    #[strum(serialize = "entity.slime.hurt")]
    SlimeHurt,
    #[strum(serialize = "entity.slime.jump")]
    SlimeJump,
    #[strum(serialize = "entity.slime.squish")]
    SlimeSquish,
    #[strum(serialize = "block.slime_block.break")]
    SlimeBlockBreak,
    #[strum(serialize = "block.slime_block.fall")]
    SlimeBlockFall,
    #[strum(serialize = "block.slime_block.hit")]
    SlimeBlockHit,
    #[strum(serialize = "block.slime_block.place")]
    SlimeBlockPlace,
    #[strum(serialize = "block.slime_block.step")]
    SlimeBlockStep,
    #[strum(serialize = "block.small_amethyst_bud.break")]
    SmallAmethystBudBreak,
    #[strum(serialize = "block.small_amethyst_bud.place")]
    SmallAmethystBudPlace,
    #[strum(serialize = "block.small_dripleaf.break")]
    SmallDripleafBreak,
    #[strum(serialize = "block.small_dripleaf.fall")]
    SmallDripleafFall,
    #[strum(serialize = "block.small_dripleaf.hit")]
    SmallDripleafHit,
    #[strum(serialize = "block.small_dripleaf.place")]
    SmallDripleafPlace,
    #[strum(serialize = "block.small_dripleaf.step")]
    SmallDripleafStep,
    #[strum(serialize = "block.soul_sand.break")]
    SoulSandBreak,
    #[strum(serialize = "block.soul_sand.step")]
    SoulSandStep,
    #[strum(serialize = "block.soul_sand.place")]
    SoulSandPlace,
    #[strum(serialize = "block.soul_sand.hit")]
    SoulSandHit,
    #[strum(serialize = "block.soul_sand.fall")]
    SoulSandFall,
    #[strum(serialize = "block.soul_soil.break")]
    SoulSoilBreak,
    #[strum(serialize = "block.soul_soil.step")]
    SoulSoilStep,
    #[strum(serialize = "block.soul_soil.place")]
    SoulSoilPlace,
    #[strum(serialize = "block.soul_soil.hit")]
    SoulSoilHit,
    #[strum(serialize = "block.soul_soil.fall")]
    SoulSoilFall,
    #[strum(serialize = "particle.soul_escape")]
    SoulEscape,
    #[strum(serialize = "block.spore_blossom.break")]
    SporeBlossomBreak,
    #[strum(serialize = "block.spore_blossom.fall")]
    SporeBlossomFall,
    #[strum(serialize = "block.spore_blossom.hit")]
    SporeBlossomHit,
    #[strum(serialize = "block.spore_blossom.place")]
    SporeBlossomPlace,
    #[strum(serialize = "block.spore_blossom.step")]
    SporeBlossomStep,
    #[strum(serialize = "entity.strider.ambient")]
    StriderAmbient,
    #[strum(serialize = "entity.strider.happy")]
    StriderHappy,
    #[strum(serialize = "entity.strider.retreat")]
    StriderRetreat,
    #[strum(serialize = "entity.strider.death")]
    StriderDeath,
    #[strum(serialize = "entity.strider.hurt")]
    StriderHurt,
    #[strum(serialize = "entity.strider.step")]
    StriderStep,
    #[strum(serialize = "entity.strider.step_lava")]
    StriderStepLava,
    #[strum(serialize = "entity.strider.eat")]
    StriderEat,
    #[strum(serialize = "entity.strider.saddle")]
    StriderSaddle,
    #[strum(serialize = "entity.slime.death_small")]
    SlimeDeathSmall,
    #[strum(serialize = "entity.slime.hurt_small")]
    SlimeHurtSmall,
    #[strum(serialize = "entity.slime.jump_small")]
    SlimeJumpSmall,
    #[strum(serialize = "entity.slime.squish_small")]
    SlimeSquishSmall,
    #[strum(serialize = "block.smithing_table.use")]
    SmithingTableUse,
    #[strum(serialize = "block.smoker.smoke")]
    SmokerSmoke,
    #[strum(serialize = "entity.snowball.throw")]
    SnowballThrow,
    #[strum(serialize = "block.snow.break")]
    SnowBreak,
    #[strum(serialize = "block.snow.fall")]
    SnowFall,
    #[strum(serialize = "entity.snow_golem.ambient")]
    SnowGolemAmbient,
    #[strum(serialize = "entity.snow_golem.death")]
    SnowGolemDeath,
    #[strum(serialize = "entity.snow_golem.hurt")]
    SnowGolemHurt,
    #[strum(serialize = "entity.snow_golem.shoot")]
    SnowGolemShoot,
    #[strum(serialize = "entity.snow_golem.shear")]
    SnowGolemShear,
    #[strum(serialize = "block.snow.hit")]
    SnowHit,
    #[strum(serialize = "block.snow.place")]
    SnowPlace,
    #[strum(serialize = "block.snow.step")]
    SnowStep,
    #[strum(serialize = "entity.spider.ambient")]
    SpiderAmbient,
    #[strum(serialize = "entity.spider.death")]
    SpiderDeath,
    #[strum(serialize = "entity.spider.hurt")]
    SpiderHurt,
    #[strum(serialize = "entity.spider.step")]
    SpiderStep,
    #[strum(serialize = "entity.splash_potion.break")]
    SplashPotionBreak,
    #[strum(serialize = "entity.splash_potion.throw")]
    SplashPotionThrow,
    #[strum(serialize = "item.spyglass.use")]
    SpyglassUse,
    #[strum(serialize = "item.spyglass.stop_using")]
    SpyglassStopUsing,
    #[strum(serialize = "entity.squid.ambient")]
    SquidAmbient,
    #[strum(serialize = "entity.squid.death")]
    SquidDeath,
    #[strum(serialize = "entity.squid.hurt")]
    SquidHurt,
    #[strum(serialize = "entity.squid.squirt")]
    SquidSquirt,
    #[strum(serialize = "block.stone.break")]
    StoneBreak,
    #[strum(serialize = "block.stone_button.click_off")]
    StoneButtonClickOff,
    #[strum(serialize = "block.stone_button.click_on")]
    StoneButtonClickOn,
    #[strum(serialize = "block.stone.fall")]
    StoneFall,
    #[strum(serialize = "block.stone.hit")]
    StoneHit,
    #[strum(serialize = "block.stone.place")]
    StonePlace,
    #[strum(serialize = "block.stone_pressure_plate.click_off")]
    StonePressurePlateClickOff,
    #[strum(serialize = "block.stone_pressure_plate.click_on")]
    StonePressurePlateClickOn,
    #[strum(serialize = "block.stone.step")]
    StoneStep,
    #[strum(serialize = "entity.stray.ambient")]
    StrayAmbient,
    #[strum(serialize = "entity.stray.death")]
    StrayDeath,
    #[strum(serialize = "entity.stray.hurt")]
    StrayHurt,
    #[strum(serialize = "entity.stray.step")]
    StrayStep,
    #[strum(serialize = "block.sweet_berry_bush.break")]
    SweetBerryBushBreak,
    #[strum(serialize = "block.sweet_berry_bush.place")]
    SweetBerryBushPlace,
    #[strum(serialize = "block.sweet_berry_bush.pick_berries")]
    SweetBerryBushPickBerries,
    #[strum(serialize = "enchant.thorns.hit")]
    ThornsHit,
    #[strum(serialize = "entity.tnt.primed")]
    TntPrimed,
    #[strum(serialize = "item.totem.use")]
    TotemUse,
    #[strum(serialize = "item.trident.hit")]
    TridentHit,
    #[strum(serialize = "item.trident.hit_ground")]
    TridentHitGround,
    #[strum(serialize = "item.trident.return")]
    TridentReturn,
    #[strum(serialize = "item.trident.riptide_1")]
    TridentRiptide1,
    #[strum(serialize = "item.trident.riptide_2")]
    TridentRiptide2,
    #[strum(serialize = "item.trident.riptide_3")]
    TridentRiptide3,
    #[strum(serialize = "item.trident.throw")]
    TridentThrow,
    #[strum(serialize = "item.trident.thunder")]
    TridentThunder,
    #[strum(serialize = "block.tripwire.attach")]
    TripwireAttach,
    #[strum(serialize = "block.tripwire.click_off")]
    TripwireClickOff,
    #[strum(serialize = "block.tripwire.click_on")]
    TripwireClickOn,
    #[strum(serialize = "block.tripwire.detach")]
    TripwireDetach,
    #[strum(serialize = "entity.tropical_fish.ambient")]
    TropicalFishAmbient,
    #[strum(serialize = "entity.tropical_fish.death")]
    TropicalFishDeath,
    #[strum(serialize = "entity.tropical_fish.flop")]
    TropicalFishFlop,
    #[strum(serialize = "entity.tropical_fish.hurt")]
    TropicalFishHurt,
    #[strum(serialize = "block.tuff.break")]
    TuffBreak,
    #[strum(serialize = "block.tuff.step")]
    TuffStep,
    #[strum(serialize = "block.tuff.place")]
    TuffPlace,
    #[strum(serialize = "block.tuff.hit")]
    TuffHit,
    #[strum(serialize = "block.tuff.fall")]
    TuffFall,
    #[strum(serialize = "entity.turtle.ambient_land")]
    TurtleAmbientLand,
    #[strum(serialize = "entity.turtle.death")]
    TurtleDeath,
    #[strum(serialize = "entity.turtle.death_baby")]
    TurtleDeathBaby,
    #[strum(serialize = "entity.turtle.egg_break")]
    TurtleEggBreak,
    #[strum(serialize = "entity.turtle.egg_crack")]
    TurtleEggCrack,
    #[strum(serialize = "entity.turtle.egg_hatch")]
    TurtleEggHatch,
    #[strum(serialize = "entity.turtle.hurt")]
    TurtleHurt,
    #[strum(serialize = "entity.turtle.hurt_baby")]
    TurtleHurtBaby,
    #[strum(serialize = "entity.turtle.lay_egg")]
    TurtleLayEgg,
    #[strum(serialize = "entity.turtle.shamble")]
    TurtleShamble,
    #[strum(serialize = "entity.turtle.shamble_baby")]
    TurtleShambleBaby,
    #[strum(serialize = "entity.turtle.swim")]
    TurtleSwim,
    #[strum(serialize = "ui.button.click")]
    UiButtonClick,
    #[strum(serialize = "ui.loom.select_pattern")]
    UiLoomSelectPattern,
    #[strum(serialize = "ui.loom.take_result")]
    UiLoomTakeResult,
    #[strum(serialize = "ui.cartography_table.take_result")]
    UiCartographyTableTakeResult,
    #[strum(serialize = "ui.stonecutter.take_result")]
    UiStonecutterTakeResult,
    #[strum(serialize = "ui.stonecutter.select_recipe")]
    UiStonecutterSelectRecipe,
    #[strum(serialize = "ui.toast.challenge_complete")]
    UiToastChallengeComplete,
    #[strum(serialize = "ui.toast.in")]
    UiToastIn,
    #[strum(serialize = "ui.toast.out")]
    UiToastOut,
    #[strum(serialize = "entity.vex.ambient")]
    VexAmbient,
    #[strum(serialize = "entity.vex.charge")]
    VexCharge,
    #[strum(serialize = "entity.vex.death")]
    VexDeath,
    #[strum(serialize = "entity.vex.hurt")]
    VexHurt,
    #[strum(serialize = "entity.villager.ambient")]
    VillagerAmbient,
    #[strum(serialize = "entity.villager.celebrate")]
    VillagerCelebrate,
    #[strum(serialize = "entity.villager.death")]
    VillagerDeath,
    #[strum(serialize = "entity.villager.hurt")]
    VillagerHurt,
    #[strum(serialize = "entity.villager.no")]
    VillagerNo,
    #[strum(serialize = "entity.villager.trade")]
    VillagerTrade,
    #[strum(serialize = "entity.villager.yes")]
    VillagerYes,
    #[strum(serialize = "entity.villager.work_armorer")]
    VillagerWorkArmorer,
    #[strum(serialize = "entity.villager.work_butcher")]
    VillagerWorkButcher,
    #[strum(serialize = "entity.villager.work_cartographer")]
    VillagerWorkCartographer,
    #[strum(serialize = "entity.villager.work_cleric")]
    VillagerWorkCleric,
    #[strum(serialize = "entity.villager.work_farmer")]
    VillagerWorkFarmer,
    #[strum(serialize = "entity.villager.work_fisherman")]
    VillagerWorkFisherman,
    #[strum(serialize = "entity.villager.work_fletcher")]
    VillagerWorkFletcher,
    #[strum(serialize = "entity.villager.work_leatherworker")]
    VillagerWorkLeatherworker,
    #[strum(serialize = "entity.villager.work_librarian")]
    VillagerWorkLibrarian,
    #[strum(serialize = "entity.villager.work_mason")]
    VillagerWorkMason,
    #[strum(serialize = "entity.villager.work_shepherd")]
    VillagerWorkShepherd,
    #[strum(serialize = "entity.villager.work_toolsmith")]
    VillagerWorkToolsmith,
    #[strum(serialize = "entity.villager.work_weaponsmith")]
    VillagerWorkWeaponsmith,
    #[strum(serialize = "entity.vindicator.ambient")]
    VindicatorAmbient,
    #[strum(serialize = "entity.vindicator.celebrate")]
    VindicatorCelebrate,
    #[strum(serialize = "entity.vindicator.death")]
    VindicatorDeath,
    #[strum(serialize = "entity.vindicator.hurt")]
    VindicatorHurt,
    #[strum(serialize = "block.vine.break")]
    VineBreak,
    #[strum(serialize = "block.vine.fall")]
    VineFall,
    #[strum(serialize = "block.vine.hit")]
    VineHit,
    #[strum(serialize = "block.vine.place")]
    VinePlace,
    #[strum(serialize = "block.vine.step")]
    VineStep,
    #[strum(serialize = "block.lily_pad.place")]
    LilyPadPlace,
    #[strum(serialize = "entity.wandering_trader.ambient")]
    WanderingTraderAmbient,
    #[strum(serialize = "entity.wandering_trader.death")]
    WanderingTraderDeath,
    #[strum(serialize = "entity.wandering_trader.disappeared")]
    WanderingTraderDisappeared,
    #[strum(serialize = "entity.wandering_trader.drink_milk")]
    WanderingTraderDrinkMilk,
    #[strum(serialize = "entity.wandering_trader.drink_potion")]
    WanderingTraderDrinkPotion,
    #[strum(serialize = "entity.wandering_trader.hurt")]
    WanderingTraderHurt,
    #[strum(serialize = "entity.wandering_trader.no")]
    WanderingTraderNo,
    #[strum(serialize = "entity.wandering_trader.reappeared")]
    WanderingTraderReappeared,
    #[strum(serialize = "entity.wandering_trader.trade")]
    WanderingTraderTrade,
    #[strum(serialize = "entity.wandering_trader.yes")]
    WanderingTraderYes,
    #[strum(serialize = "block.water.ambient")]
    WaterAmbient,
    #[strum(serialize = "weather.rain")]
    WeatherRain,
    #[strum(serialize = "weather.rain.above")]
    WeatherRainAbove,
    #[strum(serialize = "block.wet_grass.break")]
    WetGrassBreak,
    #[strum(serialize = "block.wet_grass.fall")]
    WetGrassFall,
    #[strum(serialize = "block.wet_grass.hit")]
    WetGrassHit,
    #[strum(serialize = "block.wet_grass.place")]
    WetGrassPlace,
    #[strum(serialize = "block.wet_grass.step")]
    WetGrassStep,
    #[strum(serialize = "entity.witch.ambient")]
    WitchAmbient,
    #[strum(serialize = "entity.witch.celebrate")]
    WitchCelebrate,
    #[strum(serialize = "entity.witch.death")]
    WitchDeath,
    #[strum(serialize = "entity.witch.drink")]
    WitchDrink,
    #[strum(serialize = "entity.witch.hurt")]
    WitchHurt,
    #[strum(serialize = "entity.witch.throw")]
    WitchThrow,
    #[strum(serialize = "entity.wither.ambient")]
    WitherAmbient,
    #[strum(serialize = "entity.wither.break_block")]
    WitherBreakBlock,
    #[strum(serialize = "entity.wither.death")]
    WitherDeath,
    #[strum(serialize = "entity.wither.hurt")]
    WitherHurt,
    #[strum(serialize = "entity.wither.shoot")]
    WitherShoot,
    #[strum(serialize = "entity.wither_skeleton.ambient")]
    WitherSkeletonAmbient,
    #[strum(serialize = "entity.wither_skeleton.death")]
    WitherSkeletonDeath,
    #[strum(serialize = "entity.wither_skeleton.hurt")]
    WitherSkeletonHurt,
    #[strum(serialize = "entity.wither_skeleton.step")]
    WitherSkeletonStep,
    #[strum(serialize = "entity.wither.spawn")]
    WitherSpawn,
    #[strum(serialize = "entity.wolf.ambient")]
    WolfAmbient,
    #[strum(serialize = "entity.wolf.death")]
    WolfDeath,
    #[strum(serialize = "entity.wolf.growl")]
    WolfGrowl,
    #[strum(serialize = "entity.wolf.howl")]
    WolfHowl,
    #[strum(serialize = "entity.wolf.hurt")]
    WolfHurt,
    #[strum(serialize = "entity.wolf.pant")]
    WolfPant,
    #[strum(serialize = "entity.wolf.shake")]
    WolfShake,
    #[strum(serialize = "entity.wolf.step")]
    WolfStep,
    #[strum(serialize = "entity.wolf.whine")]
    WolfWhine,
    #[strum(serialize = "block.wooden_door.close")]
    WoodenDoorClose,
    #[strum(serialize = "block.wooden_door.open")]
    WoodenDoorOpen,
    #[strum(serialize = "block.wooden_trapdoor.close")]
    WoodenTrapdoorClose,
    #[strum(serialize = "block.wooden_trapdoor.open")]
    WoodenTrapdoorOpen,
    #[strum(serialize = "block.wood.break")]
    WoodBreak,
    #[strum(serialize = "block.wooden_button.click_off")]
    WoodenButtonClickOff,
    #[strum(serialize = "block.wooden_button.click_on")]
    WoodenButtonClickOn,
    #[strum(serialize = "block.wood.fall")]
    WoodFall,
    #[strum(serialize = "block.wood.hit")]
    WoodHit,
    #[strum(serialize = "block.wood.place")]
    WoodPlace,
    #[strum(serialize = "block.wooden_pressure_plate.click_off")]
    WoodenPressurePlateClickOff,
    #[strum(serialize = "block.wooden_pressure_plate.click_on")]
    WoodenPressurePlateClickOn,
    #[strum(serialize = "block.wood.step")]
    WoodStep,
    #[strum(serialize = "block.wool.break")]
    WoolBreak,
    #[strum(serialize = "block.wool.fall")]
    WoolFall,
    #[strum(serialize = "block.wool.hit")]
    WoolHit,
    #[strum(serialize = "block.wool.place")]
    WoolPlace,
    #[strum(serialize = "block.wool.step")]
    WoolStep,
    #[strum(serialize = "entity.zoglin.ambient")]
    ZoglinAmbient,
    #[strum(serialize = "entity.zoglin.angry")]
    ZoglinAngry,
    #[strum(serialize = "entity.zoglin.attack")]
    ZoglinAttack,
    #[strum(serialize = "entity.zoglin.death")]
    ZoglinDeath,
    #[strum(serialize = "entity.zoglin.hurt")]
    ZoglinHurt,
    #[strum(serialize = "entity.zoglin.step")]
    ZoglinStep,
    #[strum(serialize = "entity.zombie.ambient")]
    ZombieAmbient,
    #[strum(serialize = "entity.zombie.attack_wooden_door")]
    ZombieAttackWoodenDoor,
    #[strum(serialize = "entity.zombie.attack_iron_door")]
    ZombieAttackIronDoor,
    #[strum(serialize = "entity.zombie.break_wooden_door")]
    ZombieBreakWoodenDoor,
    #[strum(serialize = "entity.zombie.converted_to_drowned")]
    ZombieConvertedToDrowned,
    #[strum(serialize = "entity.zombie.death")]
    ZombieDeath,
    #[strum(serialize = "entity.zombie.destroy_egg")]
    ZombieDestroyEgg,
    #[strum(serialize = "entity.zombie_horse.ambient")]
    ZombieHorseAmbient,
    #[strum(serialize = "entity.zombie_horse.death")]
    ZombieHorseDeath,
    #[strum(serialize = "entity.zombie_horse.hurt")]
    ZombieHorseHurt,
    #[strum(serialize = "entity.zombie.hurt")]
    ZombieHurt,
    #[strum(serialize = "entity.zombie.infect")]
    ZombieInfect,
    #[strum(serialize = "entity.zombified_piglin.ambient")]
    ZombifiedPiglinAmbient,
    #[strum(serialize = "entity.zombified_piglin.angry")]
    ZombifiedPiglinAngry,
    #[strum(serialize = "entity.zombified_piglin.death")]
    ZombifiedPiglinDeath,
    #[strum(serialize = "entity.zombified_piglin.hurt")]
    ZombifiedPiglinHurt,
    #[strum(serialize = "entity.zombie.step")]
    ZombieStep,
    #[strum(serialize = "entity.zombie_villager.ambient")]
    ZombieVillagerAmbient,
    #[strum(serialize = "entity.zombie_villager.converted")]
    ZombieVillagerConverted,
    #[strum(serialize = "entity.zombie_villager.cure")]
    ZombieVillagerCure,
    #[strum(serialize = "entity.zombie_villager.death")]
    ZombieVillagerDeath,
    #[strum(serialize = "entity.zombie_villager.hurt")]
    ZombieVillagerHurt,
    #[strum(serialize = "entity.zombie_villager.step")]
    ZombieVillagerStep,
}
