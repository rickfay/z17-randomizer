crate::region! {
    course: DungeonHagure,
    name: "Thieves Hideout",
    hideout {
        locations: [
            "(B1) Jail Cell": RupeePurple @Chest(1[1323]) :- {|s| s.can_merge() || (s.glitched() && s.has_boots())}, // not including softlock strategy
            "(B1) Grate Chest": RupeePurple @Chest(1[576]),
        ],
        paths: [
            basement2 :- {|s| (s.can_merge() && s.can_hit_switch()) || (s.glitched() && s.can_ice_rod() && s.can_bomb())},
        ],
    },
    basement2 {
        locations: [
            "(B2) Grate Chest (Fall)": RupeePurple @Chest(1[1292]),
            "(B2) Switch Puzzle Room": LiverPurple @Chest(1[949]),
            "(B2) Jail Cell": Compass @Chest(1[283]) :- can_merge, // Not putting in glitched logic, miss-able if Master Ore hallway opened
            "(B2) Eyegores": KeySmall @Chest(1[543]) :- {|s| s.can_damage() && s.can_hit_shielded_switch()},
        ],
        paths: [
            escape :- {|s| (s.small_keys(COURSE) >= 1 && s.can_swim() && s.can_merge()) || (s.glitched() && s.can_tornado_rod() && (s.can_ice_rod() || s.can_bomb()))},
        ],
    },
    escape {
        locations: [
            "(B1) Behind Wall": RupeeSilver @Chest(1[1359]) :- {|s| s.small_keys(COURSE) >= 1 && s.can_swim() && s.can_merge()}, // Not putting in glitched logic
            "(B1) Big Chest (Entrance)": KeyBoss @Chest(1[580]) :- {|s| s.small_keys(COURSE) >= 1 && s.can_swim() && s.can_merge()}, // Not putting in glitched logic
            "(B3) Underwater": RupeeSilver @Chest(1[667]) :- can_merge,
            "(B3) Big Chest (Hidden)": OreBlue @Chest(1[657]),
        ],
        paths: [
            boss :- {|s| s.has_boss_key(COURSE) && s.small_keys(COURSE) >= 1 && s.can_merge()}, // Small Key check is to make sure we get Thief Girl
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
