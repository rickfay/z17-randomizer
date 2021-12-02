crate::region! {
    course: DungeonDark,
    name: "Dark Palace",
    palace {
        locations: [
            "(1F) Near Entrance": RupeeB @Chest(2[23]),
            "(1F) Narrow Ledge": KeySmall @Key(2[25]),
        ],
        paths: [
            floor1 :- {|p| p.small_keys(COURSE) > 0},
        ],
    },
    floor1 {
        locations: [
            "(1F) Switch Puzzle": Compass @Chest(2[122]),
            "(1F) Hidden Room (Upper)": RupeePurple @Chest(2[102]),
            "(1F) Hidden Room (Lower)": LiverBlue @Chest(2[233]),
            "(B1) Fall From 1F": KeySmall @Key(1[26]),
            "(B1) Maze": KeySmall @Chest(1[102]),
            "(B1) Helmasaur Room": KeySmall @Key(1[281]),
            "(B1) Helmasaur Room (Fall)": LiverYellow @Chest(1[100]),
        ],
        paths: [
            boss_key :- {|s| super::key_check(s)},
            floor2 :- {|s| super::key_check(s)},
        ],
    },
    floor2 {
        locations: [
            "(2F) Big Chest (Hidden)": OreYellow @Chest(3[41]),
            "(2F) Alcove": LiverPurple @Chest(3[269]),
            "(1F) Fall From 2F": RupeePurple @Chest(2[127]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE) && p.can_light()},
        ],
    },
    boss_key {
        locations: [
            "(B1) Big Chest (Switches)": KeyBoss @Chest(1[84]),
        ],
    },
    boss {
        locations: [
            "Gemesaur King": HeartContainer @Heart(1[119]),
        ],
        quest: Portrait::Gulley,
    },
}

fn key_check(state: &crate::state::State) -> bool {
    let keys = if state.settings().logic.unsafe_key_placement {
        1
    } else {
        3
    };
    state.small_keys(COURSE) > keys
}
