crate::region! {
    course: DungeonHera,
    name: "Tower of Hera",
    hera {
        locations: [
            "[ToH] (1F) Outside": RupeePurple @Chest(1[6]) :- can_merge,
        ],
        paths: [
            floor2 :- {|p| p.can_merge() || (p.glitched() && (p.sword() && p.nice_bombs()))},
        ],
    },
    floor2 {
        locations: [
            "[ToH] (1F) Center": Compass @Chest(1[5]),
            "[ToH] (3F) Platform": KeySmall @Key(1[244]),
        ],
        paths: [
            floor4 :- {|p| (p.can_merge() && p.small_keys(COURSE) >= 1) || (p.glitched() && p.nice_bombs() && p.can_tornado_rod())},
        ],
    },
    floor4 {
        locations: [
            "[ToH] (5F) Red/Blue Switches": RupeeB @Chest(1[251]),
            "[ToH] (6F) Left Mole": KeySmall @Key(1[334]),
            "[ToH] (6F) Right Mole": LiverPurple @Chest(1[694]),
        ],
        paths: [
            floor7 :- {|p| (p.can_merge() && p.small_keys(COURSE) >= 2) || (p.glitched() && p.nice_bombs() && p.can_tornado_rod())},
        ],
    },
    floor7 {
        locations: [
            "[ToH] (7F) Outside (Ledge)": RupeeSilver @Chest(1[793]),
            "[ToH] (8F) Fairy Room": RupeePurple @Chest(1[838]),
            "[ToH] (11F) Big Chest": KeyBoss @Chest(1[741]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE)},
        ],
    },
    boss {
        locations: [
            "[ToH] Moldorm": HeartContainer @Heart(1[772]),
        ],
        quest: Pendant::Power,
    },
}
