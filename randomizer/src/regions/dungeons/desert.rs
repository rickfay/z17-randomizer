crate::region! {
    course: DungeonSand,
    name: "Desert Palace",
    palace {
        paths: [
            floor1 :- can_sand_rod,
        ],
    },
    floor1 {
        locations: [
            "(1F) Entrance": RupeeB @Chest(1[78]),
            "(1F) Sand Room (South)": LiverBlue @Chest(1[565]),
            "(1F) Sand Switch Room": Compass @Chest(1[289]),
            "(1F) Sand Room (North)": KeySmall @Chest(1[371]),
            "(1F) Behind Rocks": KeySmall @Chest(1[349]) :- can_lift_big,
        ],
        paths: [
            center,
        ],
    },
    center {
        locations: [
            "(1F) Big Chest (Behind Wall)": PowerfulGlove @Chest(1[70]) :- {|p| p.small_keys(COURSE) > 0},
        ],
        paths: [
            floor2 :- {|p| (p.can_lift_big() && p.small_keys(COURSE) > 1) || (p.glitched() && (p.can_fire_rod() || p.can_bomb()))},
        ],
    },
    floor2 {
        locations: [
            "(2F) Under Rock (Left)": RupeeSilver @Chest(2[550]) :- can_lift_big,
            "(2F) Beamos Room": RupeeSilver @Chest(2[545]),
            "(2F) Under Rock (Right)": RupeeSilver @Chest(2[548]) :- can_lift_big,
            "(2F) Under Rock (Ball Room)": RupeeSilver @Chest(2[276]) :- can_lift_big,
            "(2F) Big Chest (Puzzle)": KeyBoss @Chest(2[35]),
            "(2F) Red/Blue Switches": KeySmall @Chest(2[462]),
        ],
        paths: [
            floor2west :- {|p| p.small_keys(COURSE) > 2 || (p.glitched() && (p.can_fire_rod() || p.can_bomb()))},
        ],
    },
    floor2west {
        locations: [
            "(2F) Leever Room": KeySmall @Chest(2[257]),
        ],
        paths: [
            floor3 :- {|p| p.small_keys(COURSE) > 3 || (p.glitched() && (p.can_fire_rod() || p.can_bomb()))},
        ],
    },
    floor3 {
        locations: [
            "(3F) Silver Rupee": RupeeSilver @Chest(3[195]),
            "(3F) Armos Room": KeySmall @Chest(3[110]),
        ],
        paths: [
            boss :- {|p| (p.small_keys(COURSE) > 4 && p.has_boss_key(COURSE)) || (p.glitched() && (p.can_fire_rod() || p.can_bomb()))},
        ],
    },
    boss {
        locations: [
            "Zaganaga": HeartContainer @Heart(FieldDark 31[83]),
        ],
        quest: Portrait::Irene,
    },
}