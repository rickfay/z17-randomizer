crate::region! {
    course: DungeonHera,
    name: "Tower of Hera",
    hera {
        locations: [
            "(1F) Outside": RupeePurple @Chest(1[6]) :- can_merge,
        ],
        paths: [
            floor2 :- {|p| p.can_merge() || (p.glitched() && (p.sword() && p.nice_bombs()))},
        ],
    },
    floor2 {
        locations: [
            "(1F) Center": Compass @Chest(1[5]),
            "(3F) Platform": KeySmall @Key(1[244]),
        ],
        paths: [
            floor4 :- {|p| (p.can_merge() && p.small_keys(COURSE) > 0) || (p.glitched() && (p.nice_bombs() && p.can_tornado_rod()))},
        ],
    },
    floor4 {
        locations: [
            "(5F) Red/Blue Switches": RupeeB @Chest(1[251]),
            "(6F) Left Mole": KeySmall @Key(1[334]),
            "(6F) Right Mole": LiverPurple @Chest(1[694]),
        ],
        paths: [
            floor7 :- {|p| (p.can_merge() && p.small_keys(COURSE) > 1) || (p.glitched() && (p.nice_bombs() && p.can_tornado_rod()))},
        ],
    },
    floor7 {
        locations: [
            "(7F) Outside (Ledge)": RupeeSilver @Chest(1[793]),
            "(8F) Fairy Room": RupeePurple @Chest(1[838]),
            "(11F) Big Chest": KeyBoss @Chest(1[741]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE)},
        ],
    },
    boss {
        locations: [
            "Moldorm": HeartContainer @Heart(1[772]),
        ],
        quest: Pendant::Power,
    },
}
