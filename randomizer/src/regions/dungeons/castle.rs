crate::region! {
    course: DungeonGanon,
    name: "Lorule Castle",
    lorule {
        locations: [
            "[LC] (1F) Ledge": RupeeR @Chest(1[714]),
            "[LC] (1F) Center": KeySmall @Chest(1[723]),
            "[LC] (2F) Near Torches": RupeeR @Chest(1[994]),
            "[LC] (2F) Hidden Path": RupeeSilver @Chest(1[995]),
            "[LC] (2F) Ledge": LiverYellow @Chest(1[717]),
        ],
        paths: [
            floor4 :- {|s| s.small_keys(COURSE) >= 3},
            bomb_trial :- can_bomb,
            ball_trial,
        ],
    },
    floor4 {
        locations: [
            "[LC] (4F) Center": Compass @Chest(1[1002]),
            "[LC] (4F) Hidden Path": ClothesRed @Chest(1[725]),
        ],
        paths: [
            lamp_trial :- {|p| p.can_light() || p.glitched()},
            hookshot_trial :- can_hookshot,
        ],
    },
    bomb_trial {
        locations: [
            "[LC] (3F) Bomb Trial (Chest)": RupeeR @Chest(1[1167]),
            "[LC] (3F) Bomb Trial (Behind Rock)": KeySmall @Chest(1[1115]),
        ],
    },
    ball_trial {
        locations: [
            "[LC] (3F) Ball Trial (Chest)": LiverBlue @Chest(1[495]),
            "[LC] (3F) Ball Trial (Puzzle)": KeySmall @Chest(1[882]),
        ],
    },
    lamp_trial {
        locations: [
            "[LC] (4F) Lamp Trial": KeySmall @Chest(1[1092]),
        ],
    },
    hookshot_trial {
        locations: [
            "[LC] (4F) Hookshot Trial (Chest)": RupeePurple @Chest(1[1581]),
            "[LC] (4F) Hookshot Trial (Eyes)": KeySmall @Chest(1[1308]),
        ],
    },
    boss {
        locations: [
            "[LC] Zelda": ItemBowLight @Event(DungeonBoss/Ganon[0x42]) :- {|s| s.sword() || s.has_net()},
        ],
    },
}
