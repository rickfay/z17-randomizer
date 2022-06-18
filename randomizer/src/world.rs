use std::collections::HashMap;

use crate::check::Check;
use crate::{FillerItem, LocationInfo, regions};
use crate::FillerItem::*;
use crate::location::Location;
use crate::location::Location::*;
use crate::location_node::LocationNode;
use crate::path::Path;
use crate::progress::Progress;

/// Build the World Graph
pub fn build_world_graph() -> HashMap<Location, LocationNode> {
    HashMap::from([

        // --- Hyrule --- //

        (HyruleField, location("Hyrule Field", vec![
            check_free(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Dampe")),
            check(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Pegs"), |p| p.has_hammer()),
            check(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Behind Blacksmith"), |p| p.can_merge()),
            check(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Castle Rocks"), |p| p.has_power_glove()),
            check(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Woods"), |p| p.has_pendant_of_courage()),
            check(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Ledge"), |p| p.can_merge()),

            // Lake Hylia
            check(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lake Hylia Ledge Chest"), |p| p.can_merge()),
            check(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Shore"), |p| p.has_flippers()),
            check(LocationInfo::new(regions::hyrule::lake::hotfoot::SUBREGION, "Hyrule Hotfoot"), |p| p.has_boots()),
            check(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Bird Lover"), |p| p.has_flippers()),

            // Kakariko Village
            check_free(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Merchant (Left)")),
            check_free(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Merchant (Right)")),
            check(LocationInfo::new(regions::hyrule::kakariko::shady_guy::SUBREGION, "Shady Guy"), |p| p.can_merge() || p.has_boots()),
            check_free(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Cucco Ranch")),
            check_free(LocationInfo::new(regions::hyrule::field::rupee_rush::SUBREGION, "Rupee Rush")),

            // Eastern Ruins
            check(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Pegs (South)"), |p| p.has_hammer()),
        ], vec![
            path_free(RavioShop),
            path(EasternRuinsUpper, |p| p.can_hit_far_switch() || p.has_ice_rod() || p.can_merge()),
            path(EasternRuinsEastLedge, |p| p.has_power_glove()),
            path(WitchCave, |p| p.has_bombs()),
            path(ZoraDomainArea, |p| p.can_merge()),
            path(WaterfallCaveShallowWater, |p| p.has_flippers()),
            path_free(BlacksmithHouse),
            path(BlacksmithCave, |p| p.has_titans_mitt()),
            path_free(LostWoods),
            path(HyruleCastleCourtyard, |p| p.has_master_sword()),
            path_free(FortuneTeller),
            path_free(KakarikoJailCell),
            path(WellUpper, |p| p.has_power_glove()),
            path_free(WellLower),
            path_free(MilkBar),
            path_free(BeeGuyHouse),
            path_free(KakarikoItemShop),
            path_free(LakesideItemShop),
            path(ItemSellerCave, |p| p.has_bombs()),
            path(FlippersDungeon, |p| p.has_titans_mitt()),
            path(SouthernRuinsBombCave, |p| p.has_bombs()),
            path_free(LakeDarkCave),
            path(IceRodCave, |p| p.has_bombs()),
            path(Sanctuary, |p| p.has_sword() || p.has_bombs() || p.has_fire_rod() || p.has_ice_rod() || p.has_lamp() || p.has_boots()),
            path(MoldormCave, |p| p.has_power_glove()),
            path(RossoHouse, |p| p.has_pendant_of_courage()),
            path(RossoCave, |p| p.has_hammer()),
            path(TornadoRodDungeon, |p| p.has_bombs()),
            path(HouseOfGalesIsland, |p| p.has_flippers()),
            path(HauntedGroveLedge, |p| p.can_merge()),
            path(LoruleLakeWest, |p| p.can_merge()),
            path(LoruleLakeEast, |p| p.can_merge()),
            path(MiseryMire, |p| p.can_merge()),
            path(SkullWoodsOverworld, |p| p.can_merge()),
        ])),
        (EasternRuinsUpper, location("Eastern Ruins Upper", vec![
            check_free(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Armos Chest")),
            check(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Hookshot Chest"), |p| p.has_hookshot()),
            check(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Chest"), |p| p.can_merge()),
        ], vec![
            path_free(HyruleField),
            path(EasternRuinsEastLedge, |p| p.can_merge()),
            path_free(EasternPalaceFoyer),
            path_free(MergeDungeon),
            path(WitchCave, |p| p.has_bombs()),
        ])),
        (EasternRuinsEastLedge, location("Eastern Ruins East Ledge", vec![], vec![
            path(EastRuinsBombCaveUpper, |p| p.has_bombs()),
            path(EasternRuinsUpper, |p| p.can_merge()),
            path_free(HyruleField),
        ])),
        (WitchCave, location("Witch Cave", vec![], vec![
            path_free(EasternRuinsUpper),
            path_free(HyruleField),
        ])),
        (RavioShop, location("Ravio's Shop", vec![
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (1)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (2)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (3)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (4)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (5)")),
            check(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (6)"), |p| p.has_sage_osfala()),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (7)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (8)")),
            check_free(LocationInfo::new(regions::hyrule::field::rentals::SUBREGION, "Ravio (9)")),
        ], vec![
            path_free(HyruleField),
        ])),
        (ZoraDomain, location("Zora's Domain", vec![
            check(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Zora Queen"), |p| p.has_smooth_gem()),
        ], vec![
            path_free(ZoraDomainArea),
        ])),
        (ZoraDomainArea, location("Zora's Domain Area", vec![
            check(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Zora's Domain Ledge Chest"), |p| p.can_merge()),
        ], vec![
            path_free(HyruleField),
            path_free(ZoraDomain),
            path(KusDomain, |p| p.can_merge()),
            path(WaterfallCaveShallowWater, inaccessible),
        ])),
        (WaterfallCaveShallowWater, location("Waterfall Cave Shallow Water", vec![], vec![
            path_free(WaterfallCave),
            path(HyruleField, |p| p.has_flippers()),
        ])),
        (WaterfallCave, location("Waterfall Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::zoras::domain::SUBREGION, "Behind Waterfall")),
        ], vec![
            path_free(WaterfallCaveShallowWater),
        ])),
        (MergeDungeon, location("Eastern Ruins Treasure Dungeon", vec![
            check(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Merge Treasure Dungeon"), |p| p.can_merge()),
        ], vec![
            path_free(EasternRuinsUpper),
        ])),
        (EastRuinsBombCaveUpper, location("Eastern Ruins Bomb Cave Upper", vec![
            check(LocationInfo::new(regions::hyrule::eastern::hill::SUBREGION, "Cave"), |p| p.can_merge()),
        ], vec![
            path(EastRuinsBombCaveLower, inaccessible),
            path_free(EasternRuinsUpper),
        ])),
        (EastRuinsBombCaveLower, location("Eastern Ruins Bomb Cave Lower", vec![], vec![
            path_free(HyruleField),
        ])),
        (HouseOfGalesIsland, location("House of Gales Island", vec![], vec![
            path(HyruleField, |p| p.has_flippers()),
            path(HouseOfGalesFoyer, |p| p.has_tornado_rod()),
        ])),
        (RossoHouse, location("Rosso's House", vec![
            check_free(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Rosso")),
            check(LocationInfo::new(regions::hyrule::field::post_eastern::SUBREGION, "Clean Rocks"), |p| p.has_power_glove()),
        ], vec![
            path_free(HyruleField),
            path(SkullWoodsOverworld, |p| p.can_merge()),
        ])),
        (RossoCave, location("Rosso Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Rosso Cave")),
        ], vec![
            path_free(HyruleField),
        ])),
        (TornadoRodDungeon, location("Sanctuary Treasure Dungeon", vec![
            check(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Sanctuary Treasure Dungeon"), |p| p.can_merge()),
        ], vec![
            path_free(HyruleField),
        ])),
        (GraveyardLedge, location("Graveyard Ledge", vec![], vec![
            path_free(HyruleField),
            path_free(GraveyardLedgeCave),
            path(LoruleGraveyard, |p| p.can_merge()),
        ])),
        (GraveyardLedgeCave, location("Graveyard Ledge Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::field::sanctuary_cave::SUBREGION, "Sanctuary Cave")),
        ], vec![
            path_free(GraveyardLedge),
        ])),
        (BlacksmithHouse, location("Blacksmith's House (Hyrule)", vec![
            check(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"), |p| p.has_master_ore(2)),
        ], vec![
            path_free(HyruleField),
        ])),
        (BlacksmithCave, location("Blacksmith Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Cave")),
        ], vec![
            path_free(HyruleField),
        ])),


        // Hyrule Castle
        (HyruleCastleCourtyard, location("Hyrule Castle Courtyard", vec![], vec![
            path_free(HyruleCastleLeftRoom),
            path_free(HyruleCastleRightRoom),
            path_free(HyruleCastleInterior),
            path(HyruleField, |p| p.has_master_sword()),
        ])),
        (HyruleCastleInterior, location("Hyrule Castle Interior", vec![], vec![
            path_free(HyruleCastleCourtyard),
            path_free(HyruleCastleRoof),
        ])),
        (HyruleCastleRightRoom, location("Hyrule Castle Right Room", vec![], vec![
            path_free(HyruleCastleCourtyard),
        ])),
        (HyruleCastleLeftRoom, location("Hyrule Castle Left Room", vec![
            check_free(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle (Indoors)")),
        ], vec![
            path_free(HyruleCastleCourtyard),
        ])),
        (HyruleCastleRoof, location("Hyrule Castle Roof", vec![
            check_free(LocationInfo::new(regions::hyrule::field::castle::SUBREGION, "Castle Balcony")),
        ], vec![
            path_free(HyruleField),
            path_free(HyruleCastleCourtyard),
            path_free(HyruleCastleInterior),
            path_free(HyruleCastleDungeon),
        ])),
        (LostWoods, location("Lost Woods", vec![
            check(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Alcove"), |p| p.can_merge()),
            check(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Lost Woods Big Rock Chest"), |p| p.has_titans_mitt()),
        ], vec![
            path_free(HyruleField),
            path(MasterSwordArea, |p| p.has_all_pendants()),
        ])),
        (MasterSwordArea, location("Master Sword Area", vec![
            check_free(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Pedestal")),
        ], vec![
            path_free(LostWoods),
        ])),
        (FortuneTeller, location("Fortune-Teller (Hyrule)", vec![
            check_free(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Fortune Teller")),
        ], vec![
            path_free(HyruleField),
        ])),
        (KakarikoJailCell, location("Kakariko Jail Cell", vec![
            check(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Jail"), |p| p.can_merge()),
        ], vec![
            path_free(HyruleField),
        ])),
        (WellUpper, location("Kakariko Well Upper", vec![
            check_free(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Upper)")),
        ], vec![
            path_free(WellLower),
        ])),
        (WellLower, location("Kakariko Well Lower", vec![
            check_free(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Well (Chest)")),
        ], vec![
            path_free(HyruleField),
        ])),
        (StylishWomanHouse, location("Stylish Woman's House", vec![
            check_free(LocationInfo::new(regions::hyrule::kakariko::closed::SUBREGION, "Stylish Woman")),
            check_quest_free("Open Stylish Woman's House", StylishWomansHouseOpen),
        ], vec![
            path(LoruleCastleField, |p| p.can_merge()),
            path_free(HyruleField),
        ])),
        (MilkBar, location("Milk Bar", vec![
            check(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Milk Bar Owner"), |p| p.has_letter_in_a_bottle()),
        ], vec![
            path_free(HyruleField),
        ])),
        (BeeGuyHouse, location("Bee Guy's House", vec![
            check(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy"), |p| p.has_bottle()),
            check(LocationInfo::new(regions::hyrule::kakariko::post_sanc::SUBREGION, "Bee Guy (Golden Bee)"), |p| p.has_gold_bee()),
        ], vec![
            path_free(HyruleField),
        ])),
        (KakarikoItemShop, location("Kakariko Item Shop", vec![
            check_quest_free("Kakariko Item Shop Scoot Fruit", ScootFruit),
        ], vec![
            path_free(HyruleField),
        ])),
        (LakesideItemShop, location("Lakeside Item Shop", vec![
            check_quest_free("Lakeside Item Shop Scoot Fruit", ScootFruit),
        ], vec![
            path_free(HyruleField),
        ])),
        (ItemSellerCave, location("Runaway Item-Seller Cave", vec![
            check(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Runaway Item Seller"), |p| p.has_scoot_fruit()),
        ], vec![
            path_free(HyruleField),
        ])),
        (FlippersDungeon, location("Southern Ruins Treasure Dungeon", vec![
            check(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Treasure Room"), |p| p.has_boomerang() && p.has_hookshot() && p.has_flippers()),
        ], vec![
            path_free(HyruleField),
        ])),
        (SouthernRuinsBombCave, location("Southern Ruins Bomb Cave", vec![], vec![
            path_free(HyruleField),
            path_free(SouthernRuinsPillars),
        ])),
        (SouthernRuinsPillars, location("Southern Ruins Pillars", vec![
            check_free(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Behind Pillars")),
        ], vec![
            path_free(SouthernRuinsBombCave),
        ])),
        (LakeDarkCave, location("Lake Hylia Dark Cave", vec![
            check(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Torch Cave"), |p| p.has_fire_source()),
        ], vec![
            path_free(HyruleField),
        ])),
        (IceRodCave, location("Ice Rod Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Secret Cave")),
        ], vec![
            path_free(HyruleField),
        ])),
        (SanctuaryChurch, location("Sanctuary Church", vec![], vec![
            path(LoruleSanctuaryCaveLower, |p| p.can_merge()),
            path_free(HyruleField),
        ])),
        (Sanctuary, location("Sanctuary", vec![
            check_free(LocationInfo::new(regions::hyrule::sanctuary::lobby::SUBREGION, "[HS] Entrance")),
            check(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Lower Chest"), |p| p.has_lamp()),
            check(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Upper Chest"), |p| p.has_lamp()),
            check(LocationInfo::new(regions::hyrule::sanctuary::inside::SUBREGION, "[HS] Ledge"), |p| p.has_lamp() && p.can_merge()),
        ], vec![
            path_free(HyruleField),
            path(SanctuaryChurch, |p| p.has_lamp() && p.can_attack() && p.has_sanctuary_key()),
        ])),
        (MoldormCave, location("Moldorm Cave", vec![], vec![
            path_free(HyruleField),
            path(MoldormCaveTop, |p| p.has_titans_mitt()),
            path_free(DeathMountainBase),
        ])),
        (MoldormCaveTop, location("Moldorm Cave Top", vec![], vec![
            path_free(MoldormLedge),
            path(MoldormCave, |p| p.has_titans_mitt()),
        ])),
        (MoldormLedge, location("Moldorm Ledge", vec![], vec![
            path_free(MoldormCaveTop),
            path_free(HyruleField),
        ])),
        (DeathMountainBase, location("Death Mountain Base", vec![], vec![
            path_free(MoldormCave),
            path(DeathBombCave, |p| p.can_merge() && p.has_bombs()),
            path_free(DeathWeatherVaneCaveLeft),
            path(DeathFairyCave, |p| p.can_merge()),
            path_free(DonkeyCaveLower),
        ])),
        (DeathBombCave, location("Death Mountain Bomb Cave", vec![
            check_free(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Blocked Cave")),
        ], vec![
            path_free(DeathMountainBase),
        ])),
        (DeathWeatherVaneCaveLeft, location("Death Mountain Cave Left of Weather Vane", vec![
            check_free(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "First Cave")),
        ], vec![
            path_free(DeathMountainBase),
        ])),
        (DeathFairyCave, location("Death Mountain Fairy Cave", vec![
            check(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Fairy Cave"), |p| p.has_hammer() || p.has_bombs()),
        ], vec![
            path_free(DeathMountainBase),
        ])),
        (DonkeyCaveLower, location("Donkey Cave Lower", vec![], vec![
            path_free(DeathMountainBase),
            path(DonkeyCaveUpper, |p| p.can_merge()),
        ])),
        (DonkeyCaveUpper, location("Donkey Cave Upper", vec![
            check(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Pegs)"), |p| p.has_hammer()),
        ], vec![
            path(DonkeyCaveLower, |p| p.can_merge()),
            path_free(DeathWestLedge),
            path_free(DeathSecondFloor),
        ])),
        (DeathWestLedge, location("Death Mountain West Ledge", vec![
            check_free(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Death Mountain West Ledge")),
        ], vec![
            path_free(DonkeyCaveUpper),
            path_free(DeathSecondFloor),
        ])),
        (DeathSecondFloor, location("Death Mountain Second Floor", vec![], vec![
            path_free(DonkeyCaveUpper),
            path_free(AmidaCaveLower),
            path_free(DeathMountainBase),
            path(DeathFairyCave, inaccessible),
        ])),
        (AmidaCaveLower, location("Amida Cave Lower", vec![], vec![
            path_free(DeathSecondFloor),
            path_free(DeathThirdFloor),
            path(AmidaCaveUpper, inaccessible),
        ])),
        (DeathThirdFloor, location("Death Mountain Third Floor", vec![], vec![
            path_free(AmidaCaveLower),
            path_free(AmidaCaveUpper),
            path_free(DeathSecondFloor),
        ])),
        (AmidaCaveUpper, location("Amida Cave Upper", vec![
            check_free(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Rock Cave (Top)")),
        ], vec![
            path_free(AmidaCaveLower),
            path_free(DeathThirdFloor),
            path_free(DeathTopLeftLedge),
        ])),
        (DeathTopLeftLedge, location("Death Mountain West Top Left Ledge", vec![], vec![
            path_free(AmidaCaveUpper),
            path_free(DeathThirdFloor),
            path(SpectacleRock, |p| p.can_merge()), // noobs don't realize you can just jump here
            path(DeathMountainWestTop, |p| p.can_merge()),
        ])),
        (SpectacleRock, location("Spectacle Rock", vec![
            check_free(LocationInfo::new(regions::hyrule::death::upper::SUBREGION, "Spectacle Rock")),
        ], vec![
            path_free(DeathThirdFloor),
            path_free(SpectacleRockCaveLeft),
        ])),
        (SpectacleRockCaveLeft, location("Spectacle Rock Cave Left", vec![], vec![
            path_free(SpectacleRock),
            path_free(SpectacleRockCaveRight),
        ])),
        (SpectacleRockCaveRight, location("Spectacle Rock Cave Right", vec![], vec![
            path_free(DeathMountainWestTop),
        ])),
        (DeathMountainWestTop, location("Death Mountain West Top", vec![], vec![
            path_free(SpectacleRockCaveRight),
            path(TowerOfHeraFoyer, |p| p.has_hammer()),
            path(DeathTopLeftLedge, |p| p.can_merge()),
            path(SpectacleRock, |p| p.can_merge()), // noobs don't realize you can just jump here
            path_free(DeathThirdFloor),
            path(DeathMountainEastTop, |p| p.has_hookshot()),
        ])),
        (DeathMountainEastTop, location("Death Mountain East Top", vec![], vec![
            path(DeathMountainWestTop, |p| p.has_hookshot()),
            path_free(FireCaveTop),
            path_free(HookshotDungeon),
            path(BoulderingLedgeRight, inaccessible),
            path(RossosOreMine, inaccessible),
        ])),
        (HookshotDungeon, location("Hookshot Treasure Dungeon", vec![
            check(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Hookshot Treasure Dungeon"), |p| p.has_hookshot()),
        ], vec![
            path_free(DeathMountainEastTop),
        ])),
        (FireCaveTop, location("Fire Cave Top", vec![
            check(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Fire Cave Pillar"), |p| p.can_merge() && p.has_hammer()),
        ], vec![
            path_free(DeathMountainEastTop),
            path(FireCaveMiddle, |p| p.can_merge()),
        ])),
        (FireCaveMiddle, location("Fire Cave Middle", vec![], vec![
            path(FireCaveBottom, |p| p.can_merge()),
            path_free(BoulderingLedgeLeft),
            path_free(BoulderingLedgeRight),
        ])),
        (FireCaveBottom, location("Fire Cave Bottom", vec![], vec![
            path_free(RossosOreMine),
            path_free(FireCaveTop),
        ])),
        (BoulderingLedgeLeft, location("Bouldering Guy Left Ledge", vec![], vec![
            path_free(FireCaveMiddle),
            path(BoulderingLedgeRight, |p| p.can_merge()),
            path_free(BoulderingLedgeBottom),
        ])),
        (BoulderingLedgeBottom, location("Bouldering Guy Bottom Ledge", vec![], vec![
            path_free(FireCaveMiddle),
        ])),
        (BoulderingLedgeRight, location("Bouldering Guy Right Ledge", vec![
            check(LocationInfo::new(regions::hyrule::death::east::SUBREGION, "Bouldering Guy"), |p| p.has_premium_milk()),
        ], vec![
            path_free(BoulderingLedgeBottom),
            path(BoulderingLedgeLeft, |p| p.can_merge()),
            path(RossosOreMine, inaccessible),
        ])),
        (RossosOreMine, location("Rosso's Ore Mine", vec![], vec![
            path_free(FireCaveBottom),
            path(RossosOreMineLorule, |p| p.can_merge()),
        ])),
        (FloatingIslandHyrule, location("Hyrule Floating Island", vec![
            check_free(LocationInfo::new(regions::hyrule::death::far_island::SUBREGION, "Floating Island")),
        ], vec![
            path(FloatingIslandLorule, |p| p.can_merge()),
        ])),


        // --- Lorule --- //


        (LoruleCastleField, location("Lorule Castle Field", vec![
            check_free(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Rupee Rush (Lorule)")),
            check_free(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Octoball Derby")),
            check_quest_free("Access Hilda Barrier", AccessHildaBarrier),
        ], vec![
            path(GreatRupeeFairyCave, |p| p.has_bomb_flower()),
            path_free(LoruleBlacksmith),
            path_free(BootsDungeon),
            path_free(VacantHouseBottom),
            path(VacantHouseTop, |p| p.has_bombs()),
            path_free(ThiefGirlCave),
            path(SwampCave, |p| p.has_bomb_flower()),
            path(BigBombCave, |p| p.has_bomb_flower()),
            path(SwampPalaceOutside, |p| p.has_hookshot()), // cannot consider flippers as water may be drained
            path_free(ThievesHideoutB1),
            path(LoruleCastle1F, |p| p.has_all_sages()),
            path(StylishWomanHouse, |p| p.can_merge()),
            path_free(BigBombFlowerShop),
            path(BigBombFlowerField, |p| p.has_bomb_flower()),
        ])),
        (BigBombFlowerShop, location("Big Bomb Flower Shop", vec![], vec![
            path_free(LoruleCastleField),
            path_free(BigBombFlowerField),
        ])),
        (BigBombFlowerField, location("Big Bomb Flower Field", vec![
            check_quest_free("Obtain Big Bomb Flower", BigBombFlower),
        ], vec![
            path_free(BigBombFlowerShop),
            path(LoruleCastleField, |p| p.has_bomb_flower()),
        ])),
        (LoruleGraveyard, location("Lorule Graveyard", vec![
            check_free(LocationInfo::new(regions::lorule::graveyard::field::SUBREGION, "Peninsula Chest")),
        ], vec![
            path_free(LoruleSanctuaryCaveLower),
            path(LoruleSanctuary, |p| p.has_titans_mitt()),
            path(DarkRuins, inaccessible),
            path(GraveyardLedge, |p| p.can_merge()),
        ])),
        (LoruleSanctuary, location("Lorule Sanctuary", vec![
            check(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Entrance Chest"), |p| p.has_lamp()),
            check(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Lower Chest"), |p| p.has_lamp()),
            check(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Upper Chest"), |p| p.has_lamp()),
            check(LocationInfo::new(regions::dungeons::graveyard::main::SUBREGION, "[LS] Ledge"), |p| p.has_lamp() && p.can_merge()),
        ], vec![
            path_free(LoruleGraveyard),
            path(LoruleSanctuaryCaveUpper, |p| p.has_lamp() && p.can_attack() && p.has_lorule_sanctuary_key()),
        ])),
        (LoruleSanctuaryCaveLower, location("Lorule Sanctuary Cave Lower", vec![], vec![
            path(SanctuaryChurch, |p| p.can_merge()),
            path_free(LoruleGraveyard),
        ])),
        (LoruleSanctuaryCaveUpper, location("Lorule Sanctuary Cave Upper", vec![
            check(LocationInfo::new(regions::lorule::graveyard::cave::SUBREGION, "Philosopher's Cave Big Chest"), |p| p.can_merge()),
        ], vec![
            path_free(LoruleSanctuary),
            path_free(LoruleSanctuaryCaveLower),
        ])),
        (GreatRupeeFairyCave, location("Great Rupee Fairy Cave", vec![
            check_free(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy")),
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (LoruleBlacksmith, location("Lorule Blacksmith", vec![
            check(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Blacksmith (Lorule)"), |p| p.has_master_ore(4)), // TODO Hyrule Blacksmith access
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (BootsDungeon, location("Boots Treasure Dungeon", vec![
            check(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Boots Treasure Dungeon"), |p| p.has_boots()),
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (VacantHouseBottom, location("Vacant House (Bottom)", vec![], vec![
            path_free(LoruleCastleField),
        ])),
        (VacantHouseTop, location("Vacant House (Top)", vec![
            check_free(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Vacant House")),
        ], vec![
            path(LoruleCastleField, |p| p.has_bombs()),
        ])),
        (ThiefGirlCave, location("Thief Girl Cave", vec![
            check(LocationInfo::new(regions::lorule::field::thief_girl::SUBREGION, "Thief Girl Cave"), |p| p.has_sage_osfala()),
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (SwampCave, location("Swamp Cave", vec![
            check_free(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Left)")),
            check_free(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Middle)")),
            check_free(LocationInfo::new(regions::lorule::field::swamp::SUBREGION, "Swamp Cave (Right)")),
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (BigBombCave, location("Haunted Grove Big Bomb Cave", vec![
            check_free(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Big Bomb Cave")),
        ], vec![
            path_free(LoruleCastleField),
        ])),
        (HauntedGroveLedge, location("Haunted Grove Upper Ledge", vec![
            check(LocationInfo::new(regions::lorule::field::ledge::SUBREGION, "Hookshot Ledge"), |p| p.has_hookshot()),
        ], vec![
            path_free(LoruleCastleField),
            path(HyruleField, |p| p.can_merge()),
        ])),

        // Desert / Misery Mire

        (Desert, location("Desert", vec![], vec![
            path(MiseryMire, |p| p.can_merge()),
            path(MiseryMireLedge, |p| p.can_merge() && p.has_bombs() && p.has_sand_rod()),
            path(MiseryMireBridge, |p| p.can_merge() && p.has_sand_rod()),
            path(DesertSouthWestLedge, inaccessible),
            path(DesertPalaceWeatherVane, inaccessible),
        ])),
        (DesertSouthWestLedge, location("Desert South West Ledge", vec![], vec![
            path_free(Desert),
            path(MiseryMireBridge, |p| p.can_merge()),
            path(DesertPalaceWeatherVane, |p| p.has_sand_rod()),
        ])),
        (DesertPalaceWeatherVane, location("Desert Palace Weather Vane", vec![], vec![
            path_free(Desert),
            path(DesertPalaceFoyer, |p| p.has_sand_rod()),
        ])),
        (MiseryMire, location("Misery Mire", vec![], vec![
            path_free(SandRodDungeon),
            path(Desert, |p| p.can_merge()),
            path(MiseryMireOoB, inaccessible),
        ])),
        (MiseryMireBridge, location("Misery Mire Bridge", vec![], vec![
            path_free(MiseryMire),
            path(Desert, |p| p.can_merge()),
            path(DesertSouthWestLedge, |p| p.can_merge()),
            path(MiseryMireOoB, inaccessible),
        ])),
        (MiseryMireOoB, location("Misery Mire Out of Bounds", vec![], vec![
            path(MiseryMire, inaccessible),
            path(MiseryMireBridge, inaccessible),
            path(DesertZaganagaLedge, inaccessible),
            path(ZaganagasArena, inaccessible),
            path(MiseryMireRewardBasket, inaccessible),
        ])),
        (SandRodDungeon, location("Misery Mire Treasure Dungeon", vec![
            check(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Sand Rod Treasure Dungeon"), |p| p.has_sand_rod() && p.has_tornado_rod()),
        ], vec![
            path_free(MiseryMire),
        ])),
        (MiseryMireLedge, location("Misery Mire Ledge", vec![
            check_free(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Misery Mire Ledge")),
        ], vec![
            path_free(MiseryMire),
        ])),

        // Lake Lolia Area

        (LoruleLakeEast, location("Lake Lolia East", vec![], vec![
            path(HyruleField, |p| p.can_merge()),
            path(LoruleLakeWest, |p| p.has_flippers()),
            path(TurtleRockIsland, |p| p.has_flippers() && p.has_ice_rod() && p.can_merge()),
            path(DarkRuins, inaccessible),
        ])),
        (LoruleLakeWest, location("Lake Lolia West", vec![
            check_free(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lake NW Chest")),
        ], vec![
            path(HyruleField, |p| p.can_merge()),
            path_free(LakesideItemShopLorule),
            path(LoruleLakeEast, |p| p.has_flippers()),
            path(TurtleRockIsland, |p| p.has_flippers() && p.has_ice_rod() && p.can_merge()),
        ])),
        (LakesideItemShopLorule, location("Lorule Lakeside Item Shop", vec![], vec![
            path_free(LoruleLakeWest),
        ])),
        (TurtleRockIsland, location("Turtle Rock Island", vec![], vec![
            path_free(TurtleRockFoyer),
            path(LoruleLakeWest, |p| p.has_flippers()),
            path(LoruleLakeEast, |p| p.has_flippers()),
        ])),


        // Dark Ruins Area

        (DarkRuins, location("Dark Ruins", vec![
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Ruins Lakeview Chest")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Chest")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Ledge")),
        ], vec![
            path(HyruleField, |p| p.can_merge()),
            path(KusDomain, |p| p.can_merge()),
            path_free(DarkRuinsShallowWater),
            path(DarkPalaceFoyer, |p| p.has_bombs()),
            path(LoruleLakeWest, inaccessible),
            path(TurtleRockIsland, inaccessible),
            path(LoruleLakeEast, inaccessible),
        ])),
        (DarkRuinsShallowWater, location("Dark Ruins Shallow Water", vec![], vec![
            // todo figure out waterfall portal
            path(HinoxCaveShallowWater, |p| p.can_merge()),
            path(DarkRuins, |p| p.has_flippers()),
        ])),
        (KusDomain, location("Ku's Domain", vec![], vec![
            path(ZoraDomainArea, |p| p.can_merge()),
            path(HinoxCaveShallowWater, |p| p.has_flippers()),
            path(DarkRuins, |p| p.can_merge()),
        ])),
        (HinoxCaveShallowWater, location("Hinox Cave Shallow Water", vec![], vec![
            path_free(HinoxCave),
            path(DarkRuinsShallowWater, |p| p.can_merge()),
        ])),
        (HinoxCave, location("Hinox Cave", vec![
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (1)")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (2)")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (3)")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (4)")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (5)")),
            check_free(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (6)")),
        ], vec![
            path_free(HinoxCaveShallowWater),
        ])),


        // Skull Woods Area

        (SkullWoodsOverworld, location("Skull Woods (Overworld)", vec![
            check(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Canyon House"), |p| p.can_merge()),
            check_free(LocationInfo::new(regions::lorule::skull::woods::SUBREGION, "Cucco Shack")),
        ], vec![
            path(RossoHouse, |p| p.can_merge()),
            path_free(MysteriousManCave),
            path(HyruleField, |p| p.can_merge()),
            path_free(SkullWoodsFoyer),
        ])),
        (MysteriousManCave, location("Gold Bee Cave", vec![
            check_quest("Mysterious Man", GoldBee, |p| p.has_bottle())
        ], vec![
            path_free(SkullWoodsOverworld),
        ])),

        // Lorule Death Mountain

        (LoruleDeathWest, location("Lorule Death Mountain West", vec![
            check(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Ice Gimos (West)"), |p| p.can_attack()),
            check(LocationInfo::new(regions::lorule::death::west::SUBREGION, "Ledge (West)"), |p| p.can_merge()),
            check(LocationInfo::new(regions::lorule::death::tower::SUBREGION, "Treacherous Tower (Intermediate)"), |p| p.can_attack() && (p.has_bombs() || p.has_hammer() || p.has_tornado_rod())),
        ], vec![
            path(DeathMountainBase, |p| p.can_merge()),
            path(RossosOreMineLorule, inaccessible),
        ])),
        (RossosOreMineLorule, location("Rosso's Ore Mine Lorule", vec![], vec![
            path(RossosOreMine, |p| p.can_merge()),
            path(LoruleDeathWest, |p| p.has_hookshot()),
            path_free(IceCaveEast),
        ])),
        (IceCaveEast, location("Ice Cave East", vec![], vec![
            path_free(RossosOreMineLorule),
            path(IceCaveCenter, |p| p.can_merge()),
        ])),
        (IceCaveCenter, location("Ice Cave Center", vec![], vec![
            path(IceCaveEast, |p| p.can_merge()),
            path(IceCaveSouth, |p| p.can_merge()),
            path(IceCaveWest, |p| p.has_tornado_rod()),
            path_free(LoruleDeathEastTop),
        ])),
        (IceCaveSouth, location("Ice Cave South", vec![], vec![
            path_free(LoruleDeathEastLedgeLower),
            path(IceCaveCenter, |p| p.can_merge()),
        ])),
        (IceCaveWest, location("Ice Cave West", vec![], vec![
            path_free(IceCaveCenter),
            path(IceCaveNorthWest, |p| p.has_tornado_rod()),
            path(IceCaveSouthWest, |p| p.has_tornado_rod()),
        ])),
        (IceCaveNorthWest, location("Ice Cave North West", vec![], vec![
            path_free(FloatingIslandLorule),
            path(IceCaveWest, |p| p.has_tornado_rod()),
        ])),
        (FloatingIslandLorule, location("Floating Island Lorule", vec![], vec![
            path_free(IceCaveNorthWest),
            path(FloatingIslandHyrule, |p| p.can_merge()),
        ])),
        (IceCaveSouthWest, location("Ice Cave South West", vec![], vec![
            path_free(IceCaveWest),
            path_free(LoruleDeathEastLedgeUpper),
        ])),
        (LoruleDeathEastLedgeUpper, location("Lorule Death Mountain East Upper Ledge", vec![
            check(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Ledge (East)"), |p| p.can_merge()),
        ], vec![
            path_free(IceCaveWest),
            path_free(LoruleDeathEastLedgeLower),
        ])),
        (LoruleDeathEastLedgeLower, location("Lorule Death Mountain East Lower Ledge", vec![], vec![
            path_free(IceCaveSouth),
        ])),
        (LoruleDeathEastTop, location("Lorule Death Mountain East Top", vec![
            check(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Behind Ice Gimos"), |p| p.has_fire_rod()),
        ], vec![
            path_free(IceCaveCenter),
            path(IceRuinsFoyer, |p| p.has_fire_rod()),
        ])),


        // --- Hyrule Dungeons --- //

        // Eastern Palace
        (EasternPalaceFoyer, location("Eastern Palace", vec![
            check(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Outside (East)"), |p| p.can_merge()),
        ], vec![
            path_free(EasternRuinsUpper),
            path(EasternPalace1F, |p| p.can_hit_far_switch() || p.can_merge()),
        ])),
        (EasternPalace1F, location("Eastern Palace 1F", vec![
            check(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Near Entrance"), |p| p.can_hit_far_switch()),
            check(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Defeat Popos"), |p| p.can_attack()),
            check(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Hidden Door"), |p| p.can_attack()),
            check(LocationInfo::new(regions::dungeons::eastern::floor1::SUBREGION, "[EP] (1F) Switch Puzzle"), |p| p.can_hit_far_switch()),
        ], vec![
            path(EasternPalaceFoyer, |p| p.can_hit_switch() || p.can_merge()),
            path(EasternPalaceMiniboss, |p| p.has_eastern_keys(1)),
        ])),
        (EasternPalaceMiniboss, location("Eastern Palace Miniboss", vec![], vec![
            path(EasternPalace1F, |p| p.can_attack()),
            path(EasternPalace2F, |p| p.can_attack()),
        ])),
        (EasternPalace2F, location("Eastern Palace 2F", vec![
            check(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Defeat Popos"), |p| p.can_attack()),
            check_free(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Ball Room")),
            check(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Switch Room"), |p| p.can_hit_far_switch()),
            check(LocationInfo::new(regions::dungeons::eastern::floor2::SUBREGION, "[EP] (2F) Big Chest"), |p| p.has_eastern_keys(2)),
        ], vec![
            path_free(EasternPalaceMiniboss),
            path(EasternPalaceBoss, |p| p.has_eastern_keys(2) && p.can_attack() && p.has_eastern_big_key() && p.can_hit_far_switch()),
        ])),
        (EasternPalaceBoss, location("Eastern Palace 3F", vec![], vec![
            path(EasternPalacePostYuga, |p| p.can_defeat_yuga()),
        ])),
        (EasternPalacePostYuga, location("Eastern Palace Post Yuga", vec![
            check_free(LocationInfo::new(regions::dungeons::eastern::boss::SUBREGION, "[EP] (3F) After Cutscene")),
            check_free(LocationInfo::new(regions::dungeons::eastern::boss::SUBREGION, "[EP] Yuga")),
            check_quest_free("Pendant of Courage", PendantOfCourage),
        ], vec![
            path_free(EasternPalace2F),
            path(EasternPalaceEscape, |p| p.can_merge()),
        ])),
        (EasternPalaceEscape, location("Eastern Palace Escape", vec![
            check_free(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "[EP] (3F) Outside (North)")),
            check_free(LocationInfo::new(regions::dungeons::eastern::post_boss::SUBREGION, "[EP] (1F) Outside (West)")),
        ], vec![
            // do not include path back to 3F
            path_free(EasternPalace1F),
        ])),


        // House of Gales
        (HouseOfGalesFoyer, location("House of Gales Entrance", vec![], vec![
            path_free(HouseOfGalesIsland),
            path(HouseOfGalesEast1F, |p| p.has_tornado_rod()),
        ])),
        (HouseOfGalesEast1F, location("House of Gales East1F", vec![
            check(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Torches"), |p| p.has_fire_source()),
            check(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Switch Room"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::house::floor1::SUBREGION, "[HoG] (1F) Fire Bubbles"), |p| p.can_merge() && p.can_attack()),
        ], vec![
            path_free(HouseOfGalesFoyer),
            path(HouseOfGalesWest1F, |p| p.has_gales_keys(1)),
        ])),
        (HouseOfGalesWest1F, location("House of Gales West 1F", vec![
            check(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "[HoG] (1F) Blue Bari Room"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::house::floor1west::SUBREGION, "[HoG] (1F) Blue Bari Room (Bottom Left)"), |p| p.can_merge() && p.can_hit_switch()),
        ], vec![
            path_free(HouseOfGalesEast1F),
            path(HouseOfGales2F, |p| p.can_hit_hog_1f_switch()), // oddly specific switch hitting requirements
        ])),
        (HouseOfGales2F, location("House of Gales 2F", vec![
            check_free(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "[HoG] (2F) Narrow Ledge")),
            check_free(LocationInfo::new(regions::dungeons::house::floor2::SUBREGION, "[HoG] (2F) Big Chest")),
            check(LocationInfo::new(regions::dungeons::house::floor2outer::SUBREGION, "[HoG] (2F) Fire Ring"), |p| p.has_gales_keys(2) && p.can_merge()),
        ], vec![
            path_free(HouseOfGalesWest1F),
            path(HouseOfGales3F, |p| p.has_gales_keys(3) && p.can_attack() && p.can_hit_switch()),
        ])),
        (HouseOfGales3F, location("House of Gales 3F", vec![
            check(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "[HoG] (3F) Fire Bubbles"), |p| p.has_fire_source()),
            check(LocationInfo::new(regions::dungeons::house::floor3::SUBREGION, "[HoG] (3F) Rat Room"), |p| p.has_fire_source() || p.has_gales_keys(4)),
        ], vec![
            path_free(HouseOfGales2F),
            path(HouseOfGalesBoss, |p| p.has_gales_keys(4) && p.has_gales_big_key()),
        ])),
        (HouseOfGalesBoss, location("House of Gales Boss", vec![
            check(LocationInfo::new(regions::dungeons::house::boss::SUBREGION, "[HoG] Margomill"), |p| p.can_defeat_margomill()),
            check_quest("Pendant of Wisdom", PendantOfWisdom, |p| p.can_defeat_margomill()),
        ], vec![])),


        // Tower of Hera
        (TowerOfHeraFoyer, location("Tower of Hera Entrance", vec![], vec![
            path_free(DeathMountainWestTop),
            path(TowerOfHeraBottom, |p| p.has_hammer()),
        ])),
        (TowerOfHeraBottom, location("Tower of Hera Bottom", vec![
            check(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[ToH] (1F) Outside"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "[ToH] (1F) Center"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::tower::floor2::SUBREGION, "[ToH] (3F) Platform"), |p| p.can_merge()),
        ], vec![
            path(TowerOfHeraFoyer, |p| p.has_hammer()),
            path(TowerOfHeraMiddle, |p| p.has_hera_keys(1) && p.can_merge()),
        ])),
        (TowerOfHeraMiddle, location("Tower of Hera Middle", vec![
            check_free(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (5F) Red/Blue Switches")),
            check_free(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (6F) Right Mole")),
            check_free(LocationInfo::new(regions::dungeons::tower::floor4::SUBREGION, "[ToH] (6F) Left Mole")),
        ], vec![
            path_free(TowerOfHeraBottom),
            path(TowerOfHeraTop, |p| p.has_hera_keys(2)),
        ])),
        (TowerOfHeraTop, location("Tower of Hera Top", vec![
            check_free(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (7F) Outside (Ledge)")),
            check_free(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (8F) Fairy Room")),
            check_free(LocationInfo::new(regions::dungeons::tower::floor7::SUBREGION, "[ToH] (11F) Big Chest")),
        ], vec![
            path_free(TowerOfHeraMiddle),
            path(TowerOfHeraBoss, |p| p.has_hera_big_key()),
        ])),
        (TowerOfHeraBoss, location("Tower of Hera Boss", vec![
            check(LocationInfo::new(regions::dungeons::tower::boss::SUBREGION, "[ToH] Moldorm"), |p| p.can_defeat_moldorm()),
            check_quest("Pendant of Power", PendantOfPower, |p| p.can_defeat_moldorm()),
        ], vec![])),


        // Inside Hyrule Castle
        (HyruleCastleDungeon, location("Inside Hyrule Castle", vec![], vec![
            path_free(HyruleCastleRoof),
            path(LoruleBlacksmith, |p| p.has_bow() || p.has_ice_rod()),
            // TODO add game mode check for Portalsanity
        ])),

        // --- Lorule Dungeons --- //

        // Dark Palace
        (DarkPalaceFoyer, location("Dark Palace", vec![], vec![
            path_free(DarkRuins),
            path(DarkPalaceSecondRoom, |p| p.has_bombs() && p.has_lamp()),
        ])),
        (DarkPalaceSecondRoom, location("Dark Palace Second Room", vec![
            check_free(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PoD] (1F) Near Entrance")),
            check(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PoD] (1F) Narrow Ledge"), |p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
        ], vec![
            path_free(DarkPalaceFoyer),
            path(DarkPalaceMain, |p| p.has_dark_keys(1)),
        ])),
        (DarkPalaceMain, location("Dark Palace", vec![
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Switch Puzzle")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Hidden Room (Upper)")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (1F) Hidden Room (Lower)")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Fall From 1F")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Helmasaur Room")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Helmasaur Room (Fall)")),
            check(LocationInfo::new(regions::dungeons::dark::floor1::SUBREGION, "[PoD] (B1) Maze"), |p| p.can_merge()),
        ], vec![
            path_free(DarkPalaceSecondRoom),
            path(DarkPalaceLockedDoors, |p| p.has_dark_keys(4)),
        ])),
        (DarkPalaceLockedDoors, location("Dark Palace Locked Doors", vec![
            check_free(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (1F) Fall From 2F")),
            check_free(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (2F) Big Chest (Hidden)")),
            check(LocationInfo::new(regions::dungeons::dark::floor2::SUBREGION, "[PoD] (2F) Alcove"), |p| p.can_merge()),
            check_free(LocationInfo::new(regions::dungeons::dark::boss_key::SUBREGION, "[PoD] (B1) Big Chest (Switches)")),
        ], vec![
            path_free(DarkPalaceMain),
            path(DarkPalaceBoss, |p| p.has_dark_big_key()),
        ])),
        (DarkPalaceBoss, location("Dark Palace Boss", vec![
            check(LocationInfo::new(regions::dungeons::dark::boss::SUBREGION, "[PoD] Gemesaur King"), |p| p.can_defeat_gemasaur()),
            check_quest("Sage Gulley", SageGulley, |p| p.can_defeat_gemasaur()),
        ], vec![])),

        // Swamp Palace
        (SwampPalaceOutside, location("Swamp Palace Outside", vec![], vec![
            path(LoruleCastleField, |p| p.has_hookshot()),
            path_free(SwampPalaceAntechamber),
        ])),
        (SwampPalaceAntechamber, location("Swamp Palace Antechamber", vec![], vec![
            path_free(SwampPalaceOutside),
            path(SwampPalaceFoyer, |p| p.has_bomb_flower()),
        ])),
        (SwampPalaceFoyer, location("Swamp Palace", vec![], vec![
            path_free(SwampPalaceAntechamber),
            path(SwampPalaceMain, |p| p.has_flippers() && p.has_hookshot()),
        ])),
        (SwampPalaceMain, location("Swamp Palace", vec![
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Center")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Waterfall Room")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Pillar)")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Right)")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Raft Room (Left)")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Gyorm")),
        ], vec![
            path_free(SwampPalaceFoyer),
            path(SwampPalacePostMiniboss, |p| p.has_swamp_keys(2) && p.can_attack() && p.can_merge()),
        ])),
        (SwampPalacePostMiniboss, location("Swamp Palace", vec![
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) West Room")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) East Room")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) Water Puzzle")),
            check_free(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (B1) Big Chest (Secret)")),
            check(LocationInfo::new(regions::dungeons::swamp::dungeon::SUBREGION, "[SP] (1F) Big Chest (Fire)"), |p| p.has_swamp_keys(4) || p.has_tornado_rod() || p.has_ice_rod()),
        ], vec![
            path_free(SwampPalaceMain),
            path(SwampPalaceBoss, |p| p.has_swamp_keys(4) && p.has_swamp_big_key()),
        ])),
        (SwampPalaceBoss, location("Swamp Palace Boss", vec![], vec![
            path(SwampPalacePostBoss, |p| p.can_defeat_arrgus()),
        ])),
        (SwampPalacePostBoss, location("Swamp Palace Post Boss", vec![
            check_free(LocationInfo::new(regions::dungeons::swamp::boss::SUBREGION, "[SP] Arrghus")),
            check_quest_free("Sage Oren", SageOren),
        ], vec![])),


        // Skull Woods
        (SkullWoodsFoyer, location("Skull Woods Foyer", vec![], vec![
            path_free(SkullWoodsOverworld),
            path(SkullWoodsMain, |p| p.has_lamp()),
        ])),
        (SkullWoodsMain, location("Skull Woods", vec![
            check_free(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "[SW] (B1) South Chest")),
            check_free(LocationInfo::new(regions::dungeons::skull::palace::SUBREGION, "[SW] (B1) Gibdo Room (Lower)")),
            check(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "[SW] (B1) Gibdo Room (Hole)"), |p| p.has_skull_keys(1)),
            check(LocationInfo::new(regions::dungeons::skull::outdoors::SUBREGION, "[SW] (B1) Grate Room"), |p| p.has_skull_keys(1)),
        ], vec![
            path_free(SkullWoodsFoyer),
            path(SkullWoodsB2, |p| p.has_skull_keys(2) && p.can_merge()),
        ])),
        (SkullWoodsB2, location("Skull Woods B2", vec![], vec![
            path(SkullWoodsMain, |p| p.can_merge() && p.can_attack()),
            path(SkullWoodsElevatorHallway, |p| p.can_merge() && p.can_attack()),
        ])),
        (SkullWoodsElevatorHallway, location("Skull Woods Elevator Hallway", vec![
            check(LocationInfo::new(regions::dungeons::skull::basement2::SUBREGION, "[SW] (B2) Moving Platform Room"), |p| p.can_attack()),
        ], vec![
            path_free(SkullWoodsB2),
            path(SkullWoodsBossHallway, |p| p.has_skull_keys(3)),
        ])),
        (SkullWoodsBossHallway, location("Skull Woods Boss Hallway", vec![], vec![
            path_free(SkullWoodsElevatorHallway),
            path(SkullWoodsEastB1NorthFoyer, |p| p.has_fire_source() && p.can_attack()),
            path(SkullWoodsBossRoom, |p| p.has_skull_big_key()),
        ])),
        (SkullWoodsBossRoom, location("Skull Woods Boss Room", vec![
            check(LocationInfo::new(regions::dungeons::skull::boss::SUBREGION, "[SW] Knucklemaster"), |p| p.can_defeat_knucklemaster()),
        ], vec![
            path(SkullWoodsBossHallway, |p| p.can_defeat_knucklemaster()),
            path(SkullWoodsSeresGrove, |p| p.can_defeat_knucklemaster()),
        ])),
        (SkullWoodsSeresGrove, location("Skull Woods Seres Grove", vec![
            check_quest_free("Sage Seres", SageSeres),
        ], vec![
            path_free(SkullWoodsBossRoom),
        ])),
        (SkullWoodsEastB1NorthFoyer, location("Skull Woods East B1 North Foyer", vec![], vec![
            path_free(SkullWoodsBossHallway),
            path(SkullWoodsEastB1North, |p| p.can_merge()),
        ])),
        (SkullWoodsEastB1North, location("Skull Woods East B1 North", vec![
            check(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "[SW] (B1) Big Chest (Eyes)"), |p| p.has_skull_eyes()),
            check_quest_free("Skull Eye Right", SkullEyeRight),
        ], vec![
            path(SkullWoodsEastB1NorthFoyer, |p| p.can_merge()),
            path(SkullWoodsEastB1South, |p| p.has_skull_eye_right()),
        ])),
        (SkullWoodsEastB1South, location("Skull Woods East B1 South", vec![], vec![
            path(SkullWoodsEastB1North, |p| p.can_merge() && p.has_skull_eye_right()),
            path(SkullWoodsEastB1SouthFoyer, |p| p.can_merge()),
        ])),
        (SkullWoodsEastB1SouthFoyer, location("Skull Woods East B1 South Foyer", vec![], vec![
            path(SkullWoodsEastB1South, |p| p.can_merge()),
            path_free(SkullWoodsOutdoor3),
        ])),
        (SkullWoodsEastB1SouthLedges, location("Skull Woods East B1 South Ledges", vec![
            check(LocationInfo::new(regions::dungeons::skull::end::SUBREGION, "[SW] (B1) Big Chest (Upper)"), |p| p.can_merge()),
            check_quest("Skull Eye Left", SkullEyeLeft, |p| p.can_merge()),
        ], vec![
            path_free(SkullWoodsEastB1South),
        ])),
        (SkullWoodsOutdoor3, location("Skull Woods Outdoor Area 3", vec![
            check_free(LocationInfo::new(regions::lorule::skull::chest::SUBREGION, "Skull Woods Outdoor Chest")), // Do not use [SW] prefix
        ], vec![
            path_free(SkullWoodsEastB1SouthFoyer),
            path_free(SkullWoodsEastB1SouthLedges),
        ])),


        // Thieves' Hideout
        (ThievesHideoutB1, location("Thieves' Hideout", vec![
            check_free(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B1) Grate Chest")),
            check(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B1) Jail Cell"), |p| p.can_merge()),
        ], vec![
            path_free(LoruleCastleField),
            path(ThievesHideoutB2, |p| p.can_merge() && p.can_hit_switch()),
        ])),
        (ThievesHideoutB2, location("Thieves' Hideout B2", vec![
            check_free(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[TH] (B2) Grate Chest (Fall)")),
            check(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Jail Cell"), |p| p.can_merge()),
            check_free(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Switch Puzzle Room")),
            check(LocationInfo::new(regions::dungeons::thieves::basement2::SUBREGION, "[TH] (B2) Eyegores"), |p| p.can_hit_shielded_switch() && p.can_attack()),
        ], vec![
            path_free(ThievesHideoutB1),
            path(ThievesHideoutEscape, |p| p.has_thieves_key() && p.can_merge() && p.has_flippers() && p.can_attack()),
        ])),
        (ThievesHideoutEscape, location("Thieves' Hideout Escape", vec![
            check(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B3) Underwater"), |p| p.can_merge()),
            check_free(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B3) Big Chest (Hidden)")),
            check_free(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B1) Behind Wall")),
            check(LocationInfo::new(regions::dungeons::thieves::escape::SUBREGION, "[TH] (B1) Big Chest (Entrance)"), |p| p.can_merge()),
        ], vec![
            path_free(ThievesHideoutB2),
            path(ThievesBoss, |p| p.has_thieves_big_key()),
        ])),
        (ThievesBoss, location("Thieves' Hideout Boss", vec![], vec![
            path(ThievesPostBoss, |p| p.can_defeat_stalblind()),
        ])),
        (ThievesPostBoss, location("Thieves' Hideout Post Boss", vec![
            check_free(LocationInfo::new(regions::dungeons::thieves::boss::SUBREGION, "Stalblind")),
            check_quest_free("Sage Osfala", SageOsfala),
        ], vec![])),

        // Turtle Rock
        (TurtleRockFoyer, location("Turtle Rock Foyer", vec![], vec![
            path_free(TurtleRockIsland),
            path(TurtleRockMain, |p| p.has_ice_rod()),
        ])),
        (TurtleRockMain, location("Turtle Rock Main", vec![
            check_free(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Center")),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Northeast Ledge"), |p| p.can_merge() || p.has_boomerang() || p.has_hookshot()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Southeast Chest"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Defeat Flamolas"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Portal Room (Northwest)"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Grate Chest"), |p| p.can_merge()),
            check_free(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Northeast Room")),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Grate Chest (Small)"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Top)"), |p| (p.has_turtle_keys(3) || (p.has_turtle_keys(2) && p.can_defeat_grinexx())) && p.can_merge() && p.can_hit_shielded_switch()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Center)"), |p| p.can_merge() && p.can_hit_shielded_switch()),
            check(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Platform"), |p| p.has_turtle_keys(2) || p.can_merge()),
        ], vec![
            path(TurtleRockFoyer, |p| p.has_ice_rod()),
            path(TurtleRockLeftBalconyPath, |p| p.can_merge()),
            path(TurtleRockRightBalconyPath, |p| p.can_merge()),
            path(TurtleRockBoss, |p| p.has_turtle_keys(3) && p.can_merge() && p.has_turtle_big_key()),
        ])),
        (TurtleRockLeftBalconyPath, location("Turtle Rock Left Balcony Path", vec![], vec![
            path(TurtleRockMain, |p| p.has_ice_rod()),
            path(TurtleRockLeftBalcony, |p| p.has_ice_rod()),
        ])),
        (TurtleRockLeftBalcony, location("Turtle Rock Left Balcony", vec![
            check_free(LocationInfo::new(regions::lorule::lake::balcony::SUBREGION, "Turtle Rock Left Balcony")), // Do not use [TR] prefix
        ], vec![
            path_free(TurtleRockLeftBalconyPath),
        ])),
        (TurtleRockRightBalconyPath, location("Turtle Rock Right Balcony Path", vec![], vec![
            path(TurtleRockMain, |p| p.has_ice_rod()),
            path(TurtleRockRightBalcony, |p| p.has_ice_rod()),
        ])),
        (TurtleRockRightBalcony, location("Turtle Rock Right Balcony", vec![], vec![
            path_free(TurtleRockRightBalconyPath),
        ])),
        (TurtleRockBoss, location("Turtle Rock Boss", vec![], vec![
            path(TurtleRockPostBoss, |p| p.can_defeat_grinexx()),
        ])),
        (TurtleRockPostBoss, location("Turtle Rock Boss", vec![
            check_free(LocationInfo::new(regions::dungeons::turtle::boss::SUBREGION, "[TR] Grinexx")),
            check_quest_free("Sage Impa", SageImpa),
        ], vec![])),

        // Desert Palace
        (DesertPalaceFoyer, location("Desert Palace Entrance", vec![
            check(LocationInfo::new(regions::dungeons::desert::floor1::SUBREGION, "[DP] (1F) Entrance"), |p| p.has_sand_rod()),
        ], vec![
            path_free(DesertPalaceWeatherVane),
            path(DesertPalace1F, |p| p.has_sand_rod() && p.can_attack()),
        ])),
        (DesertPalace1F, location("Desert Palace 1F", vec![
            check_free(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Switch Room")),
            check_free(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Room (North)")),
            check_free(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Sand Room (South)")),
            check(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Behind Rocks"), |p| p.has_titans_mitt()),
            check(LocationInfo::new(regions::dungeons::desert::post_miniboss::SUBREGION, "[DP] (1F) Big Chest (Behind Wall)"), |p| p.has_desert_keys(4)),
        ], vec![
            path(DesertPalaceFoyer, |p| p.has_sand_rod() && p.can_attack()),
            path(DesertPalaceMidwayLedge, |p| p.has_titans_mitt()),
        ])),
        (DesertPalaceMidwayLedge, location("Desert Palace Midway Ledge", vec![], vec![
            path_free(DesertPalaceWeatherVane),
            path_free(DesertPalace1F),
            path_free(DesertPalace2F),
        ])),
        (DesertPalace2F, location("Desert Palace 2F", vec![
            check(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Left)"), |p| p.has_titans_mitt()),
            check(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Right)"), |p| p.has_titans_mitt()),
            check(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Under Rock (Ball Room)"), |p| p.has_titans_mitt()),
            check_free(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Beamos Room")),
            check_free(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Red/Blue Switches")),
            check_free(LocationInfo::new(regions::dungeons::desert::floor2::SUBREGION, "[DP] (2F) Big Chest (Puzzle)")),
            check(LocationInfo::new(regions::dungeons::desert::floor2west::SUBREGION, "[DP] (2F) Leever Room"), |p| p.has_desert_keys(3)),
        ], vec![
            path_free(DesertPalaceMidwayLedge),
            path(DesertPalace1F, |p| p.can_attack()), // midway
            path(DesertPalace3F, |p| p.has_desert_keys(4) && p.has_sand_rod()),
        ])),
        (DesertPalace3F, location("Desert Palace 3F", vec![
            check(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "[DP] (3F) Behind Falling Sand"), |p| p.has_sand_rod()),
            check(LocationInfo::new(regions::dungeons::desert::floor3::SUBREGION, "[DP] (3F) Armos Room"), |p| p.has_sand_rod()),
        ], vec![
            path_free(DesertPalace2F),
            path(DesertZaganagaLedge, |p| p.has_desert_keys(5) && p.has_desert_big_key()),
        ])),
        (DesertZaganagaLedge, location("Desert Zaganaga Ledge", vec![], vec![
            path_free(DesertPalace3F),
            path(ZaganagasArena, |p| p.can_merge()),
        ])),
        (ZaganagasArena, location("Zaganaga's Arena", vec![], vec![
            path(DesertZaganagaLedge, |p| p.can_merge()),
            path(MiseryMireRewardBasket, |p| p.can_defeat_zaganaga()),
        ])),
        (MiseryMireRewardBasket, location("Misery Mire Reward Basket", vec![
            check_free(LocationInfo::new(regions::dungeons::desert::boss::SUBREGION, "Zaganaga")), // Do not use [DP] prefix
            check_quest_free("Sage Irene", SageIrene),
        ], vec![])),

        // Ice Ruins
        (IceRuinsFoyer, location("Ice Ruins Entrance", vec![], vec![
            path_free(LoruleDeathEastTop),
            path(IceRuins, |p| p.has_fire_rod()),
        ])),

        // Require Fire Rod
        (IceRuins, location("Ice Ruins", vec![
            check_free(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (1F) Hidden Chest")),
            check_free(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Ice Pillar")),
            check_free(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Left)")),
            check_free(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Right)")),
            check_free(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B5) Big Chest")),
            check(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "[IR] (B1) Narrow Ledge"), |p| p.has_ice_keys(1)),
            check(LocationInfo::new(regions::dungeons::ice::basement1::SUBREGION, "[IR] (B1) East Chest"), |p| p.has_ice_keys(1)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B1) Upper Chest"), |p| p.has_ice_keys(2)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B2) Far North"), |p| p.has_ice_keys(2) && p.has_stamina_scroll()),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B3) Big Chest (Puzzle)"), |p| p.has_ice_keys(2)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Switches"), |p| p.has_ice_keys(2)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Southwest Chest (Fall)"), |p| p.has_ice_keys(2)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Narrow Platform"), |p| p.has_ice_keys(2)),
            check(LocationInfo::new(regions::dungeons::ice::basement2::SUBREGION, "[IR] (B4) Southeast Chest (Fall)"), |p| p.has_ice_keys(2)),
        ], vec![
            path(IceRuinsFoyer, |p| p.has_fire_rod()),
            path(IceRuinsBoss, |p| p.has_ice_keys(3) && p.has_ice_big_key()),
        ])),
        (IceRuinsBoss, location("Ice Ruins Boss", vec![], vec![
            path(IceRuinsPostBoss, |p| p.can_defeat_dharkstare()),
        ])),
        (IceRuinsPostBoss, location("Ice Ruins Post Boss", vec![
            check_free(LocationInfo::new(regions::dungeons::ice::boss::SUBREGION, "[IR] Dharkstare")),
            check_quest_free("Sage Rosso", SageRosso),
        ], vec![])),


        // Lorule Castle
        (LoruleCastle1F, location("Lorule Castle 1F", vec![], vec![
            path(LoruleCastleEastLedge1F, |p| p.can_merge()),
            path(LoruleCastle2F3F, |p| p.can_attack()),
            path(LoruleCastleCenter1F, inaccessible),
        ])),
        (LoruleCastleEastLedge1F, location("Lorule Castle East Ledge 1F", vec![
            check(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (1F) Ledge"), |p| p.can_merge()),
        ], vec![
            path(LoruleCastle1F, |p| p.can_merge()),
        ])),
        (LoruleCastleCenter1F, location("Lorule Castle 1F Center", vec![
            check_free(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (1F) Center")),
        ], vec![
            path_free(LoruleCastle1F),
            path(LoruleCastleEastLedge1F, inaccessible),
        ])),
        (LoruleCastle2F3F, location("Lorule Castle 2F 3F", vec![
            check_free(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Near Torches")),
            check(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Hidden Path"), |p| p.can_extinguish_torches()),
            check(LocationInfo::new(regions::dungeons::castle::lorule::SUBREGION, "[LC] (2F) Ledge"), |p| p.can_merge()),
            check(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "[LC] (3F) Bomb Trial (Chest)"), |p| p.has_bombs()),
            check(LocationInfo::new(regions::dungeons::castle::bomb_trial::SUBREGION, "[LC] (3F) Bomb Trial (Behind Rock)"), |p| p.has_bombs() && p.can_merge()),
            check_free(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "[LC] (3F) Ball Trial (Chest)")),
            check(LocationInfo::new(regions::dungeons::castle::ball_trial::SUBREGION, "[LC] (3F) Ball Trial (Puzzle)"), |p| p.can_merge()),
        ], vec![
            path_free(LoruleCastle1F),
            path_free(LoruleCastleCenter1F),
            path(LoruleCastle4F5F, |p| p.has_lorule_keys(3)),
            path(HildasStudy, |p| p.has_lorule_keys(5) && p.has_bombs() && p.has_hookshot() && p.has_lamp()),
        ])),

        // require 3 small keys
        (LoruleCastle4F5F, location("Lorule Castle 4F 5F", vec![
            check(LocationInfo::new(regions::dungeons::castle::lamp_trial::SUBREGION, "[LC] (4F) Lamp Trial"), |p| p.has_fire_source()),
            check(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "[LC] (4F) Hookshot Trial (Eyes)"), |p| p.has_hookshot()),
            check(LocationInfo::new(regions::dungeons::castle::hookshot_trial::SUBREGION, "[LC] (4F) Hookshot Trial (Chest)"), |p| p.has_hookshot()),
            check_free(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "[LC] (4F) Center")),
            check(LocationInfo::new(regions::dungeons::castle::floor4::SUBREGION, "[LC] (4F) Hidden Path"), |p| p.can_extinguish_torches()),
        ], vec![
            path_free(LoruleCastle2F3F),
        ])),
        (ZeldasStudy, location("Zelda's Study", vec![], vec![
            path(HildasStudy, |p| p.can_merge()),
        ])),
        (HildasStudy, location("Hilda's Study", vec![], vec![
            path_free(LoruleCastle2F3F),
            path(ZeldasStudy, |p| p.can_merge()),
            path(ThroneRoom, |p| p.has_all_sages() && p.can_reach_hilda_barrier()),
        ])),
        (ThroneRoom, location("Throne Room", vec![
            check(LocationInfo::new(regions::dungeons::castle::boss::SUBREGION, "[LC] Zelda"), |p| p.can_attack() && p.has_sword()),
        ], vec![
            path(SacredRealm, |p| p.can_defeat_yuganon()),
        ])),
        (SacredRealm, location("Sacred Realm", vec![], vec![])),
    ])
}

fn location(name: &'static str, checks: Vec<Check>, paths: Vec<Path>) -> LocationNode {
    LocationNode::new(name, checks, paths)
}

fn check(location_info: LocationInfo, logic: fn(&Progress) -> bool) -> Check {
    Check::new(location_info.name, Some(logic), None, Some(location_info))
}

fn check_free(location_info: LocationInfo) -> Check {
    Check::new(location_info.name, None, None, Some(location_info))
}

fn check_quest(name: &'static str, quest: FillerItem, logic: fn(&Progress) -> bool) -> Check {
    Check::new(name, Some(logic), Some(quest), None)
}

fn check_quest_free(name: &'static str, quest: FillerItem) -> Check {
    Check::new(name, None, Some(quest), None)
}

fn path_free(default: Location) -> Path {
    Path::new(default, None)
}

// add logic to choose random entrances here
fn path(default: Location, logic: fn(&Progress) -> bool) -> Path {
    Path::new(default, Some(logic))
}

fn inaccessible(_: &Progress) -> bool {
    false
}