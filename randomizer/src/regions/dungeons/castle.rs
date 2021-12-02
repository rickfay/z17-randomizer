crate::region! {
    course: DungeonGanon,
    name: "Lorule Castle",
    lorule {
        locations: [
            "(1F) Ledge": RupeeR @Chest(1[714]) :- can_merge,
            "(1F) Center": KeySmall @Chest(1[723]),
            "(2F) Near Torches": RupeeR @Chest(1[994]),
            "(2F) Hidden Path": RupeeSilver @Chest(1[995]),
            "(2F) Ledge": LiverYellow @Chest(1[717]),
        ],
        paths: [
            floor4 :- {|s| {
                let keys = if s.settings().logic.unsafe_key_placement {
                    0
                } else {
                    2
                };
                s.small_keys(COURSE) > keys
            }},
            bomb_trial :- can_bomb,
            ball_trial,
        ],
    },
    floor4 {
        locations: [
            "(4F) Center": Compass @Chest(1[1002]),
            "(4F) Hidden Path": ClothesRed @Chest(1[725]),
        ],
        paths: [
            lamp_trial :- can_light,
            hookshot_trial :- can_hookshot,
        ],
    },
    bomb_trial {
        locations: [
            "(3F) Bomb Trial (Chest)": RupeeR @Chest(1[1167]),
            "(3F) Bomb Trial (Behind Rock)": KeySmall @Chest(1[1115]),
        ],
    },
    ball_trial {
        locations: [
            "(3F) Ball Trial (Chest)": LiverBlue @Chest(1[495]),
            "(3F) Ball Trial (Puzzle)": KeySmall @Chest(1[882]),
        ],
    },
    lamp_trial {
        locations: [
            "(4F) Lamp Trial": KeySmall @Chest(1[1092]),
        ],
    },
    hookshot_trial {
        locations: [
            "(4F) Hookshot Trial (Chest)": RupeePurple @Chest(1[1581]),
            "(4F) Hookshot Trial (Eyes)": KeySmall @Chest(1[1308]),
        ],
    },
    boss {
        locations: [
            "Zelda": ItemBowLight @Event(DungeonBoss/Ganon[0x42]),
        ],
    },
}
