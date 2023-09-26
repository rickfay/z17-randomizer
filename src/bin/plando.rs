use game::Item::*;
use log::{error, info, LevelFilter};
use macros::fail;
use modinfo::settings::active_weather_vanes::ActiveWeatherVanes;
use modinfo::settings::{
    entrance_shuffle::EntranceShuffleSetting,
    hyrule_castle::HyruleCastleSetting,
    logic::{Logic, LogicMode},
    pedestal::PedestalSetting,
    Exclude, Exclusion, Options, Settings,
};
use randomizer::{
    constants::VERSION,
    regions,
    system::{System, UserConfig},
    Layout, LocationInfo, SeedHash, SeedInfo,
};
use simplelog::SimpleLogger;
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
    let settings = &plando_settings();

    let seed_info = SeedInfo {
        seed,
        version: VERSION,
        hash: SeedHash::new(seed, settings),
        settings,
        layout: build_layout(),
        metrics: Default::default(),
        hints: Default::default(),
    };

    seed_info.settings.log_settings();

    match randomizer::patch_seed(&seed_info, &user_config, args.no_patch, args.no_spoiler) {
        Ok(_) => {
            println!();
            info!("Successfully Generated ALBW Plandomizer Seed");
        }
        Err(err) => {
            println!();
            error!("Plandomizer execution failed:\n{}", err.into_inner());
        }
    }

    cli::pause();
}

fn plando_settings() -> Settings {
    Settings {
        dev_mode: true,
        logic: Logic {
            logic_mode: LogicMode::Normal,
            randomize_dungeon_prizes: true,
            vanilla_charm: false,
            lc_requirement: 1,
            yuganon_requirement: 1,
            ped_requirement: PedestalSetting::Standard,
            hyrule_castle_setting: HyruleCastleSetting::EarlyLoruleCastle,

            nice_mode: false,
            super_items: true,
            reverse_sage_events: true,
            no_progression_enemies: true,
            entrance_rando: EntranceShuffleSetting::NotShuffled,

            start_with_merge: true,
            bell_in_shop: false,
            pouch_in_shop: false,
            sword_in_shop: false,
            boots_in_shop: false,
            assured_weapon: false,

            maiamai_madness: false,

            minigames_excluded: false,
            skip_big_bomb_flower: false,
            skip_trials: false,
            bow_of_light_in_castle: false,

            active_weather_vanes: ActiveWeatherVanes::All,
            dark_rooms_lampless: false,
            swordless_mode: false,

            hint_ghost_price: 30,
        },
        options: Options { chest_size_matches_contents: true, night_mode: false },
        exclusions: Exclusion(Default::default()),
        exclude: Exclude::new(),
    }
}

#[rustfmt::skip]
fn build_layout() -> Layout {

    info!("Building Item Layout from Plan...");
    let mut layout = Layout::default();

    //////////////////////////
    // --- Ravio's Shop --- //
    //////////////////////////

    // layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Thanks"), Item::RingHekiga);

    layout.set(LocationInfo::new("Ravio (1)", regions::hyrule::field::main::SUBREGION), OreBlue);
    layout.set(LocationInfo::new("Ravio (2)", regions::hyrule::field::main::SUBREGION), RupeeG);
    layout.set(LocationInfo::new("Ravio (3)", regions::hyrule::field::main::SUBREGION), OreGreen);
    layout.set(LocationInfo::new("Ravio (4)", regions::hyrule::field::main::SUBREGION), RupeeG);
    layout.set(LocationInfo::new("Ravio (5)", regions::hyrule::field::main::SUBREGION), RupeeG);
    layout.set(LocationInfo::new("Ravio (6)", regions::hyrule::field::main::SUBREGION), OreYellow);
    layout.set(LocationInfo::new("Ravio (7)", regions::hyrule::field::main::SUBREGION), RupeeG);
    layout.set(LocationInfo::new("Ravio (8)", regions::hyrule::field::main::SUBREGION), OreRed);
    layout.set(LocationInfo::new("Ravio (9)", regions::hyrule::field::main::SUBREGION), RupeeGold);

    /////////////////////////////
    // --- Dungeons Prizes --- //
    /////////////////////////////

    layout.set(LocationInfo::new("Eastern Palace Prize", regions::dungeons::eastern::palace::SUBREGION), PendantCourage);
    layout.set(LocationInfo::new("House of Gales Prize", regions::dungeons::house::gales::SUBREGION), SageRosso);
    layout.set(LocationInfo::new("Tower of Hera Prize", regions::dungeons::tower::hera::SUBREGION), SageGulley);

    layout.set(LocationInfo::new("Hyrule Castle Prize", regions::dungeons::hyrule::castle::SUBREGION), PendantCourage);

    layout.set(LocationInfo::new("Dark Palace Prize", regions::dungeons::dark::palace::SUBREGION), PendantPower);
    layout.set(LocationInfo::new("Swamp Palace Prize", regions::dungeons::swamp::palace::SUBREGION), SageOren);
    layout.set(LocationInfo::new("Skull Woods Prize", regions::dungeons::skull::woods::SUBREGION), SageSeres);
    layout.set(LocationInfo::new("Thieves' Hideout Prize", regions::dungeons::thieves::hideout::SUBREGION), SageOsfala);
    layout.set(LocationInfo::new("Turtle Rock Prize", regions::dungeons::turtle::rock::SUBREGION), SageImpa);
    layout.set(LocationInfo::new("Desert Palace Prize", regions::dungeons::desert::palace::SUBREGION), SageIrene);
    layout.set(LocationInfo::new("Ice Ruins Prize", regions::dungeons::ice::ruins::SUBREGION), PendantWisdom);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(LocationInfo::new("Cucco Treasure Dungeon", regions::hyrule::field::main::SUBREGION), RupeeSilver);
    layout.set(LocationInfo::new("Delivery", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Behind Blacksmith", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Blacksmith Cave", regions::hyrule::field::main::SUBREGION), ItemSwordLv1);
    layout.set(LocationInfo::new("Blacksmith", regions::hyrule::field::main::SUBREGION), ItemSwordLv1);
    layout.set(LocationInfo::new("Blacksmith Table", regions::hyrule::field::main::SUBREGION), ItemSwordLv1);
    layout.set(LocationInfo::new("Hyrule Castle Rocks", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Wildlife Clearing Stump", regions::hyrule::field::main::SUBREGION), RupeeGold);

    layout.set(LocationInfo::new("[Mai] Tree Behind Blacksmith", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Behind Link's House", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Blacksmith Tornado Tile", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Cucco Dungeon Big Rock", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Wildlife Clearing Tree", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Hyrule Castle Tree", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Hyrule Castle Tornado Tile", regions::hyrule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Tree West of Link's House", regions::hyrule::field::main::SUBREGION), RupeeGold);

    // Irene the Witch
    layout.set(LocationInfo::new("Irene", regions::hyrule::irene::witch::SUBREGION), RupeeGold);

    // Lost Woods
    layout.set(LocationInfo::new("Fortune-Teller", regions::hyrule::lost::woods::SUBREGION), RingRental);
    layout.set(LocationInfo::new("Hyrule Hotfoot (First Race)", regions::hyrule::lost::woods::SUBREGION), HintGlasses);
    layout.set(LocationInfo::new("Hyrule Hotfoot (Second Race)", regions::hyrule::lost::woods::SUBREGION), RupeeSilver);
    layout.set(LocationInfo::new("Lost Woods Alcove", regions::hyrule::lost::woods::SUBREGION), ItemHookShot);
    layout.set(LocationInfo::new("Lost Woods Big Rock Chest", regions::hyrule::lost::woods::SUBREGION), ItemIceRod);
    layout.set(LocationInfo::new("Master Sword Pedestal", regions::hyrule::lost::woods::SUBREGION), ItemIceRod);
    layout.set(LocationInfo::new("Rosso", regions::hyrule::lost::woods::SUBREGION), ItemIceRod);
    layout.set(LocationInfo::new("Rosso Cave", regions::hyrule::lost::woods::SUBREGION), ItemInsectNet);
    layout.set(LocationInfo::new("Rosso Rocks", regions::hyrule::lost::woods::SUBREGION), RupeeGold);

    layout.set(LocationInfo::new("[Mai] Rosso Wall", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lost Woods Path Rock", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lost Woods Bush", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lost Woods Rock", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Fortune-Teller Tent", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Moldorm Ledge", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Small Pond", regions::hyrule::lost::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lost Woods Tree", regions::hyrule::lost::woods::SUBREGION), RupeeGold);

    // Death Mountain
    layout.set(LocationInfo::new("Death Mountain Open Cave", regions::hyrule::death::mountain::SUBREGION), PowerGlove);
    layout.set(LocationInfo::new("Death Mountain Blocked Cave", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Death Mountain Fairy Cave", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Death Mountain West Ledge", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Donkey Cave Pegs", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Death Mountain West Highest Cave", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Spectacle Rock", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Fire Cave Pillar", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Bouldering Guy", regions::hyrule::death::mountain::SUBREGION), ItemBottle);
    layout.set(LocationInfo::new("Death Mountain Treasure Dungeon", regions::hyrule::death::mountain::SUBREGION), ItemHookShotLv2);
    layout.set(LocationInfo::new("Floating Island", regions::hyrule::death::mountain::SUBREGION), RupeeGold);

    layout.set(LocationInfo::new("[Mai] Death Mountain Base Rock", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Death Mountain West Ledge", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Death Mountain East Ledge", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Rosso's Ore Mine Rock", regions::hyrule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Outside Hookshot Dungeon", regions::hyrule::death::mountain::SUBREGION), RupeeGold);

    // Kakariko
    layout.set(LocationInfo::new("Bee Guy (1)", regions::hyrule::kakariko::village::SUBREGION), HintGlasses);
    layout.set(LocationInfo::new("Bee Guy (2)", regions::hyrule::kakariko::village::SUBREGION), ItemFireRod);
    layout.set(LocationInfo::new("Dodge the Cuccos", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Kakariko Item Shop (1)", regions::hyrule::kakariko::village::SUBREGION), EscapeFruit);
    layout.set(LocationInfo::new("Kakariko Item Shop (2)", regions::hyrule::kakariko::village::SUBREGION), StopFruit);
    layout.set(LocationInfo::new("Kakariko Item Shop (3)", regions::hyrule::kakariko::village::SUBREGION), ItemShield);
    layout.set(LocationInfo::new("Kakariko Jail", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Kakariko Well (Bottom)", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Kakariko Well (Top)", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Rupee Rush (Hyrule)", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Shady Guy", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Street Merchant (Left)", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Street Merchant (Right)", regions::hyrule::kakariko::village::SUBREGION), LiverYellow);
    layout.set(LocationInfo::new("Stylish Woman", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Woman", regions::hyrule::kakariko::village::SUBREGION), RupeeR);

    layout.set(LocationInfo::new("[Mai] Cucco Ranch Tree", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Hyrule Rupee Rush Wall", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Kakariko Bush", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Kakariko Sand", regions::hyrule::kakariko::village::SUBREGION), ItemBowLight);
    layout.set(LocationInfo::new("[Mai] Woman's Roof Rock", regions::hyrule::kakariko::village::SUBREGION), RupeeGold);

    // Zora's River
    layout.set(LocationInfo::new("Queen Oren", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Waterfall Cave", regions::hyrule::zora::river::SUBREGION), Kinsta);
    layout.set(LocationInfo::new("Zora's Domain Ledge", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Zora's River Treasure Dungeon", regions::hyrule::zora::river::SUBREGION), ItemBoomerangLv2);

    layout.set(LocationInfo::new("[Mai] Inside Witch's House", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Under Wooden Bridge", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Waterfall Ledge Wall", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Zora's Domain South Wall", regions::hyrule::zora::river::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Zora's Domain Water", regions::hyrule::zora::river::SUBREGION), RupeeGold);

    // Eastern Ruins
    layout.set(LocationInfo::new("Bird Lover", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Eastern Ruins Treasure Dungeon", regions::hyrule::eastern::ruins::SUBREGION), ItemHammerLv2);
    layout.set(LocationInfo::new("Eastern Ruins Armos Chest", regions::hyrule::eastern::ruins::SUBREGION), ItemTornadeRod);
    layout.set(LocationInfo::new("Eastern Ruins Hookshot Chest", regions::hyrule::eastern::ruins::SUBREGION), ItemSandRod);
    layout.set(LocationInfo::new("Eastern Ruins Merge Chest", regions::hyrule::eastern::ruins::SUBREGION), ItemBoomerang);
    layout.set(LocationInfo::new("Eastern Ruins Cave", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Eastern Ruins Peg Circle", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);

    layout.set(LocationInfo::new("[Mai] Atop Eastern Rocks", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Eastern Ruins Big Rock", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Eastern Ruins Green Tree", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Eastern Ruins Wall", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Eastern Ruins Yellow Tree", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Southern Bridge River", regions::hyrule::eastern::ruins::SUBREGION), RupeeGold);

    // Desert of Mystery
    layout.set(LocationInfo::new("[Mai] Buried in the Desert", regions::hyrule::desert::mystery::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Buried near Desert Palace", regions::hyrule::desert::mystery::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Southern Ruins Big Rock", regions::hyrule::desert::mystery::SUBREGION), RupeeGold);

    // Southern Ruins
    layout.set(LocationInfo::new("Runaway Item Seller", regions::hyrule::southern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Southern Ruins Ledge", regions::hyrule::southern::ruins::SUBREGION), ItemHammer);
    layout.set(LocationInfo::new("Southern Ruins Pillar Cave", regions::hyrule::southern::ruins::SUBREGION), ItemBowLight);
    layout.set(LocationInfo::new("Southern Ruins Treasure Dungeon", regions::hyrule::southern::ruins::SUBREGION), ItemHammer);

    layout.set(LocationInfo::new("[Mai] Outside Flippers Dungeon", regions::hyrule::southern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Southern Ruins Bomb Cave", regions::hyrule::southern::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Southern Ruins Pillars", regions::hyrule::southern::ruins::SUBREGION), RupeeGold);

    // Lake Hylia
    layout.set(LocationInfo::new(" 10 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemBowLv2);
    layout.set(LocationInfo::new(" 20 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemBoomerangLv2);
    layout.set(LocationInfo::new(" 30 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemHookShotLv2);
    layout.set(LocationInfo::new(" 40 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemHammerLv2);
    layout.set(LocationInfo::new(" 50 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemBombLv2);
    layout.set(LocationInfo::new(" 60 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemFireRodLv2);
    layout.set(LocationInfo::new(" 70 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemIceRodLv2);
    layout.set(LocationInfo::new(" 80 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemTornadeRodLv2);
    layout.set(LocationInfo::new(" 90 Maiamai", regions::hyrule::lake::hylia::SUBREGION), ItemSandRodLv2);
    layout.set(LocationInfo::new("100 Maiamai", regions::hyrule::lake::hylia::SUBREGION), SpecialMove);

    layout.set(LocationInfo::new("Ice Rod Cave", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Lake Hylia Dark Cave", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Lake Hylia Ledge Chest", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Lakeside Item Shop (1)", regions::hyrule::lake::hylia::SUBREGION), EscapeFruit);
    layout.set(LocationInfo::new("Lakeside Item Shop (2)", regions::hyrule::lake::hylia::SUBREGION), StopFruit);
    layout.set(LocationInfo::new("Lakeside Item Shop (3)", regions::hyrule::lake::hylia::SUBREGION), ItemShield);
    layout.set(LocationInfo::new("Southeastern Shore", regions::hyrule::lake::hylia::SUBREGION), HintGlasses);

    layout.set(LocationInfo::new("[Mai] Hyrule Hotfoot Big Rock", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Island Tornado Tile", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lake Hylia SE Wall", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lake Hylia Shallow Ring", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Outside Maiamai Cave", regions::hyrule::lake::hylia::SUBREGION), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(LocationInfo::new("Lorule Field Treasure Dungeon", regions::lorule::field::main::SUBREGION), GanbariPowerUp);
    layout.set(LocationInfo::new("Vacant House", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Rupee Rush (Lorule)", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Great Rupee Fairy", regions::lorule::field::main::SUBREGION), ItemIceRod);
    layout.set(LocationInfo::new("Big Bomb Flower Cave", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Octoball Derby", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Blacksmith (Lorule)", regions::lorule::field::main::SUBREGION), ItemKandelaar);
    layout.set(LocationInfo::new("Swamp Cave (Left)", regions::lorule::field::main::SUBREGION), ItemBowLight);
    layout.set(LocationInfo::new("Swamp Cave (Middle)", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Swamp Cave (Right)", regions::lorule::field::main::SUBREGION), LiverBlue);
    layout.set(LocationInfo::new("Thief Girl", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Lorule Field Hookshot Chest", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Fortune's Choice", regions::lorule::field::main::SUBREGION), HeartPiece);
    layout.set(LocationInfo::new("[Mai] Thieves' Town Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Near Lorule Fortune-Teller", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Castle Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Castle Tree", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Blacksmith Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Rupee Rush Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Big Bomb Flower Grass", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Octoball Derby Skull", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Vacant House Big Rock", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Behind Vacant House", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Haunted Grove Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule S Ruins Pillars", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule S Ruins Wall", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule S Ruins Water", regions::lorule::field::main::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Thieves' Town Tree", regions::lorule::field::main::SUBREGION), RupeeGold);

    // Thieves' Town Shop
    layout.set(LocationInfo::new("Thieves' Town Item Shop (1)", regions::lorule::field::main::SUBREGION), Bee);
    layout.set(LocationInfo::new("Thieves' Town Item Shop (2)", regions::lorule::field::main::SUBREGION), GoldenBeeForSale);
    layout.set(LocationInfo::new("Thieves' Town Item Shop (3)", regions::lorule::field::main::SUBREGION), Fairy);
    layout.set(LocationInfo::new("Thieves' Town Item Shop (4)", regions::lorule::field::main::SUBREGION), ItemShield);

    // Lorule Lakeside Item Shop
    layout.set(LocationInfo::new("Lorule Lakeside Item Shop (1)", regions::lorule::lake::lorule::SUBREGION), Bee);
    layout.set(LocationInfo::new("Lorule Lakeside Item Shop (2)", regions::lorule::lake::lorule::SUBREGION), GoldenBeeForSale);
    layout.set(LocationInfo::new("Lorule Lakeside Item Shop (3)", regions::lorule::lake::lorule::SUBREGION), Fairy);
    layout.set(LocationInfo::new("Lorule Lakeside Item Shop (4)", regions::lorule::lake::lorule::SUBREGION), ItemShield);

    // Chamber of Sages
    layout.set(LocationInfo::new("Osfala", regions::lorule::chamber::sages::SUBREGION), DashBoots);

    // Skull Woods (overworld)
    layout.set(LocationInfo::new("Canyon House", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Destroyed House", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Mysterious Man", regions::lorule::skull::overworld::SUBREGION), GoldenBeeForSale);
    layout.set(LocationInfo::new("[Mai] Canyon House Wall", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Big Rock", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Bush", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Dry Pond", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Entrance Wall", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Grass", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Shack Tree", regions::lorule::skull::overworld::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Skull Woods Skull", regions::lorule::skull::overworld::SUBREGION), RupeeGold);

    // Lorule Death Mountain
    layout.set(LocationInfo::new("Lorule Mountain E Ledge", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Behind Ice Gimos", regions::lorule::death::mountain::SUBREGION), ItemFireRodLv2);
    layout.set(LocationInfo::new("Lorule Mountain W Ledge", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Ice Gimos Fight", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Treacherous Tower Intermediate", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Treacherous Tower Advanced (1)", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Treacherous Tower Advanced (2)", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Mountain W Skull", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Mountain W Big Rock", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Mountain E Skull", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Mountain E Wall", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Outside Ice Ruins", regions::lorule::death::mountain::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Mountain E Big Rock", regions::lorule::death::mountain::SUBREGION), RupeeGold);

    // Dark Ruins
    layout.set(LocationInfo::new("Dark Ruins Lakeview Chest", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Dark Maze Chest", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Dark Maze Ledge", regions::lorule::dark::ruins::SUBREGION), HeartPiece);
    layout.set(LocationInfo::new("Hinox (1)", regions::lorule::dark::ruins::SUBREGION), RupeeG);
    layout.set(LocationInfo::new("Hinox (2)", regions::lorule::dark::ruins::SUBREGION), RupeeB);
    layout.set(LocationInfo::new("Hinox (3)", regions::lorule::dark::ruins::SUBREGION), RupeeR);
    layout.set(LocationInfo::new("Hinox (4)", regions::lorule::dark::ruins::SUBREGION), RupeePurple);
    layout.set(LocationInfo::new("Hinox (5)", regions::lorule::dark::ruins::SUBREGION), RupeeSilver);
    layout.set(LocationInfo::new("Hinox (6)", regions::lorule::dark::ruins::SUBREGION), SpecialMove);
    layout.set(LocationInfo::new("Ku's Domain Fight", regions::lorule::dark::ruins::SUBREGION), ItemMizukaki);
    layout.set(LocationInfo::new("[Mai] Atop Dark Ruins Rocks", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Maze Center Wall", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Maze Entrance Wall", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Ruins East Tree", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Ruins South Area Wall", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Ruins Waterfall", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Dark Ruins West Tree", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Ku's Domain Grass", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Ku's Domain Water", regions::lorule::dark::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Outside Hinox Cave", regions::lorule::dark::ruins::SUBREGION), RupeeGold);

    // Misery Mire
    layout.set(LocationInfo::new("Misery Mire Ledge", regions::lorule::misery::mire::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Misery Mire Treasure Dungeon", regions::lorule::misery::mire::SUBREGION), ItemSandRodLv2);
    layout.set(LocationInfo::new("[Mai] Misery Mire Water", regions::lorule::misery::mire::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Misery Mire Wall", regions::lorule::misery::mire::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Misery Mire Big Rock", regions::lorule::misery::mire::SUBREGION), RupeeGold);

    // Lorule Lake
    layout.set(LocationInfo::new("Lorule Lake Chest", regions::lorule::lake::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Lake Big Rock", regions::lorule::lake::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Lake SE Wall", regions::lorule::lake::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Lake Skull", regions::lorule::lake::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Lake Water", regions::lorule::lake::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Lake West Wall", regions::lorule::lake::lorule::SUBREGION), RupeeGold);

    //////////////////////////
    // --- Mini Dungeon --- //
    //////////////////////////

    // Graveyard (Hyrule)
    layout.set(LocationInfo::new("Dampe", regions::dungeons::graveyards::hyrule::SUBREGION), RingRental);
    layout.set(LocationInfo::new("Sanctuary Pegs", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HS] Entrance", regions::dungeons::graveyards::hyrule::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[HS] Lower Chest", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HS] Upper Chest", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HS] Ledge", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Graveyard Ledge Cave", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Sanctuary Wall", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Hyrule Graveyard Wall", regions::dungeons::graveyards::hyrule::SUBREGION), RupeeGold);

    // Graveyard (Lorule)
    layout.set(LocationInfo::new("Graveyard Peninsula", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Philosopher's Cave", regions::dungeons::graveyards::lorule::SUBREGION), OreBlue);
    layout.set(LocationInfo::new("[LS] Entrance Chest", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LS] Ledge", regions::dungeons::graveyards::lorule::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[LS] Lower Chest", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LS] Upper Chest", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Graveyard Big Rock", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Graveyard Tree", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[Mai] Lorule Graveyard Wall", regions::dungeons::graveyards::lorule::SUBREGION), RupeeGold);

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set(LocationInfo::new("[EP] (1F) Merge Chest", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (1F) Left Door Chest", regions::dungeons::eastern::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[EP] (1F) Popo Room", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (1F) Secret Room", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (1F) Switch Room", regions::dungeons::eastern::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[EP] (2F) Ball Room", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (2F) Defeat Popos", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (2F) Switch Room", regions::dungeons::eastern::palace::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[EP] (2F) Big Chest", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] Yuga (1)", regions::dungeons::eastern::palace::SUBREGION), HintGlasses);
    layout.set(LocationInfo::new("[EP] Yuga (2)", regions::dungeons::eastern::palace::SUBREGION), HeartContainer);
    layout.set(LocationInfo::new("[EP] (3F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[EP] (1F) Escape Chest", regions::dungeons::eastern::palace::SUBREGION), RupeeGold);

    // House of Gales
    layout.set(LocationInfo::new("[HG] (1F) Torches", regions::dungeons::house::gales::SUBREGION), Compass);
    layout.set(LocationInfo::new("[HG] (1F) Switch Room", regions::dungeons::house::gales::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[HG] (1F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HG] (1F) West Room", regions::dungeons::house::gales::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HG] (1F) West Room Secret", regions::dungeons::house::gales::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HG] (2F) Big Chest", regions::dungeons::house::gales::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[HG] (2F) Narrow Ledge", regions::dungeons::house::gales::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[HG] (2F) Fire Ring", regions::dungeons::house::gales::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[HG] (3F) Rat Room", regions::dungeons::house::gales::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[HG] (3F) Fire Bubbles", regions::dungeons::house::gales::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[HG] Margomill", regions::dungeons::house::gales::SUBREGION), HintGlasses);

    // Tower of Hera
    layout.set(LocationInfo::new("[TH] (1F) Outside", regions::dungeons::tower::hera::SUBREGION), Compass);
    layout.set(LocationInfo::new("[TH] (1F) Center", regions::dungeons::tower::hera::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TH] (3F) Platform", regions::dungeons::tower::hera::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TH] (5F) Red/Blue Switches", regions::dungeons::tower::hera::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TH] (6F) Left Mole", regions::dungeons::tower::hera::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[TH] (6F) Right Mole", regions::dungeons::tower::hera::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[TH] (7F) Outside (Ledge)", regions::dungeons::tower::hera::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TH] (8F) Fairy Room", regions::dungeons::tower::hera::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TH] (11F) Big Chest", regions::dungeons::tower::hera::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[TH] Moldorm", regions::dungeons::tower::hera::SUBREGION), HeartContainer);

    // Hyrule Castle
    layout.set(LocationInfo::new("Hyrule Castle Battlement", regions::dungeons::hyrule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Hyrule Castle West Wing", regions::dungeons::hyrule::castle::SUBREGION), RupeeGold);

    // Dark Palace
    layout.set(LocationInfo::new("[PD] (1F) Right Pit", regions::dungeons::dark::palace::SUBREGION), Compass);
    layout.set(LocationInfo::new("[PD] (1F) Left Pit", regions::dungeons::dark::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[PD] (1F) Switch Puzzle", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (1F) Hidden Room (Upper)", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (1F) Hidden Room (Lower)", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (B1) Fall From 1F", regions::dungeons::dark::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[PD] (B1) Maze", regions::dungeons::dark::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[PD] (B1) Helmasaur Room", regions::dungeons::dark::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[PD] (B1) Helmasaur Room (Fall)", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (2F) Big Chest (Hidden)", regions::dungeons::dark::palace::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[PD] (2F) South Hidden Room", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (2F) Alcove", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (1F) Fall From 2F", regions::dungeons::dark::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[PD] (B1) Big Chest (Switches)", regions::dungeons::dark::palace::SUBREGION), OreGreen);
    layout.set(LocationInfo::new("[PD] Gemesaur King", regions::dungeons::dark::palace::SUBREGION), RupeeGold);

    // Swamp Palace
    layout.set(LocationInfo::new("[SP] (B1) Center", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Raft Room (Left)", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Raft Room (Right)", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Gyorm", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Waterfall Room", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Raft Room (Pillar)", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (B1) Big Chest (Secret)", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (1F) Water Puzzle", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (1F) East Room", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (1F) West Room", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] (1F) Big Chest (Fire)", regions::dungeons::swamp::palace::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SP] Arrghus", regions::dungeons::swamp::palace::SUBREGION), KeyBoss);

    // Skull Woods
    layout.set(LocationInfo::new("[SW] (B1) Gibdo Room (Lower)", regions::dungeons::skull::woods::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SW] (B1) South Chest", regions::dungeons::skull::woods::SUBREGION), Compass);
    layout.set(LocationInfo::new("[SW] (B1) Gibdo Room (Hole)", regions::dungeons::skull::woods::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SW] (B1) Grate Room", regions::dungeons::skull::woods::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[SW] (B2) Moving Platform Room", regions::dungeons::skull::woods::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[SW] (B1) Big Chest (Eyes)", regions::dungeons::skull::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Skull Woods Outdoor Chest", regions::dungeons::skull::woods::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[SW] (B1) Big Chest (Upper)", regions::dungeons::skull::woods::SUBREGION), ItemKandelaarLv2);
    layout.set(LocationInfo::new("[SW] Knucklemaster", regions::dungeons::skull::woods::SUBREGION), OreRed);

    // Thieves' Hideout
    layout.set(LocationInfo::new("[T'H] (B1) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B1) Grate Chest", regions::dungeons::thieves::hideout::SUBREGION), Compass);
    layout.set(LocationInfo::new("[T'H] (B2) Grate Chest (Fall)", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B2) Switch Puzzle Room", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B2) Jail Cell", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B2) Eyegores", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B1) Behind Wall", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B1) Big Chest (Entrance)", regions::dungeons::thieves::hideout::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[T'H] (B3) Underwater", regions::dungeons::thieves::hideout::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[T'H] (B3) Big Chest (Hidden)", regions::dungeons::thieves::hideout::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("Stalblind", regions::dungeons::thieves::hideout::SUBREGION), OreYellow);

    // Ice Ruins
    layout.set(LocationInfo::new("[IR] (1F) Hidden Chest", regions::dungeons::ice::ruins::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[IR] (B3) Grate Chest (Left)", regions::dungeons::ice::ruins::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[IR] (B3) Grate Chest (Right)", regions::dungeons::ice::ruins::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[IR] (B4) Ice Pillar", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B5) Big Chest", regions::dungeons::ice::ruins::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[IR] (B1) East Chest", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B1) Narrow Ledge", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B1) Upper Chest", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B3) Big Chest (Puzzle)", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B4) Switches", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B4) Southwest Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B4) Narrow Platform", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B2) Long Merge Chest", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] (B4) Southeast Chest (Fall)", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[IR] Dharkstare", regions::dungeons::ice::ruins::SUBREGION), RupeeGold);

    // Desert Palace
    layout.set(LocationInfo::new("[DP] (1F) Entrance", regions::dungeons::desert::palace::SUBREGION), Compass);
    layout.set(LocationInfo::new("[DP] (1F) Sand Room (South)", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (1F) Sand Switch Room", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (1F) Sand Room (North)", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (1F) Big Chest (Behind Wall)", regions::dungeons::desert::palace::SUBREGION), KeyBoss);
    layout.set(LocationInfo::new("[DP] (1F) Behind Rocks", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Under Rock (Left)", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Beamos Room", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Under Rock (Right)", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Under Rock (Ball Room)", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Big Chest (Puzzle)", regions::dungeons::desert::palace::SUBREGION), PowerfulGlove);
    layout.set(LocationInfo::new("[DP] (2F) Red/Blue Switches", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (2F) Leever Room", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (3F) Behind Falling Sand", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[DP] (3F) Armos Room", regions::dungeons::desert::palace::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Zaganaga", regions::dungeons::desert::palace::SUBREGION), RupeeGold);

    // Turtle Rock
    layout.set(LocationInfo::new("[TR] (1F) Center", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (1F) Grate Chest", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (1F) Portal Room NW", regions::dungeons::turtle::rock::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[TR] (1F) Northeast Ledge", regions::dungeons::turtle::rock::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[TR] (1F) Southeast Chest", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (1F) Defeat Flamolas", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Turtle Rock Left Balcony", regions::dungeons::turtle::rock::SUBREGION), ItemMizukaki);
    layout.set(LocationInfo::new("[TR] (B1) Northeast Room", regions::dungeons::turtle::rock::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[TR] (B1) Grate Chest (Small)", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (B1) Big Chest (Center)", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (B1) Platform", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (B1) Big Chest (Top)", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] (1F) Under Center", regions::dungeons::turtle::rock::SUBREGION), Compass);
    layout.set(LocationInfo::new("[TR] (B1) Under Center", regions::dungeons::turtle::rock::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[TR] Grinexx", regions::dungeons::turtle::rock::SUBREGION), KeyBoss);

    // Lorule Castle
    layout.set(LocationInfo::new("[LC] (1F) Ledge", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (1F) Center", regions::dungeons::lorule::castle::SUBREGION), HintGlasses);
    layout.set(LocationInfo::new("[LC] (2F) Near Torches", regions::dungeons::lorule::castle::SUBREGION), Compass);
    layout.set(LocationInfo::new("[LC] (2F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION), KeySmall);
    layout.set(LocationInfo::new("[LC] (2F) Ledge", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (4F) Center", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (4F) Hidden Path", regions::dungeons::lorule::castle::SUBREGION), ItemBowLight);
    layout.set(LocationInfo::new("[LC] (3F) Bomb Trial Center Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (3F) Big Bomb Flower Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (3F) Merge Trial Free Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (3F) Spike Ball Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (4F) Lamp Trial Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (4F) Lava Switch Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("[LC] (4F) Eyeball Chest", regions::dungeons::lorule::castle::SUBREGION), RupeeGold);
    layout.set(LocationInfo::new("Zelda", regions::dungeons::lorule::castle::SUBREGION), ItemBow);

    info!("Successfully Built Layout");

    layout
}
