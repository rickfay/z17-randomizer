use log::{error, info, LevelFilter};
use macros::fail;
use modinfo::settings::keysy::Keysy;
use modinfo::settings::portals::Portals;
use modinfo::settings::ravios_shop::RaviosShop;
use modinfo::settings::trials_door::TrialsDoor;
use modinfo::settings::weather_vanes::WeatherVanes;
use modinfo::settings::{logic::LogicMode, pedestal::PedestalSetting, portal_shuffle::PortalShuffle, Settings};
use randomizer::filler::filler_item::Item::*;
use randomizer::filler::filler_item::Randomizable;
use randomizer::filler::filler_item::Vane::*;
use randomizer::filler::portals::Portal;
use randomizer::filler::{filler_item, item_pools};
use randomizer::{
    constants::VERSION,
    regions,
    system::{System, UserConfig},
    Layout, PortalMap, SeedHash, SeedInfo, VaneMap,
};
use simplelog::SimpleLogger;
use std::collections::BTreeSet;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(long)]
    no_patch: bool,

    #[structopt(long)]
    no_spoiler: bool,
}

/**
 * PLANDOMIZER
 *
 * Special version of the randomizer where all items are placed by hand instead of relying on the
 * filler algorithm. TODO: Rework this to build Plandos from input JSON instead of hardcoding them.
 */
fn main() {
    let args = Opt::from_args();
    SimpleLogger::init(LevelFilter::Info, Default::default()).expect("Failed to init logger.");

    info!("Initializing ALBW Plandomizer...\n");

    // Load User Config
    let user_config: UserConfig = System::load_config().unwrap_or_else(|error| {
        fail!("Failed to parse configuration file: config.json\n\
                Commonly Fixed By: Replace any single backslash characters '\\' with a forward slash '/' or double backslash '\\\\'.\n\
                Full Error: {}\n", error);
    });

    let seed = 0;
    let settings = plando_settings();

    let seed_info = SeedInfo {
        seed,
        version: String::from(VERSION),
        hash: SeedHash::new(seed, &settings),
        settings,
        full_exclusions: Default::default(),
        treacherous_tower_floors: vec![],
        vane_map: get_plando_weather_vane_map(),
        portal_map: get_plando_portal_map(),
        layout: build_layout(),
        metrics: Default::default(),
        hints: Default::default(),
        trials_config: Default::default(),
        world_graph: Default::default(),
    };

    seed_info.settings.log_settings();

    match randomizer::patch_seed(&seed_info, &user_config, args.no_patch, args.no_spoiler) {
        Ok(_) => {
            println!();
            info!("Successfully Generated ALBW Plandomizer Seed");
        },
        Err(err) => {
            println!();
            error!("Plandomizer execution failed:\n{}", err.into_inner());
        },
    }

    cli::pause();
}

fn get_plando_weather_vane_map() -> VaneMap {
    use filler_item::Vane::*;
    VaneMap::from_iter([
        (YourHouseWV, YourHouseWV),
        (KakarikoVillageWV, KakarikoVillageWV),
        (EasternPalaceWV, EasternPalaceWV),
        (HouseOfGalesWV, HouseOfGalesWV),
        (TowerOfHeraWV, TowerOfHeraWV),
        (WitchsHouseWV, WitchsHouseWV),
        (DeathMountainHyruleWV, DeathMountainHyruleWV),
        (DesertPalaceWV, DesertPalaceWV),
        (SanctuaryWV, SanctuaryWV),
        (SkullWoodsWV, SkullWoodsWV),
        (TreacherousTowerWV, TreacherousTowerWV),
        (IceRuinsWV, IceRuinsWV),
        (LoruleCastleWV, LoruleCastleWV),
        (GraveyardWV, GraveyardWV),
        (ThievesTownWV, ThievesTownWV),
        (DarkPalaceWV, DarkPalaceWV),
        (BlacksmithWV, BlacksmithWV),
        (VacantHouseWV, VacantHouseWV),
        (MiseryMireWV, MiseryMireWV),
        (SwampPalaceWV, SwampPalaceWV),
        (TurtleRockWV, TurtleRockWV),
        (DeathMountainLoruleWV, DeathMountainLoruleWV),
    ])
}

fn get_plando_portal_map() -> PortalMap {
    use Portal::*;
    PortalMap::from_iter([
        (StylishWoman, ThievesTown),
        (YourHouse, VacantHouse),
        (ParadoxRightHyrule, ParadoxRightLorule),
        (ParadoxLeftHyrule, ParadoxLeftLorule),
        (WaterfallHyrule, WaterfallLorule),
        (EasternRuinsPillar, DarkRuinsPillar),
        (EasternRuinsSE, DarkRuinsSE),
        (LostWoodsPillar, SkullWoodsPillar),
        (SahasrahlasHouse, NShapedHouse),
        (RossosHouse, DestroyedHouse),
        (MiseryMireEntrance, MiseryMireExit),
        (DesertPillarRight, MirePillarRight),
        (DesertPillarLeft, MirePillarLeft),
        (DesertMiddle, MireMiddle),
        (DesertSW, MireSW),
        (DesertPalace, Zaganaga),
        (DesertNorth, MireNorth),
        (DeathWestHyrule, DeathWestLorule),
        (FloatingIslandHyrule, FloatingIslandLorule),
        (RiverHyrule, RiverLorule),
        (LakeHylia, LoruleLake),
        (HyruleHotfoot, LoruleHotfoot),
        (Sanctuary, Philosopher),
        (GraveyardLedgeHyrule, GraveyardLedgeLorule),
        (RossosOreMineHyrule, RossosOreMineLorule),
        (SwampPillarHyrule, SwampPillarLorule),
        (ZorasDomain, KusDomain),
        (HyruleCastle, LoruleCastle),
        (ThievesTown, StylishWoman),
        (VacantHouse, YourHouse),
        (ParadoxRightLorule, ParadoxRightHyrule),
        (ParadoxLeftLorule, ParadoxLeftHyrule),
        (WaterfallLorule, WaterfallHyrule),
        (DarkRuinsPillar, EasternRuinsPillar),
        (DarkRuinsSE, EasternRuinsSE),
        (SkullWoodsPillar, LostWoodsPillar),
        (NShapedHouse, SahasrahlasHouse),
        (DestroyedHouse, RossosHouse),
        (MiseryMireExit, MiseryMireEntrance),
        (MirePillarRight, DesertPillarRight),
        (MirePillarLeft, DesertPillarLeft),
        (MireMiddle, DesertMiddle),
        (MireSW, DesertSW),
        (Zaganaga, DesertPalace),
        (MireNorth, DesertNorth),
        (DeathWestLorule, DeathWestHyrule),
        (FloatingIslandLorule, FloatingIslandHyrule),
        (RiverLorule, RiverHyrule),
        (LoruleLake, LakeHylia),
        (LoruleHotfoot, HyruleHotfoot),
        (Philosopher, Sanctuary),
        (GraveyardLedgeLorule, GraveyardLedgeHyrule),
        (RossosOreMineLorule, RossosOreMineHyrule),
        (SwampPillarLorule, SwampPillarHyrule),
        (KusDomain, ZorasDomain),
        (LoruleCastle, HyruleCastle),
    ])
}

fn plando_settings() -> Settings {
    Settings {
        dev_mode: true,
        lc_requirement: 7,
        yuganon_requirement: 7,
        ped_requirement: PedestalSetting::Standard,
        logic_mode: LogicMode::Normal,
        dark_rooms_lampless: false,
        dungeon_prize_shuffle: true,
        maiamai_madness: false,
        nice_mode: false,
        super_mode: true,
        portals: Portals::Open,
        portal_shuffle: PortalShuffle::AnyWorldPairs,
        weather_vanes: WeatherVanes::Hyrule,
        ravios_shop: RaviosShop::Closed,
        bow_of_light_in_castle: false,
        no_progression_enemies: false,
        keysy: Keysy::Off,
        progressive_bow_of_light: false,
        swordless_mode: false,
        start_with_merge: true,
        start_with_pouch: true,
        bell_in_shop: false,
        sword_in_shop: false,
        boots_in_shop: false,
        assured_weapon: false,
        chest_size_matches_contents: true,
        minigames_excluded: false,
        skip_big_bomb_flower: true,
        trials_door: TrialsDoor::RequiredTrials(1),
        treacherous_tower_floors: 5,
        night_mode: false,
        user_exclusions: BTreeSet::default(),
    }
}

#[rustfmt::skip]
fn build_layout() -> Layout {

    info!("Building Item Layout from Plan...");
    let mut layout = Layout::default();

    let mut maiamai = item_pools::get_maiamai_pool().iter().map(|&item| Randomizable::from(item)).collect::<Vec<_>>();
    let mut heart_containers = item_pools::get_heart_containers();
    let mut heart_pieces = item_pools::get_heart_pieces();
    let mut gold_rupees = item_pools::get_gold_rupee_pool();
    let mut silver_rupees = item_pools::get_silver_rupee_pool();
    let mut purple_rupees = item_pools::get_purple_rupee_pool();

    // FIXME set weather vanes

    //////////////////////////
    // --- Ravio's Shop --- //
    //////////////////////////

    layout.set_item("Ravio's Gift", regions::hyrule::ravio::shop::SUBREGION, RaviosBracelet01);
    layout.set_item("Ravio's Shop (1)", regions::hyrule::ravio::shop::SUBREGION, IceRod01);
    layout.set_item("Ravio's Shop (2)", regions::hyrule::ravio::shop::SUBREGION, Hookshot01);
    layout.set_item("Ravio's Shop (3)", regions::hyrule::ravio::shop::SUBREGION, TornadoRod01);
    layout.set_item("Ravio's Shop (4)", regions::hyrule::ravio::shop::SUBREGION, Bombs01);
    layout.set_item("Ravio's Shop (5)", regions::hyrule::ravio::shop::SUBREGION, Bow01);
    layout.set_item("Ravio's Shop (6)", regions::hyrule::ravio::shop::SUBREGION, SandRod01);
    layout.set_item("Ravio's Shop (7)", regions::hyrule::ravio::shop::SUBREGION, Hammer01);
    layout.set_item("Ravio's Shop (8)", regions::hyrule::ravio::shop::SUBREGION, Boomerang01);
    layout.set_item("Ravio's Shop (9)", regions::hyrule::ravio::shop::SUBREGION, FireRod01);

    /////////////////////////////
    // --- Dungeons Prizes --- //
    /////////////////////////////

    layout.set_item("[EP] Prize", regions::dungeons::eastern::palace::SUBREGION, PendantOfCourage);
    layout.set_item("[HG] Prize", regions::dungeons::house::gales::SUBREGION, PendantOfWisdom);
    layout.set_item("[TH] Prize", regions::dungeons::tower::hera::SUBREGION, PendantOfPower);

    layout.set_item("[HC] Zelda", regions::dungeons::hyrule::castle::SUBREGION, Charm);

    layout.set_item("[PD] Prize", regions::dungeons::dark::palace::SUBREGION, SageGulley);
    layout.set_item("[SP] Prize", regions::dungeons::swamp::palace::SUBREGION, SageOren);
    layout.set_item("[SW] Prize", regions::dungeons::skull::woods::SUBREGION, SageSeres);
    layout.set_item("[TT] Prize", regions::dungeons::thieves::hideout::SUBREGION, SageOsfala);
    layout.set_item("[TR] Prize", regions::dungeons::turtle::rock::SUBREGION, SageImpa);
    layout.set_item("[DP] Prize", regions::dungeons::desert::palace::SUBREGION, SageIrene);
    layout.set_item("[IR] Prize", regions::dungeons::ice::ruins::SUBREGION, SageRosso);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field

    // layout.set_item("Delivery", regions::hyrule::field::main::SUBREGION, Empty);
    layout.set_item("Blacksmith Ledge", regions::hyrule::field::main::SUBREGION, HeartPiece04);
    layout.set_item("Blacksmith Cave", regions::hyrule::field::main::SUBREGION, HeartPiece14);
    layout.set_item("Blacksmith", regions::hyrule::field::main::SUBREGION, Sword03);
    layout.set_item("Blacksmith Table", regions::hyrule::field::main::SUBREGION, Empty);
    layout.set_item("Cucco Mini-Dungeon", regions::hyrule::field::main::SUBREGION, RupeeSilver01);
    layout.set_item("Hyrule Castle Rocks", regions::hyrule::field::main::SUBREGION, Empty);
    layout.set_item("Haunted Grove Stump", regions::hyrule::field::main::SUBREGION, Empty);

    layout.set_item("[Mai] Blacksmith Tree", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Behind Your House", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Blacksmith Tiles", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Outside Cucco Mini-Dungeon", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Haunted Grove Tree", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Hyrule Castle Tree", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Hyrule Castle Tiles", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Your House Tree", regions::hyrule::field::main::SUBREGION, maiamai.pop().unwrap());

    // Irene the Witch
    layout.set_item("Irene", regions::hyrule::irene::witch::SUBREGION, Bell);

    // Lost Woods
    layout.set_item("Fortune-Teller", regions::hyrule::lost::woods::SUBREGION, HintGlasses);
    layout.set_item("Hyrule Hotfoot 75s", regions::hyrule::lost::woods::SUBREGION, silver_rupees.pop().unwrap());
    layout.set_item("Hyrule Hotfoot 65s", regions::hyrule::lost::woods::SUBREGION, HeartPiece02);
    layout.set_item("Lost Woods Alcove", regions::hyrule::lost::woods::SUBREGION, HeartPiece09);
    layout.set_item("Lost Woods Chest", regions::hyrule::lost::woods::SUBREGION, RupeeRed);
    layout.set_item("Master Sword Pedestal", regions::hyrule::lost::woods::SUBREGION, Sword02);
    layout.set_item("Rosso (1)", regions::hyrule::lost::woods::SUBREGION, Glove01);
    layout.set_item("Rosso (2)", regions::hyrule::lost::woods::SUBREGION, purple_rupees.pop().unwrap());
    layout.set_item("Rosso Cave", regions::hyrule::lost::woods::SUBREGION, Empty);

    layout.set_item("[Mai] Rosso Wall", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lost Woods Path Rock", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lost Woods Bush", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lost Woods Rock", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Fortune-Teller Tent", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Moldorm Ledge", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Small Pond", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lost Woods Tree", regions::hyrule::lost::woods::SUBREGION, maiamai.pop().unwrap());

    // Death Mountain
    layout.set_item("Death Mountain Open Cave", regions::hyrule::death::mountain::SUBREGION, RupeeBlue);
    layout.set_item("Death Mountain Blocked Cave", regions::hyrule::death::mountain::SUBREGION, Empty);
    layout.set_item("Death Mountain Fairy Cave", regions::hyrule::death::mountain::SUBREGION, Empty);
    layout.set_item("Donkey Cave Ledge", regions::hyrule::death::mountain::SUBREGION, Empty);
    layout.set_item("Donkey Cave", regions::hyrule::death::mountain::SUBREGION, Empty);
    layout.set_item("Death Mountain West Highest Cave", regions::hyrule::death::mountain::SUBREGION, Empty);
    layout.set_item("Spectacle Rock", regions::hyrule::death::mountain::SUBREGION, HeartPiece08);
    layout.set_item("Fire Cave Pillar", regions::hyrule::death::mountain::SUBREGION, HeartPiece28);
    layout.set_item("Bouldering Guy", regions::hyrule::death::mountain::SUBREGION, Bottle05);
    layout.set_item("Hookshot Mini-Dungeon", regions::hyrule::death::mountain::SUBREGION, silver_rupees.pop().unwrap());
    layout.set_item("Floating Island", regions::hyrule::death::mountain::SUBREGION, HeartPiece15);

    layout.set_item("[Mai] Death Mountain Base Rock", regions::hyrule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Death Mountain West Ledge", regions::hyrule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Fire Cave Ledge", regions::hyrule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Rosso's Ore Mine", regions::hyrule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Outside Hookshot Mini-Dungeon", regions::hyrule::death::mountain::SUBREGION, maiamai.pop().unwrap());


    // Kakariko
    layout.set_item("Bee Guy (1)", regions::hyrule::kakariko::village::SUBREGION, Net01);
    layout.set_item("Bee Guy (2)", regions::hyrule::kakariko::village::SUBREGION, BeeBadge);
    layout.set_item("Dodge the Cuccos", regions::hyrule::kakariko::village::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Kakariko Item Shop (1)", regions::hyrule::kakariko::village::SUBREGION, ScootFruit01);
    layout.set_item("Kakariko Item Shop (2)", regions::hyrule::kakariko::village::SUBREGION, FoulFruit01);
    layout.set_item("Kakariko Item Shop (3)", regions::hyrule::kakariko::village::SUBREGION, Shield01);
    layout.set_item("Kakariko Jail", regions::hyrule::kakariko::village::SUBREGION, silver_rupees.pop().unwrap());
    layout.set_item("Kakariko Well (Bottom)", regions::hyrule::kakariko::village::SUBREGION, Empty);
    layout.set_item("Kakariko Well (Top)", regions::hyrule::kakariko::village::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Rupee Rush (Hyrule)", regions::hyrule::kakariko::village::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Shady Guy", regions::hyrule::kakariko::village::SUBREGION, PegasusBoots);
    layout.set_item("Street Merchant (Left)", regions::hyrule::kakariko::village::SUBREGION, Bottle01);
    layout.set_item("Street Merchant (Right)", regions::hyrule::kakariko::village::SUBREGION, SmoothGem);
    layout.set_item("Stylish Woman", regions::hyrule::kakariko::village::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Woman", regions::hyrule::kakariko::village::SUBREGION, RupeeRed);

    layout.set_item("[Mai] Cucco Ranch Tree", regions::hyrule::kakariko::village::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Hyrule Rupee Rush Wall", regions::hyrule::kakariko::village::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Kakariko Bush", regions::hyrule::kakariko::village::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Kakariko Sand", regions::hyrule::kakariko::village::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Woman's Roof", regions::hyrule::kakariko::village::SUBREGION, maiamai.pop().unwrap());

    // Zora's River
    layout.set_item("Queen Oren", regions::hyrule::zora::river::SUBREGION, Empty);
    layout.set_item("Waterfall Cave", regions::hyrule::zora::river::SUBREGION, Empty);
    layout.set_item("Zora's Domain Ledge", regions::hyrule::zora::river::SUBREGION, Empty);
    layout.set_item("River Mini-Dungeon", regions::hyrule::zora::river::SUBREGION, Empty);

    layout.set_item("[Mai] Witch's House", regions::hyrule::zora::river::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Wooden Bridge", regions::hyrule::zora::river::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Waterfall Ledge", regions::hyrule::zora::river::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] South of Zora's Domain", regions::hyrule::zora::river::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Zora's Domain", regions::hyrule::zora::river::SUBREGION, maiamai.pop().unwrap());

    // Eastern Ruins
    layout.set_item("Bird Lover", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Merge Mini-Dungeon", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Eastern Ruins Armos Chest", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Eastern Ruins Hookshot Chest", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Eastern Ruins Merge Chest", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Eastern Ruins Cave", regions::hyrule::eastern::ruins::SUBREGION, Empty);
    layout.set_item("Eastern Ruins Peg Circle", regions::hyrule::eastern::ruins::SUBREGION, Empty);

    layout.set_item("[Mai] Eastern Ruins Bonk Rocks", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Eastern Ruins Rock", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Eastern Ruins Green Tree", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Eastern Ruins Wall", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Eastern Ruins Yellow Tree", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Eastern Ruins River", regions::hyrule::eastern::ruins::SUBREGION, maiamai.pop().unwrap());

    // Desert of Mystery
    layout.set_item("[Mai] Buried in the Desert", regions::hyrule::desert::mystery::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Buried near Desert Palace", regions::hyrule::desert::mystery::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Southern Ruins Big Rock", regions::hyrule::desert::mystery::SUBREGION, maiamai.pop().unwrap());

    // Southern Ruins
    layout.set_item("Runaway Item Seller", regions::hyrule::southern::ruins::SUBREGION, Empty);
    layout.set_item("Southern Ruins Ledge", regions::hyrule::southern::ruins::SUBREGION, Empty);
    layout.set_item("Southern Ruins Pillar Cave", regions::hyrule::southern::ruins::SUBREGION, Empty);
    layout.set_item("Flippers Mini-Dungeon", regions::hyrule::southern::ruins::SUBREGION, Empty);

    layout.set_item("[Mai] Outside Flippers Mini-Dungeon", regions::hyrule::southern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Southern Ruins Bomb Cave", regions::hyrule::southern::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Southern Ruins Pillars", regions::hyrule::southern::ruins::SUBREGION, maiamai.pop().unwrap());

    // Lake Hylia
    layout.set_item("Maiamai Bow Upgrade", regions::hyrule::lake::hylia::SUBREGION, Bow02);
    layout.set_item("Maiamai Boomerang Upgrade", regions::hyrule::lake::hylia::SUBREGION, Boomerang02);
    layout.set_item("Maiamai Hookshot Upgrade", regions::hyrule::lake::hylia::SUBREGION, Hookshot02);
    layout.set_item("Maiamai Hammer Upgrade", regions::hyrule::lake::hylia::SUBREGION, Hammer02);
    layout.set_item("Maiamai Bombs Upgrade", regions::hyrule::lake::hylia::SUBREGION, Bombs02);
    layout.set_item("Maiamai Fire Rod Upgrade", regions::hyrule::lake::hylia::SUBREGION, FireRod02);
    layout.set_item("Maiamai Ice Rod Upgrade", regions::hyrule::lake::hylia::SUBREGION, IceRod02);
    layout.set_item("Maiamai Tornado Rod Upgrade", regions::hyrule::lake::hylia::SUBREGION, TornadoRod02);
    layout.set_item("Maiamai Sand Rod Upgrade", regions::hyrule::lake::hylia::SUBREGION, SandRod02);
    layout.set_item("100 Maiamai", regions::hyrule::lake::hylia::SUBREGION, GreatSpin);

    layout.set_item("Ice Rod Cave", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lake Hylia Dark Cave", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lake Hylia Ledge Chest", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lakeside Item Shop (1)", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lakeside Item Shop (2)", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lakeside Item Shop (3)", regions::hyrule::lake::hylia::SUBREGION, Empty);
    layout.set_item("Lake Hylia Eastern Shore", regions::hyrule::lake::hylia::SUBREGION, Empty);

    layout.set_item("[Mai] Hyrule Hotfoot Rock", regions::hyrule::lake::hylia::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lake Hylia Island Tile", regions::hyrule::lake::hylia::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lake Hylia SE Wall", regions::hyrule::lake::hylia::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lake Hylia Shallow Ring", regions::hyrule::lake::hylia::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Outside Maiamai Cave", regions::hyrule::lake::hylia::SUBREGION, maiamai.pop().unwrap());

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set_item("Pegasus Boots Pyramid", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Vacant House", regions::lorule::field::main::SUBREGION, Bottle03);
    layout.set_item("Rupee Rush (Lorule)", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Great Rupee Fairy", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Big Bomb Flower Cave", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Octoball Derby", regions::lorule::field::main::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Blacksmith (Lorule)", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Swamp Cave (Left)", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Swamp Cave (Middle)", regions::lorule::field::main::SUBREGION, heart_pieces.pop().unwrap());
    layout.set_item("Swamp Cave (Right)", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Thief Girl", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Lorule Field Hookshot Chest", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("Fortune's Choice", regions::lorule::field::main::SUBREGION, Empty);
    layout.set_item("[Mai] Thieves' Town Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Fortune-Teller Rock", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Castle Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Castle Tree", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Blacksmith Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Rupee Rush Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Big Bomb Flower Grass", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Octoball Derby Skull", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Vacant House Rock", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Behind Vacant House", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Haunted Grove Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule S Ruins Pillars", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule S Ruins Wall", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule S Ruins Water", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Thieves' Town Tree", regions::lorule::field::main::SUBREGION, maiamai.pop().unwrap());

    // Thieves' Town Shop
    layout.set_item("Thieves' Town Item Shop (1)", regions::lorule::field::main::SUBREGION, Bee01);
    layout.set_item("Thieves' Town Item Shop (2)", regions::lorule::field::main::SUBREGION, GoldBee01);
    layout.set_item("Thieves' Town Item Shop (3)", regions::lorule::field::main::SUBREGION, Fairy01);
    layout.set_item("Thieves' Town Item Shop (4)", regions::lorule::field::main::SUBREGION, Shield03);

    // Lorule Lakeside Item Shop
    layout.set_item("Lorule Lakeside Item Shop (1)", regions::lorule::lake::lorule::SUBREGION, Bee02);
    layout.set_item("Lorule Lakeside Item Shop (2)", regions::lorule::lake::lorule::SUBREGION, GoldBee02);
    layout.set_item("Lorule Lakeside Item Shop (3)", regions::lorule::lake::lorule::SUBREGION, Fairy02);
    layout.set_item("Lorule Lakeside Item Shop (4)", regions::lorule::lake::lorule::SUBREGION, Shield04);

    // Skull Woods (overworld)
    layout.set_item("n-Shaped House", regions::lorule::skull::overworld::SUBREGION, Empty);
    layout.set_item("Destroyed House", regions::lorule::skull::overworld::SUBREGION, Empty);
    layout.set_item("Mysterious Man", regions::lorule::skull::overworld::SUBREGION, Empty);
    layout.set_item("[Mai] n-Shaped House Wall", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Rock", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Bush", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Dry Pond", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Entrance Wall", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Grass", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Destroyed House Tree", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Skull Woods Skull", regions::lorule::skull::overworld::SUBREGION, maiamai.pop().unwrap());

    // Lorule Death Mountain
    layout.set_item("Ice Cave Ledge", regions::lorule::death::mountain::SUBREGION, Empty);
    layout.set_item("Behind Ice Gimos", regions::lorule::death::mountain::SUBREGION, Empty);
    layout.set_item("Lorule Mountain W Ledge", regions::lorule::death::mountain::SUBREGION, Empty);
    layout.set_item("Ice Gimos Fight", regions::lorule::death::mountain::SUBREGION, Empty);
    layout.set_item("Treacherous Tower", regions::lorule::death::mountain::SUBREGION, Empty);
    layout.set_item("[Mai] Lorule Mountain W Skull", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Mountain W Big Rock", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Ice Cave Ledge", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Mountain E Wall", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Outside Ice Ruins", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Mountain E Big Rock", regions::lorule::death::mountain::SUBREGION, maiamai.pop().unwrap());

    // Dark Ruins
    layout.set_item("Dark Ruins Lakeview Chest", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Dark Maze Chest", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Dark Maze Ledge", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (1)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (2)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (3)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (4)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (5)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Hinox (6)", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("Ku's Domain Fight", regions::lorule::dark::ruins::SUBREGION, Empty);
    layout.set_item("[Mai] Dark Ruins Bonk Rocks", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Maze Center Wall", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Maze Entrance Wall", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Ruins East Tree", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Ruins South Wall", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Ruins Waterfall", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Dark Ruins West Tree", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Ku's Domain Grass", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Ku's Domain Water", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Outside Hinox Cave", regions::lorule::dark::ruins::SUBREGION, maiamai.pop().unwrap());

    // Misery Mire
    layout.set_item("Misery Mire Ledge", regions::lorule::misery::mire::SUBREGION, Empty);
    layout.set_item("Sand Mini-Dungeon", regions::lorule::misery::mire::SUBREGION, Empty);
    layout.set_item("[Mai] Misery Mire Water", regions::lorule::misery::mire::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Misery Mire Wall", regions::lorule::misery::mire::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Misery Mire Rock", regions::lorule::misery::mire::SUBREGION, maiamai.pop().unwrap());

    // Lorule Lake
    layout.set_item("Lorule Lake Chest", regions::lorule::lake::lorule::SUBREGION, Empty);
    layout.set_item("[Mai] Lorule Lake Rock", regions::lorule::lake::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Lake SE Wall", regions::lorule::lake::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Lake Skull", regions::lorule::lake::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Lake Water", regions::lorule::lake::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Lake West Wall", regions::lorule::lake::lorule::SUBREGION, maiamai.pop().unwrap());

    //////////////////////////
    // --- Mini Dungeon --- //
    //////////////////////////

    // Graveyard (Hyrule)
    layout.set_item("Dampe", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("Sanctuary Pegs", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("[HS] Entrance", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("[HS] Lower Chest", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("[HS] Upper Chest", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("[HS] Ledge", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("Graveyard Ledge Cave", regions::dungeons::graveyards::hyrule::SUBREGION, Empty);
    layout.set_item("[Mai] Sanctuary Wall", regions::dungeons::graveyards::hyrule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Hyrule Graveyard Wall", regions::dungeons::graveyards::hyrule::SUBREGION, maiamai.pop().unwrap());

    // Graveyard (Lorule)
    layout.set_item("Graveyard Peninsula", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("Philosopher's Cave", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("[LS] Entrance Chest", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("[LS] Ledge", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("[LS] Lower Chest", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("[LS] Upper Chest", regions::dungeons::graveyards::lorule::SUBREGION, Empty);
    layout.set_item("[Mai] Lorule Graveyard Big Rock", regions::dungeons::graveyards::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Graveyard Tree", regions::dungeons::graveyards::lorule::SUBREGION, maiamai.pop().unwrap());
    layout.set_item("[Mai] Lorule Graveyard Wall", regions::dungeons::graveyards::lorule::SUBREGION, maiamai.pop().unwrap());

    layout.set_item("Graveyard Weather Vane", regions::dungeons::graveyards::lorule::SUBREGION, GraveyardWV);

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set_item("[EP] (1F) Merge Chest", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (1F) Left Door Chest", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (1F) Popo Room", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (1F) Secret Room", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (1F) Switch Room", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (2F) Ball Room", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (2F) Defeat Popos", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (2F) Switch Room", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (2F) Big Chest", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] Yuga (1)", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] Yuga (2)", regions::dungeons::eastern::palace::SUBREGION, heart_containers.pop().unwrap());
    layout.set_item("[EP] (3F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION, Empty);
    layout.set_item("[EP] (1F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION, Empty);

    // House of Gales
    layout.set_item("[HG] (1F) Torches", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (1F) Switch Room", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (1F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (1F) West Room", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (1F) West Room Secret", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (2F) Big Chest", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (2F) Narrow Ledge", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (2F) Fire Ring", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (3F) Rat Room", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] (3F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION, Empty);
    layout.set_item("[HG] Margomill", regions::dungeons::house::gales::SUBREGION, heart_containers.pop().unwrap());

    // Tower of Hera
    layout.set_item("[TH] (1F) Outside", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (1F) Center", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (3F) Platform", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (5F) Red/Blue Switches", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (6F) Left Mole", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (6F) Right Mole", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (7F) Outside (Ledge)", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (8F) Fairy Room", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] (11F) Big Chest", regions::dungeons::tower::hera::SUBREGION, Empty);
    layout.set_item("[TH] Moldorm", regions::dungeons::tower::hera::SUBREGION, heart_containers.pop().unwrap());

    // Hyrule Castle
    layout.set_item("[HC] Battlement", regions::dungeons::hyrule::castle::SUBREGION, Empty);
    layout.set_item("[HC] West Wing", regions::dungeons::hyrule::castle::SUBREGION, Empty);

    // Dark Palace
    layout.set_item("[PD] (1F) Right Pit", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (1F) Left Pit", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (1F) Switch Puzzle", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (1F) Hidden Room (Upper)", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (1F) Hidden Room (Lower)", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (B1) Fall From 1F", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (B1) Glow-in-the-Dark Maze", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (B1) Helmasaur Room", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (B1) Helmasaur Room (Fall)", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (2F) Big Chest (Hidden)", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (2F) South Hidden Room", regions::dungeons::dark::palace::SUBREGION, gold_rupees.pop().unwrap());
    layout.set_item("[PD] (2F) Alcove", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (1F) Fall From 2F", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] (B1) Bomb Bowling", regions::dungeons::dark::palace::SUBREGION, Empty);
    layout.set_item("[PD] Gemesaur King", regions::dungeons::dark::palace::SUBREGION, heart_containers.pop().unwrap());

    // Swamp Palace
    layout.set_item("[SP] (B1) Center", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Raft Room (Left)", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Raft Room (Right)", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Gyorm", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Waterfall Room", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Raft Room (Pillar)", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (B1) Big Chest (Secret)", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (1F) Water Puzzle", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (1F) East Room", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (1F) West Room", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] (1F) Big Chest (Fire)", regions::dungeons::swamp::palace::SUBREGION, Empty);
    layout.set_item("[SP] Arrghus", regions::dungeons::swamp::palace::SUBREGION, heart_containers.pop().unwrap());

    // Skull Woods
    layout.set_item("[SW] (B1) Gibdo Room (Lower)", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B1) South Chest", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B1) Gibdo Room (Hole)", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B1) Grate Room", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B2) Moving Platform Room", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B1) Big Chest (Eyes)", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] Outdoor Chest", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] (B1) Big Chest (Upper)", regions::dungeons::skull::woods::SUBREGION, Empty);
    layout.set_item("[SW] Knucklemaster", regions::dungeons::skull::woods::SUBREGION, heart_containers.pop().unwrap());

    // Thieves' Hideout
    layout.set_item("[TT] (B1) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B1) Grate Chest", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B2) Grate Chest (Fall)", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B2) Switch Puzzle Room", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B2) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B2) Eyegores", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B1) Behind Wall", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B1) Big Chest (Entrance)", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B3) Underwater", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] (B3) Big Chest (Hidden)", regions::dungeons::thieves::hideout::SUBREGION, Empty);
    layout.set_item("[TT] Stalblind", regions::dungeons::thieves::hideout::SUBREGION, heart_containers.pop().unwrap());

    // Ice Ruins
    layout.set_item("[IR] (1F) Hidden Chest", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B3) Grate Chest (Left)", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B3) Grate Chest (Right)", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B4) Ice Pillar", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B5) Big Chest", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B1) East Chest", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B1) Narrow Ledge", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B1) Upper Chest", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B3) Big Chest (Puzzle)", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B4) Switches", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B4) Southwest Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B4) Narrow Platform", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B2) Long Merge Chest", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] (B4) Southeast Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION, Empty);
    layout.set_item("[IR] Dharkstare", regions::dungeons::ice::ruins::SUBREGION, heart_containers.pop().unwrap());

    // Desert Palace
    layout.set_item("[DP] (1F) Entrance", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (1F) Sand Room (South)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (1F) Sand Switch Room", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (1F) Sand Room (North)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (1F) Big Chest (Behind Wall)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (1F) Behind Rocks", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Under Rock (Left)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Beamos Room", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Under Rock (Right)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Under Rock (Ball Room)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Big Chest (Puzzle)", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Red/Blue Switches", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (2F) Leever Room", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (3F) Behind Falling Sand", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] (3F) Armos Room", regions::dungeons::desert::palace::SUBREGION, Empty);
    layout.set_item("[DP] Zaganaga", regions::dungeons::desert::palace::SUBREGION, heart_containers.pop().unwrap());

    // Turtle Rock
    layout.set_item("[TR] (1F) Center", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Grate Chest", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Portal Room NW", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Northeast Ledge", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Southeast Chest", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Defeat Flamolas", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] Left Balcony", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (B1) Northeast Room", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (B1) Grate Chest (Small)", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (B1) Big Chest (Center)", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (B1) Platform", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (B1) Big Chest (Top)", regions::dungeons::turtle::rock::SUBREGION, Empty);
    layout.set_item("[TR] (1F) Under Center", regions::dungeons::turtle::rock::SUBREGION, silver_rupees.pop().unwrap());
    layout.set_item("[TR] (B1) Under Center", regions::dungeons::turtle::rock::SUBREGION, gold_rupees.pop().unwrap());
    layout.set_item("[TR] Grinexx", regions::dungeons::turtle::rock::SUBREGION, heart_containers.pop().unwrap());

    // Lorule Castle
    layout.set_item("[LC] (1F) Ledge", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (1F) Center", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (2F) Near Torches", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (2F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (2F) Ledge", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (4F) Center", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] (4F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Bomb Trial I", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Bomb Trial II", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Tile Trial I", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Tile Trial II", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Lamp Trial", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Hook Trial I", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Hook Trial II", regions::dungeons::lorule::castle::SUBREGION, Empty);
    layout.set_item("[LC] Zelda", regions::dungeons::lorule::castle::SUBREGION, Empty);

    info!("Successfully Built Layout");

    layout
}
