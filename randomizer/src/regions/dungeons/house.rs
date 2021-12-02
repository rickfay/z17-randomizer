crate::region! {
    course: DungeonWind,
    name: "House of Gales",
    gales {
        paths: [
            floor1 :- can_tornado_rod,
        ],
    },
    floor1 {
        locations: [
            "(1F) Torches": RupeeR @Chest(1[365]) :- can_light,
            "(1F) Switch Room": LiverPurple @Chest(1[331]) :- {|s| s.sword() || s.can_use_projectile()},
            "(1F) Fire Bubbles": KeySmall @Chest(1[44]) :- can_merge,
        ],
        paths: [
            floor1west :- {|p| p.small_keys(COURSE) > 0 && p.can_merge()},
        ],
    },
    floor1west {
        locations: [
            "(1F) Blue Bari Room": Compass @Chest(1[286]),
            "(1F) Blue Bari Room (Bottom Left)": RupeeSilver @Chest(1[69]),
        ],
        paths: [
            floor2,
        ],
    },
    floor2 {
        locations: [
            "(2F) Big Chest": KeyBoss @Chest(2[72]),
            "(2F) Narrow Ledge": KeySmall @Key(2[180]),
        ],
        paths: [
            floor2outer :- {|p| p.small_keys(COURSE) > 1},
        ],
    },
    floor2outer {
        locations: [
            "(2F) Fire Ring": KeySmall @Key(2[97]),
        ],
        paths: [
            floor3 :- {|s| s.small_keys(COURSE) > 2 && s.can_damage() && s.can_light()},
        ],
    },
    floor3 {
        locations: [
            "(3F) Rat Room": KeySmall @Chest(3[405]),
            "(3F) Fire Bubbles": RupeePurple @Chest(3[548]),
        ],
        paths: [
            boss :- {|p| p.small_keys(COURSE) > 3 && p.has_boss_key(COURSE)},
        ],
    },
    boss {
        locations: [
            "Margomill": HeartContainer @Heart(3[458]),
        ],
        quest: Pendant::Wisdom,
    },
}
