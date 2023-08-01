use game::{
    world::{self, LocationKey},
    Item::*,
};
use log::{error, info, LevelFilter};
use modd::{
    settings::{
        entrance_shuffle::EntranceShuffle,
        hyrule_castle::HyruleCastle,
        logic::{Logic, LogicMode},
        pedestal::Pedestal,
        Exclude, Exclusion, Options, Settings,
    },
    Layout, Mod,
};
use patcher::system::{System, UserConfig};
use simplelog::SimpleLogger;
use structopt::StructOpt;

use albw_randomizer::fail;

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

    let settings = plando_settings();
    settings.log_settings();

    let mod_ = Mod {
        name: "".into(),
        hash: None,
        settings,
        layout: build_layout(),
        hints: Default::default(),
    };

    match patcher::patch(&mod_, &user_config, args.no_patch, args.no_spoiler) {
        Ok(_) => {
            println!();
            info!("Successfully Generated ALBW Plandomizer Seed");
        }
        Err(err) => {
            println!();
            error!("Plandomizer execution failed:\n{}", err);
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
            ped_requirement: Pedestal::Standard,
            hyrule_castle_setting: HyruleCastle::EarlyLoruleCastle,

            nice_mode: true,
            super_items: true,
            reverse_sage_events: false,
            no_progression_enemies: true,
            entrance_rando: EntranceShuffle::NotShuffled,

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

    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (1)"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (2)"), MessageBottle);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (3)"), ItemBottle);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (4)"), ItemSwordLv1);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (5)"), ItemMizukaki);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (6)"), RupeeG);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (7)"), ItemBell);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (8)"), ItemHookShotLv2);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Ravio (9)"), RupeeGold);
    // layout.set(LocationInfo::new(world::hyrule::field::main::AREA, "Thanks"), Item::RingHekiga);

    /////////////////////////////
    // --- Dungeons Prizes --- //
    /////////////////////////////

    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "Eastern Palace Prize"), PendantCourage);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "House of Gales Prize"), SageRosso);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "Tower of Hera Prize"), SageGulley);

    layout.set(LocationKey::new(world::dungeons::hyrule::castle::AREA, "Hyrule Castle Prize"), PendantCourage);

    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "Dark Palace Prize"), PendantPower);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "Swamp Palace Prize"), SageOren);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "Skull Woods Prize"), SageSeres);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "Thieves' Hideout Prize"), SageOsfala);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "Turtle Rock Prize"), SageImpa);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "Desert Palace Prize"), SageIrene);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "Ice Ruins Prize"), PendantWisdom);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Cucco Treasure Dungeon"), RupeeSilver);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Delivery"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Behind Blacksmith"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Blacksmith Cave"), ItemSwordLv1);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Blacksmith"), ItemSwordLv1);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Blacksmith Table"), ItemSwordLv1);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Hyrule Castle Rocks"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "Wildlife Clearing Stump"), RupeeGold);

    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Tree Behind Blacksmith"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Behind Link's House"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Blacksmith Tornado Tile"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Cucco Dungeon Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Wildlife Clearing Tree"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Hyrule Castle Tree"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Hyrule Castle Tornado Tile"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::field::main::AREA, "[Mai] Tree West of Link's House"), RupeeGold);

    // Irene the Witch
    layout.set(LocationKey::new(world::hyrule::irene::witch::AREA, "Irene"), RupeeGold);

    // Lost Woods
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Fortune-Teller"), RingRental);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Hyrule Hotfoot (First Race)"), HintGlasses);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Hyrule Hotfoot (Second Race)"), RupeeSilver);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Lost Woods Alcove"), ItemHookShot);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Lost Woods Big Rock Chest"), ItemIceRod);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Master Sword Pedestal"), ItemIceRod);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Rosso"), ItemIceRod);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Rosso Cave"), ItemInsectNet);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "Rosso Rocks"), RupeeGold);

    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Rosso Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Lost Woods Path Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Lost Woods Bush"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Lost Woods Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Fortune-Teller Tent"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Moldorm Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Small Pond"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lost::woods::AREA, "[Mai] Lost Woods Tree"), RupeeGold);

    // Death Mountain
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain Open Cave"), PowerGlove);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain Blocked Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain Fairy Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain West Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Donkey Cave Pegs"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain West Highest Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Spectacle Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Fire Cave Pillar"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Bouldering Guy"), ItemBottle);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Death Mountain Treasure Dungeon"), ItemHookShotLv2);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "Floating Island"), RupeeGold);

    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "[Mai] Death Mountain Base Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "[Mai] Death Mountain West Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "[Mai] Death Mountain East Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "[Mai] Rosso's Ore Mine Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::death::mountain::AREA, "[Mai] Outside Hookshot Dungeon"), RupeeGold);

    // Kakariko
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Bee Guy (1)"), HintGlasses);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Bee Guy (2)"), ItemFireRod);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Dodge the Cuccos"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Item Shop (1)"), EscapeFruit);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Item Shop (2)"), StopFruit);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Item Shop (3)"), ItemShield);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Jail"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Well (Bottom)"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Kakariko Well (Top)"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Rupee Rush (Hyrule)"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Shady Guy"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Street Merchant (Left)"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Street Merchant (Right)"), LiverYellow);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Stylish Woman"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "Woman"), RupeeR);

    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "[Mai] Cucco Ranch Tree"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "[Mai] Hyrule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "[Mai] Kakariko Bush"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "[Mai] Kakariko Sand"), ItemBowLight);
    layout.set(LocationKey::new(world::hyrule::kakariko::village::AREA, "[Mai] Woman's Roof Rock"), RupeeGold);

    // Zora's River
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "Queen Oren"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "Waterfall Cave"), Kinsta);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "Zora's Domain Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "Zora's River Treasure Dungeon"), ItemBoomerangLv2);

    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "[Mai] Inside Witch's House"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "[Mai] Under Wooden Bridge"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "[Mai] Waterfall Ledge Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "[Mai] Zora's Domain South Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::zora::river::AREA, "[Mai] Zora's Domain Water"), RupeeGold);

    // Eastern Ruins
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Bird Lover"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Treasure Dungeon"), ItemHammerLv2);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Armos Chest"), ItemTornadeRod);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Hookshot Chest"), ItemSandRod);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Merge Chest"), ItemBoomerang);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "Eastern Ruins Peg Circle"), RupeeGold);

    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Atop Eastern Rocks"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Eastern Ruins Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Eastern Ruins Green Tree"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Eastern Ruins Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Eastern Ruins Yellow Tree"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::eastern::ruins::AREA, "[Mai] Southern Bridge River"), RupeeGold);

    // Desert of Mystery
    layout.set(LocationKey::new(world::hyrule::desert::mystery::AREA, "[Mai] Buried in the Desert"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::desert::mystery::AREA, "[Mai] Buried near Desert Palace"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::desert::mystery::AREA, "[Mai] Southern Ruins Big Rock"), RupeeGold);

    // Southern Ruins
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "Runaway Item Seller"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "Southern Ruins Ledge"), ItemHammer);
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "Southern Ruins Pillar Cave"), ItemBowLight);
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "Southern Ruins Treasure Dungeon"), ItemHammer);

    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "[Mai] Outside Flippers Dungeon"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "[Mai] Southern Ruins Bomb Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::southern::ruins::AREA, "[Mai] Southern Ruins Pillars"), RupeeGold);

    // Lake Hylia
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 10 Maiamai"), ItemBowLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 20 Maiamai"), ItemBoomerangLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 30 Maiamai"), ItemHookShotLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 40 Maiamai"), ItemHammerLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 50 Maiamai"), ItemBombLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 60 Maiamai"), ItemFireRodLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 70 Maiamai"), ItemIceRodLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 80 Maiamai"), ItemTornadeRodLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, " 90 Maiamai"), ItemSandRodLv2);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "100 Maiamai"), SpecialMove);

    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Ice Rod Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Lake Hylia Dark Cave"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Lake Hylia Ledge Chest"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Lakeside Item Shop (1)"), EscapeFruit);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Lakeside Item Shop (2)"), StopFruit);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Lakeside Item Shop (3)"), ItemShield);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "Southeastern Shore"), HintGlasses);

    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "[Mai] Hyrule Hotfoot Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "[Mai] Island Tornado Tile"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "[Mai] Lake Hylia SE Wall"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "[Mai] Lake Hylia Shallow Ring"), RupeeGold);
    layout.set(LocationKey::new(world::hyrule::lake::hylia::AREA, "[Mai] Outside Maiamai Cave"), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Lorule Field Treasure Dungeon"), GanbariPowerUp);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Vacant House"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Rupee Rush (Lorule)"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Great Rupee Fairy"), ItemIceRod);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Big Bomb Flower Cave"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Octoball Derby"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Blacksmith (Lorule)"), ItemKandelaar);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Swamp Cave (Left)"), ItemBowLight);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Swamp Cave (Middle)"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Swamp Cave (Right)"), LiverBlue);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Thief Girl"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Lorule Field Hookshot Chest"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Fortune's Choice"), HeartPiece);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Thieves' Town Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Near Lorule Fortune-Teller"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule Castle Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule Castle Tree"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule Blacksmith Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule Rupee Rush Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Big Bomb Flower Grass"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Octoball Derby Skull"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Vacant House Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Behind Vacant House"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule Haunted Grove Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule S Ruins Pillars"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule S Ruins Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Lorule S Ruins Water"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "[Mai] Thieves' Town Tree"), RupeeGold);

    // Thieves' Town Shop
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Thieves' Town Item Shop (1)"), Bee);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Thieves' Town Item Shop (2)"), GoldenBeeForSale);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Thieves' Town Item Shop (3)"), Fairy);
    layout.set(LocationKey::new(world::lorule::field::main::AREA, "Thieves' Town Item Shop (4)"), ItemShield);

    // Lorule Lakeside Item Shop
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "Lorule Lakeside Item Shop (1)"), Bee);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "Lorule Lakeside Item Shop (2)"), GoldenBeeForSale);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "Lorule Lakeside Item Shop (3)"), Fairy);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "Lorule Lakeside Item Shop (4)"), ItemShield);

    // Chamber of Sages
    layout.set(LocationKey::new(world::lorule::chamber::sages::AREA, "Osfala"), DashBoots);

    // Skull Woods (overworld)
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "Canyon House"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "Destroyed House"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "Mysterious Man"), GoldenBeeForSale);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Canyon House Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Bush"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Dry Pond"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Entrance Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Grass"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Shack Tree"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::skull::overworld::AREA, "[Mai] Skull Woods Skull"), RupeeGold);

    // Lorule Death Mountain
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Lorule Mountain E Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Behind Ice Gimos"), ItemFireRodLv2);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Lorule Mountain W Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Ice Gimos Fight"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Treacherous Tower Intermediate"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Treacherous Tower Advanced (1)"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "Treacherous Tower Advanced (2)"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Lorule Mountain W Skull"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Lorule Mountain W Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Lorule Mountain E Skull"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Lorule Mountain E Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Outside Ice Ruins"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::death::mountain::AREA, "[Mai] Lorule Mountain E Big Rock"), RupeeGold);

    // Dark Ruins
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Dark Ruins Lakeview Chest"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Dark Maze Chest"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Dark Maze Ledge"), HeartPiece);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (1)"), RupeeG);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (2)"), RupeeB);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (3)"), RupeeR);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (4)"), RupeePurple);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (5)"), RupeeSilver);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Hinox (6)"), SpecialMove);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "Ku's Domain Fight"), ItemMizukaki);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Atop Dark Ruins Rocks"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Maze Center Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Maze Entrance Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Ruins East Tree"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Ruins South Area Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Ruins Waterfall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Dark Ruins West Tree"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Ku's Domain Grass"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Ku's Domain Water"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::dark::ruins::AREA, "[Mai] Outside Hinox Cave"), RupeeGold);

    // Misery Mire
    layout.set(LocationKey::new(world::lorule::misery::mire::AREA, "Misery Mire Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::misery::mire::AREA, "Misery Mire Treasure Dungeon"), ItemSandRodLv2);
    layout.set(LocationKey::new(world::lorule::misery::mire::AREA, "[Mai] Misery Mire Water"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::misery::mire::AREA, "[Mai] Misery Mire Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::misery::mire::AREA, "[Mai] Misery Mire Big Rock"), RupeeGold);

    // Lorule Lake
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "Lorule Lake Chest"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "[Mai] Lorule Lake Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "[Mai] Lorule Lake SE Wall"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "[Mai] Lorule Lake Skull"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "[Mai] Lorule Lake Water"), RupeeGold);
    layout.set(LocationKey::new(world::lorule::lake::lorule::AREA, "[Mai] Lorule Lake West Wall"), RupeeGold);

    //////////////////////////
    // --- Mini Dungeon --- //
    //////////////////////////

    // Graveyard (Hyrule)
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "Dampe"), RingRental);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "Sanctuary Pegs"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[HS] Entrance"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[HS] Lower Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[HS] Upper Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[HS] Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "Graveyard Ledge Cave"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[Mai] Sanctuary Wall"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::hyrule::AREA, "[Mai] Hyrule Graveyard Wall"), RupeeGold);

    // Graveyard (Lorule)
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "Graveyard Peninsula"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "Philosopher's Cave"), OreBlue);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[LS] Entrance Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[LS] Ledge"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[LS] Lower Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[LS] Upper Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[Mai] Lorule Graveyard Big Rock"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[Mai] Lorule Graveyard Tree"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::graveyards::lorule::AREA, "[Mai] Lorule Graveyard Wall"), RupeeGold);

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Merge Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Left Door Chest"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Popo Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Secret Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Switch Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (2F) Ball Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (2F) Defeat Popos"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (2F) Switch Room"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (2F) Big Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] Yuga (1)"), HintGlasses);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] Yuga (2)"), HeartContainer);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (3F) Escape Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::eastern::palace::AREA, "[EP] (1F) Escape Chest"), RupeeGold);

    // House of Gales
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (1F) Torches"), Compass);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (1F) Switch Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (1F) Fire Bubbles"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (1F) West Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (1F) West Room Secret"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (2F) Big Chest"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (2F) Narrow Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (2F) Fire Ring"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (3F) Rat Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] (3F) Fire Bubbles"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::house::gales::AREA, "[HG] Margomill"), HintGlasses);

    // Tower of Hera
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (1F) Outside"), Compass);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (1F) Center"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (3F) Platform"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (5F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (6F) Left Mole"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (6F) Right Mole"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (7F) Outside (Ledge)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (8F) Fairy Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] (11F) Big Chest"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::tower::hera::AREA, "[TH] Moldorm"), HeartContainer);

    // Hyrule Castle
    layout.set(LocationKey::new(world::dungeons::hyrule::castle::AREA, "Hyrule Castle Battlement"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::hyrule::castle::AREA, "Hyrule Castle West Wing"), RupeeGold);

    // Dark Palace
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Right Pit"), Compass);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Left Pit"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Switch Puzzle"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Hidden Room (Upper)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Hidden Room (Lower)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (B1) Fall From 1F"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (B1) Maze"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (B1) Helmasaur Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (B1) Helmasaur Room (Fall)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (2F) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (2F) South Hidden Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (2F) Alcove"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (1F) Fall From 2F"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] (B1) Big Chest (Switches)"), OreGreen);
    layout.set(LocationKey::new(world::dungeons::dark::palace::AREA, "[PD] Gemesaur King"), RupeeGold);

    // Swamp Palace
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Center"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Raft Room (Left)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Raft Room (Right)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Gyorm"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Waterfall Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Raft Room (Pillar)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (B1) Big Chest (Secret)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (1F) Water Puzzle"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (1F) East Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (1F) West Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] (1F) Big Chest (Fire)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::swamp::palace::AREA, "[SP] Arrghus"), KeyBoss);

    // Skull Woods
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) Gibdo Room (Lower)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) South Chest"), Compass);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) Gibdo Room (Hole)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) Grate Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B2) Moving Platform Room"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) Big Chest (Eyes)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "Skull Woods Outdoor Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] (B1) Big Chest (Upper)"), ItemKandelaarLv2);
    layout.set(LocationKey::new(world::dungeons::skull::woods::AREA, "[SW] Knucklemaster"), OreRed);

    // Thieves' Hideout
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B1) Jail Cell"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B1) Grate Chest"), Compass);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B2) Grate Chest (Fall)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B2) Switch Puzzle Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B2) Jail Cell"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B2) Eyegores"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B1) Behind Wall"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B1) Big Chest (Entrance)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B3) Underwater"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "[T'H] (B3) Big Chest (Hidden)"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::thieves::hideout::AREA, "Stalblind"), OreYellow);

    // Ice Ruins
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (1F) Hidden Chest"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B3) Grate Chest (Left)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B3) Grate Chest (Right)"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B4) Ice Pillar"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B5) Big Chest"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B1) East Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B1) Narrow Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B1) Upper Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B3) Big Chest (Puzzle)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B4) Switches"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B4) Southwest Chest (Fall)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B4) Narrow Platform"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B2) Long Merge Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] (B4) Southeast Chest (Fall)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::ice::ruins::AREA, "[IR] Dharkstare"), RupeeGold);

    // Desert Palace
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Entrance"), Compass);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Sand Room (South)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Sand Switch Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Sand Room (North)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Big Chest (Behind Wall)"), KeyBoss);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (1F) Behind Rocks"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Under Rock (Left)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Beamos Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Under Rock (Right)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Under Rock (Ball Room)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Big Chest (Puzzle)"), PowerfulGlove);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Red/Blue Switches"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (2F) Leever Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (3F) Behind Falling Sand"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "[DP] (3F) Armos Room"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::desert::palace::AREA, "Zaganaga"), RupeeGold);

    // Turtle Rock
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Center"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Grate Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Portal Room NW"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Northeast Ledge"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Southeast Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Defeat Flamolas"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "Turtle Rock Left Balcony"), ItemMizukaki);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Northeast Room"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Grate Chest (Small)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Big Chest (Center)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Platform"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Big Chest (Top)"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (1F) Under Center"), Compass);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] (B1) Under Center"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::turtle::rock::AREA, "[TR] Grinexx"), KeyBoss);

    // Lorule Castle
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (1F) Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (1F) Center"), HintGlasses);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (2F) Near Torches"), Compass);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (2F) Hidden Path"), KeySmall);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (2F) Ledge"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (4F) Center"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (4F) Hidden Path"), ItemBowLight);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (3F) Bomb Trial Center Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (3F) Big Bomb Flower Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (3F) Merge Trial Free Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (3F) Spike Ball Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (4F) Lamp Trial Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (4F) Lava Switch Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "[LC] (4F) Eyeball Chest"), RupeeGold);
    layout.set(LocationKey::new(world::dungeons::lorule::castle::AREA, "Zelda"), ItemBow);

    info!("Successfully Built Layout");

    layout
}
