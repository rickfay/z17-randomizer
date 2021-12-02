crate::region! {
    course: DungeonKame,
    name: "Turtle Rock",
    rock {
        locations: [
            "(1F) Center": Compass @Chest(1[115]),
            "(1F) Grate Chest": RupeePurple @Chest(1[58]),
            "(1F) Portal Room (Northwest)": KeySmall @Key(1[153]),
            "(1F) Northeast Ledge": KeySmall @Key(1[243]),
            "(1F) Southeast Chest": RupeePurple @Chest(1[173]),
            "(1F) Defeat Flamolas": RupeeSilver @Chest(1[220]),
            "(B1) Northeast Room": KeySmall @Key(2[53]),
            "(B1) Grate Chest (Small)": RupeePurple @Chest(2[5]),
            "(B1) Big Chest (Center)": HyruleShield @Chest(2[180]),
            "(B1) Platform": RupeeSilver @Chest(2[183]),
            "(B1) Big Chest (Top)": KeyBoss @Chest(2[29]) :- {|s| super::key_check(s)},
        ],
        paths: [
            boss :- {|s| super::key_check(s) && s.has_boss_key(COURSE)},
            lorule::lake::balcony,
        ],
    },
    boss {
        locations: [
            "Grinexx": HeartContainer @Heart(3[6]),
        ],
        quest: Portrait::Impa,
    },
}

fn key_check(state: &crate::state::State) -> bool {
    let keys = if state.settings().logic.unsafe_key_placement {
        0
    } else {
        2
    };
    state.small_keys(COURSE) > keys
}
