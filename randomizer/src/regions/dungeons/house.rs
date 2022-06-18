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
            "[HoG] (1F) Torches": RupeeR @Chest(1[365]) :- can_light,
            "[HoG] (1F) Switch Room": LiverPurple @Chest(1[331]) :- {|s| s.sword() || s.can_use_projectile()},
            "[HoG] (1F) Fire Bubbles": KeySmall @Chest(1[44]) :- can_merge,
        ],
        paths: [
            floor1west :- {|p| p.small_keys(COURSE) > 0 && (p.can_merge() || p.glitched())}, // glitched version can TRod onto blocks
        ],
    },
    floor1west {
        locations: [
            "[HoG] (1F) Blue Bari Room": Compass @Chest(1[286]),
            "[HoG] (1F) Blue Bari Room (Bottom Left)": RupeeSilver @Chest(1[69]) :- can_merge,
        ],
        paths: [
            floor2 :- {|p| p.can_use_projectile() || p.can_ice_rod() || (p.can_merge() && p.sword())}, // need to be able to hit fan crystal switch
        ],
    },
    floor2 {
        locations: [
            "[HoG] (2F) Big Chest": KeyBoss @Chest(2[72]),
            "[HoG] (2F) Narrow Ledge": KeySmall @Key(2[180]),
        ],
        paths: [
            floor2outer,
        ],
    },
    floor2outer {
        locations: [
            "[HoG] (2F) Fire Ring": KeySmall @Key(2[97]) :- {|p| p.can_merge() && (p.small_keys(COURSE) > 1 || (p.glitched() && p.has_boots()))}, // Not requiring Armos Boost
        ],
        paths: [
            floor3 :- {|s| s.can_merge() && ((s.small_keys(COURSE) > 2 && s.can_damage()) || s.glitched())}, // TRod skip the 2F
        ],
    },
    floor3 {
        locations: [
            "[HoG] (3F) Rat Room": KeySmall @Chest(3[405]) :- {|p| p.small_keys(COURSE) > 3 || p.can_light()},
            "[HoG] (3F) Fire Bubbles": RupeePurple @Chest(3[548]) :- can_light,
        ],
        paths: [
            boss :- {|p| p.small_keys(COURSE) > 3 && p.has_boss_key(COURSE)},
        ],
    },
    boss {
        locations: [
            "[HoG] Margomill": HeartContainer @Heart(3[458]),
        ],
        quest: Pendant::Wisdom,
    },
}
