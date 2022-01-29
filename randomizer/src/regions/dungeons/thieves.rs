crate::region! {
    course: DungeonHagure,
    name: "Thieves Hideout",
    hideout {
        locations: [
            "(B1) Jail Cell": RupeePurple @Chest(1[1323]),
            "(B1) Grate Chest": RupeePurple @Chest(1[576]),
        ],
        paths: [
            basement2 :- can_hit_switch,
        ],
    },
    basement2 {
        locations: [
            "(B2) Grate Chest (Fall)": RupeePurple @Chest(1[1292]),
            "(B2) Switch Puzzle Room": LiverPurple @Chest(1[949]),
            "(B2) Jail Cell": Compass @Chest(1[283]),
            "(B2) Eyegores": KeySmall @Chest(1[543]) :- can_damage,
        ],
        paths: [
            escape :- {|p| (p.small_keys(COURSE) >= 1 && p.can_swim()) || (p.glitched() && p.can_tornado_rod() && (p.can_ice_rod() || p.can_bomb()))},
        ],
    },
    escape {
        locations: [
            "(B1) Behind Wall": RupeeSilver @Chest(1[1359]) :- {|p| (p.small_keys(COURSE) >= 1 && p.can_swim()) || (p.can_bomb() || p.can_fire_rod())}, // Need to be able to escape if they didn't bring Scoot Fruit
            "(B1) Big Chest (Entrance)": KeyBoss @Chest(1[580]) :- {|p| (p.small_keys(COURSE) >= 1 && p.can_swim()) || p.has_boots()},
            "(B3) Underwater": RupeeSilver @Chest(1[667]),
            "(B3) Big Chest (Hidden)": OreBlue @Chest(1[657]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE) && p.small_keys(COURSE) >= 1}, // Small Key check is to make sure we get Thief Girl
        ],
    },
    boss {
        locations: [
            "Stalblind": HeartContainer @Heart(IndoorDark 15[12]) :- can_damage,
        ],
        paths: [
            lorule::field::thief_girl,
        ],
        quest: Portrait::Osfala,
    },
}
