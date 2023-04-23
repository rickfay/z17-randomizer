use {
    albw::Item::*,
    log::{error, info, LevelFilter},
    randomizer::{
        cli, fail, regions,
        settings::{
            entrance_shuffle_setting::EntranceShuffleSetting,
            hint_settings::HintGhostPrice::*,
            logic::Logic,
            logic_mode::LogicMode,
            pedestal_setting::PedestalSetting,
            settings::{Exclude, Exclusion, Options, Settings},
        },
        system::{System, UserConfig},
        Layout, LocationInfo, SeedInfo,
    },
    simplelog::SimpleLogger,
    structopt::StructOpt,
};

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

    let seed_info = SeedInfo {
        seed: 0,
        settings: &plando_settings(),
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
            lc_requirement: 1,
            yuganon_requirement: 1,
            ped_requirement: PedestalSetting::Standard,

            nice_mode: true,
            super_items: true,
            reverse_sage_events: false,
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

            weather_vanes_activated: true,
            dark_rooms_lampless: false,
            swordless_mode: false,

            hint_ghost_price: Price(1),
        },
        options: Options { chest_size_matches_contents: true, night_mode: true },
        exclusions: Exclusion { 0: Default::default() },
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

    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (1)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (2)"), ItemIceRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (3)"), ItemBombLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (4)"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (5)"), ItemBowLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (6)"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (7)"), ItemBell);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (8)"), ItemBoomerangLv2);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Ravio (9)"), ItemFireRodLv2);
    // layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Thanks"), Item::RingHekiga);

    /////////////////////////////
    // --- Dungeons Prizes --- //
    /////////////////////////////

    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "Eastern Palace Prize"), PendantCourage);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "House of Gales Prize"), SageRosso);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "Tower of Hera Prize"), SageGulley);

    layout.set(LocationInfo::new(regions::dungeons::hyrule::castle::SUBREGION, "Hyrule Castle Prize"), PendantCourage);

    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "Dark Palace Prize"), PendantPower);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "Swamp Palace Prize"), SageOren);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "Skull Woods Prize"), SageSeres);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "Thieves' Hideout Prize"), SageOsfala);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "Turtle Rock Prize"), SageImpa);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "Desert Palace Prize"), SageIrene);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "Ice Ruins Prize"), PendantWisdom);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Cucco Treasure Dungeon"), RupeeSilver);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Delivery"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Behind Blacksmith"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Cave"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Blacksmith Table"), ItemSwordLv1);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Hyrule Castle Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "Wildlife Clearing Stump"), RupeeGold);

    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Tree Behind Blacksmith"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Behind Link's House"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Blacksmith Tornado Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Cucco Treasure Dungeon Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Wildlife Clearing Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Hyrule Castle Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Hyrule Castle Tornado Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::field::main::SUBREGION, "[Mai] Tree West of Link's House"), RupeeGold);

    // Irene the Witch
    layout.set(LocationInfo::new(regions::hyrule::irene::witch::SUBREGION, "Irene"), RupeeGold);

    // Lost Woods
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Fortune-Teller"), RingRental);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Hyrule Hotfoot (First Race)"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Hyrule Hotfoot (Second Race)"), RupeeSilver);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Lost Woods Alcove"), ItemHookShot);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Lost Woods Big Rock Chest"), ItemIceRod);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Master Sword Pedestal"), ItemIceRod);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Rosso"), ItemIceRod);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Rosso Cave"), ItemInsectNet);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "Rosso Rocks"), RupeeGold);

    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Rosso Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Lost Woods Path Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Lost Woods Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Lost Woods Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Fortune-Teller Tent"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Moldorm Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Small Pond"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lost::woods::SUBREGION, "[Mai] Lost Woods Tree"), RupeeGold);

    // Death Mountain
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain Open Cave"), PowerGlove);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain Blocked Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain Fairy Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain West Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Donkey Cave Pegs"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain West Highest Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Spectacle Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Fire Cave Pillar"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Bouldering Guy"), HyruleShield);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Death Mountain Treasure Dungeon"), ItemHookShotLv2);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "Floating Island"), RupeeGold);

    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "[Mai] Death Mountain West Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "[Mai] Death Mountain West Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "[Mai] Death Mountain East Ledge Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "[Mai] Rosso's Ore Mine Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::death::mountain::SUBREGION, "[Mai] Outside Death Mountain Treasure Dungeon"), RupeeGold);

    // Kakariko
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Bee Guy (1)"), HintGlasses);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Bee Guy (2)"), ItemFireRod);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Dodge the Cuccos"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Item Shop (1)"), EscapeFruit);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Item Shop (2)"), StopFruit);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Item Shop (3)"), ItemShield);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Jail"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Well (Bottom)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Kakariko Well (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Milk Bar Owner"), LiverBlue);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Rupee Rush (Hyrule)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Shady Guy"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Street Merchant (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Street Merchant (Right)"), LiverYellow);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Stylish Woman"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "Woman"), RupeeR);

    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "[Mai] Cucco Ranch Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "[Mai] Hyrule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "[Mai] Kakariko Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "[Mai] Kakariko Sand"), ItemBowLight);
    layout.set(LocationInfo::new(regions::hyrule::kakariko::village::SUBREGION, "[Mai] Woman's Roof Rock"), RupeeGold);

    // Zora's River
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "Queen Oren"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "Waterfall Cave"), Kinsta);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "Zora's Domain Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "Zora's River Treasure Dungeon"), ItemBoomerangLv2);

    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "[Mai] Inside Witch's House"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "[Mai] Under Wooden Bridge"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "[Mai] Waterfall Ledge Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "[Mai] Zora's Domain South Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::zora::river::SUBREGION, "[Mai] Zora's Domain Water"), RupeeGold);

    // Eastern Ruins
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Bird Lover"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Treasure Dungeon"), ItemHammerLv2);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Armos Chest"), ItemTornadeRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Hookshot Chest"), ItemSandRod);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Merge Chest"), ItemBoomerang);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "Eastern Ruins Peg Circle"), RupeeGold);

    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Atop Eastern Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Eastern Ruins Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Eastern Ruins Green Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Eastern Ruins Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Eastern Ruins Yellow Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::eastern::ruins::SUBREGION, "[Mai] Southern Bridge River"), RupeeGold);

    // Desert of Mystery
    layout.set(LocationInfo::new(regions::hyrule::desert::mystery::SUBREGION, "[Mai] Buried in the Desert"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::desert::mystery::SUBREGION, "[Mai] Buried near Desert Palace"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::desert::mystery::SUBREGION, "[Mai] Southern Ruins Big Rock"), RupeeGold);

    // Southern Ruins
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Runaway Item Seller"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Southern Ruins Ledge"), ItemHammer);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Southern Ruins Pillar Cave"), ItemBowLight);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "Southern Ruins Treasure Dungeon"), ItemHammer);

    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "[Mai] Outside Flippers Dungeon"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "[Mai] Southern Ruins Bomb Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::southern::ruins::SUBREGION, "[Mai] Southern Ruins Pillars"), RupeeGold);

    // Lake Hylia
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 10 Maiamai"), ItemBowLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 20 Maiamai"), ItemBoomerangLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 30 Maiamai"), ItemHookShotLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 40 Maiamai"), ItemHammerLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 50 Maiamai"), ItemBombLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 60 Maiamai"), ItemFireRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 70 Maiamai"), ItemIceRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 80 Maiamai"), ItemTornadeRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, " 90 Maiamai"), ItemSandRodLv2);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "100 Maiamai"), SpecialMove);

    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Ice Rod Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lake Hylia Dark Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lake Hylia Ledge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lakeside Item Shop (1)"), EscapeFruit);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lakeside Item Shop (2)"), StopFruit);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Lakeside Item Shop (3)"), ItemShield);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "Southeastern Shore"), MessageBottle);

    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "[Mai] Hyrule Hotfoot Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "[Mai] Island Tornado Tile"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "[Mai] Lake Hylia SE Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "[Mai] Lake Hylia Shallow Ring"), RupeeGold);
    layout.set(LocationInfo::new(regions::hyrule::lake::hylia::SUBREGION, "[Mai] Outside Maiamai Cave"), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Lorule Field Treasure Dungeon"), GanbariPowerUp);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Vacant House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Rupee Rush (Lorule)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Great Rupee Fairy"), ItemIceRod);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Big Bomb Flower Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Octoball Derby"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Blacksmith (Lorule)"), ItemKandelaar);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Swamp Cave (Left)"), ItemBowLight);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Swamp Cave (Middle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Swamp Cave (Right)"), LiverBlue);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thief Girl Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Lorule Field Hookshot Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Fortune's Choice"), HeartPiece);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Thieves' Town Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Fortune-Teller Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Castle Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Castle Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Blacksmith Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Big Bomb Flower Field Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Octoball Derby Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Vacant House Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Behind Vacant House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Haunted Grove Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Southern Ruins Pillars"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Southern Ruins Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Lorule Southern Ruins Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "[Mai] Thieves' Town Tree"), RupeeGold);

    // Thieves' Town Shop
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thieves' Town Item Shop (1)"), Bee);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thieves' Town Item Shop (2)"), GoldenBeeForSale);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thieves' Town Item Shop (3)"), Fairy);
    layout.set(LocationInfo::new(regions::lorule::field::main::SUBREGION, "Thieves' Town Item Shop (4)"), ItemShield);

    // Lorule Lakeside Item Shop
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lakeside Item Shop (1)"), Bee);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lakeside Item Shop (2)"), GoldenBeeForSale);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lakeside Item Shop (3)"), Fairy);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lakeside Item Shop (4)"), ItemShield);

    // Chamber of Sages
    layout.set(LocationInfo::new(regions::lorule::chamber::sages::SUBREGION, "Osfala"), DashBoots);

    // Skull Woods (overworld)
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "Canyon House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "Destroyed House"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "Mysterious Man"), GoldenBeeForSale);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Canyon House Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Bush"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Dry Pond"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Entrance Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Shack Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::skull::overworld::SUBREGION, "[Mai] Skull Woods Skull"), RupeeGold);

    // Lorule Death Mountain
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Lorule Death Mountain East Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Behind Ice Gimos"), ItemFireRodLv2);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Lorule Death Mountain West Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Ice Gimos Fight"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Treacherous Tower Intermediate"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Treacherous Tower Advanced (1)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "Treacherous Tower Advanced (2)"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Lorule Death Mountain West Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Lorule Death Mountain West Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Lorule Death Mountain East Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Lorule Death Mountain East Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Outside Ice Ruins"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::death::mountain::SUBREGION, "[Mai] Lorule Death Mountain East Big Rock"), RupeeGold);

    // Dark Ruins
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Ruins Lakeview Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Dark Maze Ledge"), HeartPiece);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (1)"), RupeeG);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (2)"), RupeeB);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (3)"), RupeeR);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (4)"), RupeePurple);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (5)"), RupeeSilver);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Hinox (6)"), SpecialMove);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "Ku's Domain Fight"), ItemMizukaki);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Atop Dark Ruins Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Maze Center Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Maze Entrance Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Ruins East Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Ruins South Area Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Ruins Waterfall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Dark Ruins West Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Ku's Domain Grass"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Ku's Domain Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::dark::ruins::SUBREGION, "[Mai] Outside Hinox Cave"), RupeeGold);

    // Misery Mire
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Misery Mire Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "Misery Mire Treasure Dungeon"), ItemSandRodLv2);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "[Mai] Misery Mire Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "[Mai] Misery Mire Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::misery::mire::SUBREGION, "[Mai] Misery Mire Big Rock"), RupeeGold);

    // Lorule Lake
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "Lorule Lake Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "[Mai] Lorule Lake Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "[Mai] Lorule Lake SE Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "[Mai] Lorule Lake Skull"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "[Mai] Lorule Lake Water"), RupeeGold);
    layout.set(LocationInfo::new(regions::lorule::lake::lorule::SUBREGION, "[Mai] Lorule Lake West Wall"), RupeeGold);

    //////////////////////////
    // --- Mini Dungeon --- //
    //////////////////////////

    // Graveyard (Hyrule)
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "Dampe"), RingRental);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "Sanctuary Pegs"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[HS] Entrance"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[HS] Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[HS] Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[HS] Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "Graveyard Ledge Cave"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[Mai] Sanctuary Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::hyrule::SUBREGION, "[Mai] Hyrule Graveyard Wall"), RupeeGold);

    // Graveyard (Lorule)
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "Graveyard Peninsula"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "Philosopher's Cave"), OreBlue);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[LS] Entrance Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[LS] Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[LS] Lower Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[LS] Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[Mai] Lorule Graveyard Big Rock"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[Mai] Lorule Graveyard Peninsula Tree"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::graveyards::lorule::SUBREGION, "[Mai] Lorule Graveyard Wall"), RupeeGold);

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Merge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Left Door Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Popo Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Secret Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Switch Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (2F) Ball Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (2F) Defeat Popos"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (2F) Switch Room"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (2F) Big Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] Yuga (1)"), HintGlasses);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] Yuga (2)"), HeartContainer);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (3F) Escape Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::eastern::palace::SUBREGION, "[EP] (1F) Escape Chest"), RupeeGold);

    // House of Gales
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (1F) Torches"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (1F) Switch Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (1F) Fire Bubbles"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (1F) Blue Bari Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (1F) Blue Bari Room (Bottom Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (2F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (2F) Narrow Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (2F) Fire Ring"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (3F) Rat Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] (3F) Fire Bubbles"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::house::gales::SUBREGION, "[HG] Margomill"), HintGlasses);

    // Tower of Hera
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (1F) Outside"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (3F) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (5F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (6F) Left Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (6F) Right Mole"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (7F) Outside (Ledge)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (8F) Fairy Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] (11F) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::tower::hera::SUBREGION, "[TH] Moldorm"), HeartContainer);

    // Hyrule Castle
    layout.set(LocationInfo::new(regions::dungeons::hyrule::castle::SUBREGION, "[HC] Battlement"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::hyrule::castle::SUBREGION, "[HC] West Wing"), RupeeGold);

    // Dark Palace
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Right Pit"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Left Pit"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Hidden Room (Upper)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Hidden Room (Lower)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (B1) Fall From 1F"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (B1) Maze"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (B1) Helmasaur Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (B1) Helmasaur Room (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (2F) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (2F) South Hidden Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (2F) Alcove"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (1F) Fall From 2F"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] (B1) Big Chest (Switches)"), OreGreen);
    layout.set(LocationInfo::new(regions::dungeons::dark::palace::SUBREGION, "[PD] Gemesaur King"), RupeeGold);

    // Swamp Palace
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Center"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Raft Room (Left)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Raft Room (Right)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Gyorm"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Waterfall Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Raft Room (Pillar)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (B1) Big Chest (Secret)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (1F) Water Puzzle"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (1F) East Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (1F) West Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] (1F) Big Chest (Fire)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::swamp::palace::SUBREGION, "[SP] Arrghus"), KeyBoss);

    // Skull Woods
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) Gibdo Room (Lower)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) South Chest"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) Gibdo Room (Hole)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) Grate Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B2) Moving Platform Room"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) Big Chest (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "Skull Woods Outdoor Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] (B1) Big Chest (Upper)"), ItemKandelaarLv2);
    layout.set(LocationInfo::new(regions::dungeons::skull::woods::SUBREGION, "[SW] Knucklemaster"), OreRed);

    // Thieves' Hideout
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B1) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B1) Grate Chest"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B2) Grate Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B2) Switch Puzzle Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B2) Jail Cell"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B2) Eyegores"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B1) Behind Wall"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B1) Big Chest (Entrance)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B3) Underwater"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "[T'H] (B3) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::thieves::hideout::SUBREGION, "Stalblind"), OreYellow);

    // Ice Ruins
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (1F) Hidden Chest"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Left)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Grate Chest (Right)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Ice Pillar"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B5) Big Chest"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B1) East Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B1) Narrow Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B1) Upper Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B3) Big Chest (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Southwest Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Narrow Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B2) Long Merge Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] (B4) Southeast Chest (Fall)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::ice::ruins::SUBREGION, "[IR] Dharkstare"), RupeeGold);

    // Desert Palace
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Entrance"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Sand Room (South)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Sand Switch Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Sand Room (North)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Big Chest (Behind Wall)"), KeyBoss);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (1F) Behind Rocks"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Under Rock (Left)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Beamos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Under Rock (Right)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Under Rock (Ball Room)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Big Chest (Puzzle)"), PowerfulGlove);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (2F) Leever Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (3F) Behind Falling Sand"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "[DP] (3F) Armos Room"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::desert::palace::SUBREGION, "Zaganaga"), RupeeGold);

    // Turtle Rock
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Grate Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Portal Room (Northwest)"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Northeast Ledge"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Southeast Chest"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Defeat Flamolas"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "Turtle Rock Left Balcony"), ItemMizukaki);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Northeast Room"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Grate Chest (Small)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Center)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Platform"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Big Chest (Top)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (1F) Under Center"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] (B1) Under Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::turtle::rock::SUBREGION, "[TR] Grinexx"), KeyBoss);

    // Lorule Castle
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (1F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (1F) Center"), HintGlasses);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (2F) Near Torches"), Compass);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (2F) Hidden Path"), KeySmall);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (2F) Ledge"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (4F) Center"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (4F) Hidden Path"), ItemBowLight);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (3F) Bomb Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (3F) Bomb Trial (Behind Rock)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (3F) Ball Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (3F) Ball Trial (Puzzle)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (4F) Lamp Trial"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (4F) Hookshot Trial (Chest)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "[LC] (4F) Hookshot Trial (Eyes)"), RupeeGold);
    layout.set(LocationInfo::new(regions::dungeons::lorule::castle::SUBREGION, "Zelda"), ItemBow);

    info!("Successfully Built Layout");

    layout
}
