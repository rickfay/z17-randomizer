crate::region! {
    course: DungeonDark,
    name: "Dark Palace",
    palace {
        locations: [
            "[PoD] (1F) Near Entrance": RupeeB @Chest(2[23]),
            "[PoD] (1F) Narrow Ledge": KeySmall @Key(2[25]),
        ],
        paths: [
            floor1 :- {|p| p.small_keys(COURSE) >= 1},
        ],
    },
    floor1 {
        locations: [
            "[PoD] (1F) Switch Puzzle": Compass @Chest(2[122]),
            "[PoD] (1F) Hidden Room (Upper)": RupeePurple @Chest(2[102]),
            "[PoD] (1F) Hidden Room (Lower)": LiverBlue @Chest(2[233]),
            "[PoD] (B1) Fall From 1F": KeySmall @Key(1[26]),
            "[PoD] (B1) Maze": KeySmall @Chest(1[102]),
            "[PoD] (B1) Helmasaur Room": KeySmall @Key(1[281]),
            "[PoD] (B1) Helmasaur Room (Fall)": LiverYellow @Chest(1[100]),
        ],
        paths: [
            boss_key :- {|s| s.small_keys(COURSE) >= 4},
            floor2 :- {|s| s.small_keys(COURSE) >= 4},
        ],
    },
    floor2 {
        locations: [
            "[PoD] (2F) Big Chest (Hidden)": OreYellow @Chest(3[41]),
            "[PoD] (2F) Alcove": LiverPurple @Chest(3[269]),
            "[PoD] (1F) Fall From 2F": RupeePurple @Chest(2[127]),
            "[PoD] (2F) South Hidden Room": RupeeGold @GoldRupee(3[166]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE) && p.can_light()},
        ],
    },
    boss_key {
        locations: [
            "[PoD] (B1) Big Chest (Switches)": KeyBoss @Chest(1[84]),
        ],
    },
    boss {
        locations: [
            "[PoD] Gemesaur King": HeartContainer @Heart(1[119]),
        ],
        quest: Portrait::Gulley,
    },
}
