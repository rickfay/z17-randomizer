use game::{world, Item::*};
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

    layout.set(world::hyrule::field::main::get("Ravio (1)").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("Ravio (2)").unwrap(), MessageBottle);
    layout.set(world::hyrule::field::main::get("Ravio (3)").unwrap(), ItemBottle);
    layout.set(world::hyrule::field::main::get("Ravio (4)").unwrap(), ItemSwordLv1);
    layout.set(world::hyrule::field::main::get("Ravio (5)").unwrap(), ItemMizukaki);
    layout.set(world::hyrule::field::main::get("Ravio (6)").unwrap(), RupeeG);
    layout.set(world::hyrule::field::main::get("Ravio (7)").unwrap(), ItemBell);
    layout.set(world::hyrule::field::main::get("Ravio (8)").unwrap(), ItemHookShotLv2);
    layout.set(world::hyrule::field::main::get("Ravio (9)").unwrap(), RupeeGold);
    // layout.set(LocationInfo::new(world::hyrule::field::main::AREA, "Thanks"), Item::RingHekiga);

    /////////////////////////////
    // --- Dungeons Prizes --- //
    /////////////////////////////

    layout.set(world::dungeons::eastern::palace::get("Eastern Palace Prize").unwrap(), PendantCourage);
    layout.set(world::dungeons::house::gales::get("House of Gales Prize").unwrap(), SageRosso);
    layout.set(world::dungeons::tower::hera::get("Tower of Hera Prize").unwrap(), SageGulley);

    layout.set(world::dungeons::hyrule::castle::get("Hyrule Castle Prize").unwrap(), PendantCourage);

    layout.set(world::dungeons::dark::palace::get("Dark Palace Prize").unwrap(), PendantPower);
    layout.set(world::dungeons::swamp::palace::get("Swamp Palace Prize").unwrap(), SageOren);
    layout.set(world::dungeons::skull::woods::get("Skull Woods Prize").unwrap(), SageSeres);
    layout.set(world::dungeons::thieves::hideout::get("Thieves' Hideout Prize").unwrap(), SageOsfala);
    layout.set(world::dungeons::turtle::rock::get("Turtle Rock Prize").unwrap(), SageImpa);
    layout.set(world::dungeons::desert::palace::get("Desert Palace Prize").unwrap(), SageIrene);
    layout.set(world::dungeons::ice::ruins::get("Ice Ruins Prize").unwrap(), PendantWisdom);

    ////////////////////
    // --- Hyrule --- //
    ////////////////////

    // Hyrule Field
    layout.set(world::hyrule::field::main::get("Cucco Treasure Dungeon").unwrap(), RupeeSilver);
    layout.set(world::hyrule::field::main::get("Delivery").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("Behind Blacksmith").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("Blacksmith Cave").unwrap(), ItemSwordLv1);
    layout.set(world::hyrule::field::main::get("Blacksmith").unwrap(), ItemSwordLv1);
    layout.set(world::hyrule::field::main::get("Blacksmith Table").unwrap(), ItemSwordLv1);
    layout.set(world::hyrule::field::main::get("Hyrule Castle Rocks").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("Wildlife Clearing Stump").unwrap(), RupeeGold);

    layout.set(world::hyrule::field::main::get("[Mai] Tree Behind Blacksmith").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Behind Link's House").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Blacksmith Tornado Tile").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Cucco Dungeon Big Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Wildlife Clearing Tree").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Hyrule Castle Tree").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Hyrule Castle Tornado Tile").unwrap(), RupeeGold);
    layout.set(world::hyrule::field::main::get("[Mai] Tree West of Link's House").unwrap(), RupeeGold);

    // Irene the Witch
    layout.set(world::hyrule::irene::witch::get("Irene").unwrap(), RupeeGold);

    // Lost Woods
    layout.set(world::hyrule::lost::woods::get("Fortune-Teller").unwrap(), RingRental);
    layout.set(world::hyrule::lost::woods::get("Hyrule Hotfoot (First Race)").unwrap(), HintGlasses);
    layout.set(world::hyrule::lost::woods::get("Hyrule Hotfoot (Second Race)").unwrap(), RupeeSilver);
    layout.set(world::hyrule::lost::woods::get("Lost Woods Alcove").unwrap(), ItemHookShot);
    layout.set(world::hyrule::lost::woods::get("Lost Woods Big Rock Chest").unwrap(), ItemIceRod);
    layout.set(world::hyrule::lost::woods::get("Master Sword Pedestal").unwrap(), ItemIceRod);
    layout.set(world::hyrule::lost::woods::get("Rosso").unwrap(), ItemIceRod);
    layout.set(world::hyrule::lost::woods::get("Rosso Cave").unwrap(), ItemInsectNet);
    layout.set(world::hyrule::lost::woods::get("Rosso Rocks").unwrap(), RupeeGold);

    layout.set(world::hyrule::lost::woods::get("[Mai] Rosso Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Lost Woods Path Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Lost Woods Bush").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Lost Woods Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Fortune-Teller Tent").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Moldorm Ledge").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Small Pond").unwrap(), RupeeGold);
    layout.set(world::hyrule::lost::woods::get("[Mai] Lost Woods Tree").unwrap(), RupeeGold);

    // Death Mountain
    layout.set(world::hyrule::death::mountain::get("Death Mountain Open Cave").unwrap(), PowerGlove);
    layout.set(world::hyrule::death::mountain::get("Death Mountain Blocked Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Death Mountain Fairy Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Death Mountain West Ledge").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Donkey Cave Pegs").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Death Mountain West Highest Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Spectacle Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Fire Cave Pillar").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("Bouldering Guy").unwrap(), ItemBottle);
    layout.set(world::hyrule::death::mountain::get("Death Mountain Treasure Dungeon").unwrap(), ItemHookShotLv2);
    layout.set(world::hyrule::death::mountain::get("Floating Island").unwrap(), RupeeGold);

    layout.set(world::hyrule::death::mountain::get("[Mai] Death Mountain Base Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("[Mai] Death Mountain West Ledge").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("[Mai] Death Mountain East Ledge").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("[Mai] Rosso's Ore Mine Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::death::mountain::get("[Mai] Outside Hookshot Dungeon").unwrap(), RupeeGold);

    // Kakariko
    layout.set(world::hyrule::kakariko::village::get("Bee Guy (1)").unwrap(), HintGlasses);
    layout.set(world::hyrule::kakariko::village::get("Bee Guy (2)").unwrap(), ItemFireRod);
    layout.set(world::hyrule::kakariko::village::get("Dodge the Cuccos").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Item Shop (1)").unwrap(), EscapeFruit);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Item Shop (2)").unwrap(), StopFruit);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Item Shop (3)").unwrap(), ItemShield);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Jail").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Well (Bottom)").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Kakariko Well (Top)").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Rupee Rush (Hyrule)").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Shady Guy").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Street Merchant (Left)").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Street Merchant (Right)").unwrap(), LiverYellow);
    layout.set(world::hyrule::kakariko::village::get("Stylish Woman").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("Woman").unwrap(), RupeeR);

    layout.set(world::hyrule::kakariko::village::get("[Mai] Cucco Ranch Tree").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("[Mai] Hyrule Rupee Rush Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("[Mai] Kakariko Bush").unwrap(), RupeeGold);
    layout.set(world::hyrule::kakariko::village::get("[Mai] Kakariko Sand").unwrap(), ItemBowLight);
    layout.set(world::hyrule::kakariko::village::get("[Mai] Woman's Roof Rock").unwrap(), RupeeGold);

    // Zora's River
    layout.set(world::hyrule::zora::river::get("Queen Oren").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("Waterfall Cave").unwrap(), Kinsta);
    layout.set(world::hyrule::zora::river::get("Zora's Domain Ledge").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("Zora's River Treasure Dungeon").unwrap(), ItemBoomerangLv2);

    layout.set(world::hyrule::zora::river::get("[Mai] Inside Witch's House").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("[Mai] Under Wooden Bridge").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("[Mai] Waterfall Ledge Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("[Mai] Zora's Domain South Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::zora::river::get("[Mai] Zora's Domain Water").unwrap(), RupeeGold);

    // Eastern Ruins
    layout.set(world::hyrule::eastern::ruins::get("Bird Lover").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Treasure Dungeon").unwrap(), ItemHammerLv2);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Armos Chest").unwrap(), ItemTornadeRod);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Hookshot Chest").unwrap(), ItemSandRod);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Merge Chest").unwrap(), ItemBoomerang);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("Eastern Ruins Peg Circle").unwrap(), RupeeGold);

    layout.set(world::hyrule::eastern::ruins::get("[Mai] Atop Eastern Rocks").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Big Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Green Tree").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("[Mai] Eastern Ruins Yellow Tree").unwrap(), RupeeGold);
    layout.set(world::hyrule::eastern::ruins::get("[Mai] Southern Bridge River").unwrap(), RupeeGold);

    // Desert of Mystery
    layout.set(world::hyrule::desert::mystery::get("[Mai] Buried in the Desert").unwrap(), RupeeGold);
    layout.set(world::hyrule::desert::mystery::get("[Mai] Buried near Desert Palace").unwrap(), RupeeGold);
    layout.set(world::hyrule::desert::mystery::get("[Mai] Southern Ruins Big Rock").unwrap(), RupeeGold);

    // Southern Ruins
    layout.set(world::hyrule::southern::ruins::get("Runaway Item Seller").unwrap(), RupeeGold);
    layout.set(world::hyrule::southern::ruins::get("Southern Ruins Ledge").unwrap(), ItemHammer);
    layout.set(world::hyrule::southern::ruins::get("Southern Ruins Pillar Cave").unwrap(), ItemBowLight);
    layout.set(world::hyrule::southern::ruins::get("Southern Ruins Treasure Dungeon").unwrap(), ItemHammer);

    layout.set(world::hyrule::southern::ruins::get("[Mai] Outside Flippers Dungeon").unwrap(), RupeeGold);
    layout.set(world::hyrule::southern::ruins::get("[Mai] Southern Ruins Bomb Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::southern::ruins::get("[Mai] Southern Ruins Pillars").unwrap(), RupeeGold);

    // Lake Hylia
    layout.set(world::hyrule::lake::hylia::get(" 10 Maiamai").unwrap(), ItemBowLv2);
    layout.set(world::hyrule::lake::hylia::get(" 20 Maiamai").unwrap(), ItemBoomerangLv2);
    layout.set(world::hyrule::lake::hylia::get(" 30 Maiamai").unwrap(), ItemHookShotLv2);
    layout.set(world::hyrule::lake::hylia::get(" 40 Maiamai").unwrap(), ItemHammerLv2);
    layout.set(world::hyrule::lake::hylia::get(" 50 Maiamai").unwrap(), ItemBombLv2);
    layout.set(world::hyrule::lake::hylia::get(" 60 Maiamai").unwrap(), ItemFireRodLv2);
    layout.set(world::hyrule::lake::hylia::get(" 70 Maiamai").unwrap(), ItemIceRodLv2);
    layout.set(world::hyrule::lake::hylia::get(" 80 Maiamai").unwrap(), ItemTornadeRodLv2);
    layout.set(world::hyrule::lake::hylia::get(" 90 Maiamai").unwrap(), ItemSandRodLv2);
    layout.set(world::hyrule::lake::hylia::get("100 Maiamai").unwrap(), SpecialMove);

    layout.set(world::hyrule::lake::hylia::get("Ice Rod Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("Lake Hylia Dark Cave").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("Lake Hylia Ledge Chest").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("Lakeside Item Shop (1)").unwrap(), EscapeFruit);
    layout.set(world::hyrule::lake::hylia::get("Lakeside Item Shop (2)").unwrap(), StopFruit);
    layout.set(world::hyrule::lake::hylia::get("Lakeside Item Shop (3)").unwrap(), ItemShield);
    layout.set(world::hyrule::lake::hylia::get("Southeastern Shore").unwrap(), HintGlasses);

    layout.set(world::hyrule::lake::hylia::get("[Mai] Hyrule Hotfoot Big Rock").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("[Mai] Island Tornado Tile").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("[Mai] Lake Hylia SE Wall").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("[Mai] Lake Hylia Shallow Ring").unwrap(), RupeeGold);
    layout.set(world::hyrule::lake::hylia::get("[Mai] Outside Maiamai Cave").unwrap(), RupeeGold);

    ////////////////////
    // --- Lorule --- //
    ////////////////////

    // Lorule Field
    layout.set(world::lorule::field::main::get("Lorule Field Treasure Dungeon").unwrap(), GanbariPowerUp);
    layout.set(world::lorule::field::main::get("Vacant House").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Rupee Rush (Lorule)").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Great Rupee Fairy").unwrap(), ItemIceRod);
    layout.set(world::lorule::field::main::get("Big Bomb Flower Cave").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Octoball Derby").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Blacksmith (Lorule)").unwrap(), ItemKandelaar);
    layout.set(world::lorule::field::main::get("Swamp Cave (Left)").unwrap(), ItemBowLight);
    layout.set(world::lorule::field::main::get("Swamp Cave (Middle)").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Swamp Cave (Right)").unwrap(), LiverBlue);
    layout.set(world::lorule::field::main::get("Thief Girl").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Lorule Field Hookshot Chest").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("Fortune's Choice").unwrap(), HeartPiece);
    layout.set(world::lorule::field::main::get("[Mai] Thieves' Town Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Near Lorule Fortune-Teller").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule Castle Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule Castle Tree").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule Blacksmith Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule Rupee Rush Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Big Bomb Flower Grass").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Octoball Derby Skull").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Vacant House Big Rock").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Behind Vacant House").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule Haunted Grove Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule S Ruins Pillars").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule S Ruins Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Lorule S Ruins Water").unwrap(), RupeeGold);
    layout.set(world::lorule::field::main::get("[Mai] Thieves' Town Tree").unwrap(), RupeeGold);

    // Thieves' Town Shop
    layout.set(world::lorule::field::main::get("Thieves' Town Item Shop (1)").unwrap(), Bee);
    layout.set(world::lorule::field::main::get("Thieves' Town Item Shop (2)").unwrap(), GoldenBeeForSale);
    layout.set(world::lorule::field::main::get("Thieves' Town Item Shop (3)").unwrap(), Fairy);
    layout.set(world::lorule::field::main::get("Thieves' Town Item Shop (4)").unwrap(), ItemShield);

    // Lorule Lakeside Item Shop
    layout.set(world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (1)").unwrap(), Bee);
    layout.set(world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (2)").unwrap(), GoldenBeeForSale);
    layout.set(world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (3)").unwrap(), Fairy);
    layout.set(world::lorule::lake::lorule::get("Lorule Lakeside Item Shop (4)").unwrap(), ItemShield);

    // Chamber of Sages
    layout.set(world::lorule::chamber::sages::get("Osfala").unwrap(), DashBoots);

    // Skull Woods (overworld)
    layout.set(world::lorule::skull::overworld::get("Canyon House").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("Destroyed House").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("Mysterious Man").unwrap(), GoldenBeeForSale);
    layout.set(world::lorule::skull::overworld::get("[Mai] Canyon House Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Big Rock").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Bush").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Dry Pond").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Entrance Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Grass").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Shack Tree").unwrap(), RupeeGold);
    layout.set(world::lorule::skull::overworld::get("[Mai] Skull Woods Skull").unwrap(), RupeeGold);

    // Lorule Death Mountain
    layout.set(world::lorule::death::mountain::get("Lorule Mountain E Ledge").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("Behind Ice Gimos").unwrap(), ItemFireRodLv2);
    layout.set(world::lorule::death::mountain::get("Lorule Mountain W Ledge").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("Ice Gimos Fight").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("Treacherous Tower Intermediate").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("Treacherous Tower Advanced (1)").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("Treacherous Tower Advanced (2)").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Lorule Mountain W Skull").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Lorule Mountain W Big Rock").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Lorule Mountain E Skull").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Lorule Mountain E Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Outside Ice Ruins").unwrap(), RupeeGold);
    layout.set(world::lorule::death::mountain::get("[Mai] Lorule Mountain E Big Rock").unwrap(), RupeeGold);

    // Dark Ruins
    layout.set(world::lorule::dark::ruins::get("Dark Ruins Lakeview Chest").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("Dark Maze Chest").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("Dark Maze Ledge").unwrap(), HeartPiece);
    layout.set(world::lorule::dark::ruins::get("Hinox (1)").unwrap(), RupeeG);
    layout.set(world::lorule::dark::ruins::get("Hinox (2)").unwrap(), RupeeB);
    layout.set(world::lorule::dark::ruins::get("Hinox (3)").unwrap(), RupeeR);
    layout.set(world::lorule::dark::ruins::get("Hinox (4)").unwrap(), RupeePurple);
    layout.set(world::lorule::dark::ruins::get("Hinox (5)").unwrap(), RupeeSilver);
    layout.set(world::lorule::dark::ruins::get("Hinox (6)").unwrap(), SpecialMove);
    layout.set(world::lorule::dark::ruins::get("Ku's Domain Fight").unwrap(), ItemMizukaki);
    layout.set(world::lorule::dark::ruins::get("[Mai] Atop Dark Ruins Rocks").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Maze Center Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Maze Entrance Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Ruins East Tree").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Ruins South Area Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Ruins Waterfall").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Dark Ruins West Tree").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Ku's Domain Grass").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Ku's Domain Water").unwrap(), RupeeGold);
    layout.set(world::lorule::dark::ruins::get("[Mai] Outside Hinox Cave").unwrap(), RupeeGold);

    // Misery Mire
    layout.set(world::lorule::misery::mire::get("Misery Mire Ledge").unwrap(), RupeeGold);
    layout.set(world::lorule::misery::mire::get("Misery Mire Treasure Dungeon").unwrap(), ItemSandRodLv2);
    layout.set(world::lorule::misery::mire::get("[Mai] Misery Mire Water").unwrap(), RupeeGold);
    layout.set(world::lorule::misery::mire::get("[Mai] Misery Mire Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::misery::mire::get("[Mai] Misery Mire Big Rock").unwrap(), RupeeGold);

    // Lorule Lake
    layout.set(world::lorule::lake::lorule::get("Lorule Lake Chest").unwrap(), RupeeGold);
    layout.set(world::lorule::lake::lorule::get("[Mai] Lorule Lake Big Rock").unwrap(), RupeeGold);
    layout.set(world::lorule::lake::lorule::get("[Mai] Lorule Lake SE Wall").unwrap(), RupeeGold);
    layout.set(world::lorule::lake::lorule::get("[Mai] Lorule Lake Skull").unwrap(), RupeeGold);
    layout.set(world::lorule::lake::lorule::get("[Mai] Lorule Lake Water").unwrap(), RupeeGold);
    layout.set(world::lorule::lake::lorule::get("[Mai] Lorule Lake West Wall").unwrap(), RupeeGold);

    //////////////////////////
    // --- Mini Dungeon --- //
    //////////////////////////

    // Graveyard (Hyrule)
    layout.set(world::dungeons::graveyards::hyrule::get("Dampe").unwrap(), RingRental);
    layout.set(world::dungeons::graveyards::hyrule::get("Sanctuary Pegs").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("[HS] Entrance").unwrap(), KeySmall);
    layout.set(world::dungeons::graveyards::hyrule::get("[HS] Lower Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("[HS] Upper Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("[HS] Ledge").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("Graveyard Ledge Cave").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("[Mai] Sanctuary Wall").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::hyrule::get("[Mai] Hyrule Graveyard Wall").unwrap(), RupeeGold);

    // Graveyard (Lorule)
    layout.set(world::dungeons::graveyards::lorule::get("Graveyard Peninsula").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("Philosopher's Cave").unwrap(), OreBlue);
    layout.set(world::dungeons::graveyards::lorule::get("[LS] Entrance Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("[LS] Ledge").unwrap(), KeySmall);
    layout.set(world::dungeons::graveyards::lorule::get("[LS] Lower Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("[LS] Upper Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("[Mai] Lorule Graveyard Big Rock").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("[Mai] Lorule Graveyard Tree").unwrap(), RupeeGold);
    layout.set(world::dungeons::graveyards::lorule::get("[Mai] Lorule Graveyard Wall").unwrap(), RupeeGold);

    //////////////////////
    // --- Dungeons --- //
    //////////////////////

    // Eastern Palace
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Merge Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Left Door Chest").unwrap(), KeySmall);
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Popo Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Secret Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Switch Room").unwrap(), KeySmall);
    layout.set(world::dungeons::eastern::palace::get("[EP] (2F) Ball Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (2F) Defeat Popos").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (2F) Switch Room").unwrap(), KeyBoss);
    layout.set(world::dungeons::eastern::palace::get("[EP] (2F) Big Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] Yuga (1)").unwrap(), HintGlasses);
    layout.set(world::dungeons::eastern::palace::get("[EP] Yuga (2)").unwrap(), HeartContainer);
    layout.set(world::dungeons::eastern::palace::get("[EP] (3F) Escape Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::eastern::palace::get("[EP] (1F) Escape Chest").unwrap(), RupeeGold);

    // House of Gales
    layout.set(world::dungeons::house::gales::get("[HG] (1F) Torches").unwrap(), Compass);
    layout.set(world::dungeons::house::gales::get("[HG] (1F) Switch Room").unwrap(), KeySmall);
    layout.set(world::dungeons::house::gales::get("[HG] (1F) Fire Bubbles").unwrap(), RupeeGold);
    layout.set(world::dungeons::house::gales::get("[HG] (1F) West Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::house::gales::get("[HG] (1F) West Room Secret").unwrap(), RupeeGold);
    layout.set(world::dungeons::house::gales::get("[HG] (2F) Big Chest").unwrap(), KeyBoss);
    layout.set(world::dungeons::house::gales::get("[HG] (2F) Narrow Ledge").unwrap(), RupeeGold);
    layout.set(world::dungeons::house::gales::get("[HG] (2F) Fire Ring").unwrap(), KeySmall);
    layout.set(world::dungeons::house::gales::get("[HG] (3F) Rat Room").unwrap(), KeySmall);
    layout.set(world::dungeons::house::gales::get("[HG] (3F) Fire Bubbles").unwrap(), KeySmall);
    layout.set(world::dungeons::house::gales::get("[HG] Margomill").unwrap(), HintGlasses);

    // Tower of Hera
    layout.set(world::dungeons::tower::hera::get("[TH] (1F) Outside").unwrap(), Compass);
    layout.set(world::dungeons::tower::hera::get("[TH] (1F) Center").unwrap(), RupeeGold);
    layout.set(world::dungeons::tower::hera::get("[TH] (3F) Platform").unwrap(), RupeeGold);
    layout.set(world::dungeons::tower::hera::get("[TH] (5F) Red/Blue Switches").unwrap(), RupeeGold);
    layout.set(world::dungeons::tower::hera::get("[TH] (6F) Left Mole").unwrap(), KeySmall);
    layout.set(world::dungeons::tower::hera::get("[TH] (6F) Right Mole").unwrap(), KeySmall);
    layout.set(world::dungeons::tower::hera::get("[TH] (7F) Outside (Ledge)").unwrap(), RupeeGold);
    layout.set(world::dungeons::tower::hera::get("[TH] (8F) Fairy Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::tower::hera::get("[TH] (11F) Big Chest").unwrap(), KeyBoss);
    layout.set(world::dungeons::tower::hera::get("[TH] Moldorm").unwrap(), HeartContainer);

    // Hyrule Castle
    layout.set(world::dungeons::hyrule::castle::get("Hyrule Castle Battlement").unwrap(), RupeeGold);
    layout.set(world::dungeons::hyrule::castle::get("Hyrule Castle West Wing").unwrap(), RupeeGold);

    // Dark Palace
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Right Pit").unwrap(), Compass);
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Left Pit").unwrap(), KeySmall);
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Switch Puzzle").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Hidden Room (Upper)").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Hidden Room (Lower)").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (B1) Fall From 1F").unwrap(), KeySmall);
    layout.set(world::dungeons::dark::palace::get("[PD] (B1) Maze").unwrap(), KeySmall);
    layout.set(world::dungeons::dark::palace::get("[PD] (B1) Helmasaur Room").unwrap(), KeySmall);
    layout.set(world::dungeons::dark::palace::get("[PD] (B1) Helmasaur Room (Fall)").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (2F) Big Chest (Hidden)").unwrap(), KeyBoss);
    layout.set(world::dungeons::dark::palace::get("[PD] (2F) South Hidden Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (2F) Alcove").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (1F) Fall From 2F").unwrap(), RupeeGold);
    layout.set(world::dungeons::dark::palace::get("[PD] (B1) Big Chest (Switches)").unwrap(), OreGreen);
    layout.set(world::dungeons::dark::palace::get("[PD] Gemesaur King").unwrap(), RupeeGold);

    // Swamp Palace
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Center").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Left)").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Right)").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Gyorm").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Waterfall Room").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Raft Room (Pillar)").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (B1) Big Chest (Secret)").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (1F) Water Puzzle").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (1F) East Room").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (1F) West Room").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] (1F) Big Chest (Fire)").unwrap(), KeySmall);
    layout.set(world::dungeons::swamp::palace::get("[SP] Arrghus").unwrap(), KeyBoss);

    // Skull Woods
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) Gibdo Room (Lower)").unwrap(), KeySmall);
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) South Chest").unwrap(), Compass);
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) Gibdo Room (Hole)").unwrap(), KeySmall);
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) Grate Room").unwrap(), KeySmall);
    layout.set(world::dungeons::skull::woods::get("[SW] (B2) Moving Platform Room").unwrap(), KeyBoss);
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) Big Chest (Eyes)").unwrap(), RupeeGold);
    layout.set(world::dungeons::skull::woods::get("Skull Woods Outdoor Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::skull::woods::get("[SW] (B1) Big Chest (Upper)").unwrap(), ItemKandelaarLv2);
    layout.set(world::dungeons::skull::woods::get("[SW] Knucklemaster").unwrap(), OreRed);

    // Thieves' Hideout
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B1) Jail Cell").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B1) Grate Chest").unwrap(), Compass);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B2) Grate Chest (Fall)").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B2) Switch Puzzle Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B2) Jail Cell").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B2) Eyegores").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B1) Behind Wall").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B1) Big Chest (Entrance)").unwrap(), RupeeGold);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B3) Underwater").unwrap(), KeySmall);
    layout.set(world::dungeons::thieves::hideout::get("[T'H] (B3) Big Chest (Hidden)").unwrap(), KeyBoss);
    layout.set(world::dungeons::thieves::hideout::get("Stalblind").unwrap(), OreYellow);

    // Ice Ruins
    layout.set(world::dungeons::ice::ruins::get("[IR] (1F) Hidden Chest").unwrap(), KeySmall);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B3) Grate Chest (Left)").unwrap(), KeySmall);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B3) Grate Chest (Right)").unwrap(), KeySmall);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B4) Ice Pillar").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B5) Big Chest").unwrap(), KeyBoss);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B1) East Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B1) Narrow Ledge").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B1) Upper Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B3) Big Chest (Puzzle)").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B4) Switches").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B4) Southwest Chest (Fall)").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B4) Narrow Platform").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B2) Long Merge Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] (B4) Southeast Chest (Fall)").unwrap(), RupeeGold);
    layout.set(world::dungeons::ice::ruins::get("[IR] Dharkstare").unwrap(), RupeeGold);

    // Desert Palace
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Entrance").unwrap(), Compass);
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Sand Room (South)").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Sand Switch Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Sand Room (North)").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Big Chest (Behind Wall)").unwrap(), KeyBoss);
    layout.set(world::dungeons::desert::palace::get("[DP] (1F) Behind Rocks").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Under Rock (Left)").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Beamos Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Under Rock (Right)").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Under Rock (Ball Room)").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Big Chest (Puzzle)").unwrap(), PowerfulGlove);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Red/Blue Switches").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (2F) Leever Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (3F) Behind Falling Sand").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("[DP] (3F) Armos Room").unwrap(), RupeeGold);
    layout.set(world::dungeons::desert::palace::get("Zaganaga").unwrap(), RupeeGold);

    // Turtle Rock
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Center").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Grate Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Portal Room NW").unwrap(), KeySmall);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Northeast Ledge").unwrap(), KeySmall);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Southeast Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Defeat Flamolas").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("Turtle Rock Left Balcony").unwrap(), ItemMizukaki);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Northeast Room").unwrap(), KeySmall);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Grate Chest (Small)").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Big Chest (Center)").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Platform").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Big Chest (Top)").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] (1F) Under Center").unwrap(), Compass);
    layout.set(world::dungeons::turtle::rock::get("[TR] (B1) Under Center").unwrap(), RupeeGold);
    layout.set(world::dungeons::turtle::rock::get("[TR] Grinexx").unwrap(), KeyBoss);

    // Lorule Castle
    layout.set(world::dungeons::lorule::castle::get("[LC] (1F) Ledge").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (1F) Center").unwrap(), HintGlasses);
    layout.set(world::dungeons::lorule::castle::get("[LC] (2F) Near Torches").unwrap(), Compass);
    layout.set(world::dungeons::lorule::castle::get("[LC] (2F) Hidden Path").unwrap(), KeySmall);
    layout.set(world::dungeons::lorule::castle::get("[LC] (2F) Ledge").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (4F) Center").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (4F) Hidden Path").unwrap(), ItemBowLight);
    layout.set(world::dungeons::lorule::castle::get("[LC] (3F) Bomb Trial Center Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (3F) Big Bomb Flower Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (3F) Merge Trial Free Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (3F) Spike Ball Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (4F) Lamp Trial Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (4F) Lava Switch Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("[LC] (4F) Eyeball Chest").unwrap(), RupeeGold);
    layout.set(world::dungeons::lorule::castle::get("Zelda").unwrap(), ItemBow);

    info!("Successfully Built Layout");

    layout
}
