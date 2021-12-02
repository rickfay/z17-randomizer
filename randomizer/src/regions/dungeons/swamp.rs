crate::region! {
    course: DungeonWater,
    name: "Swamp Palace",
    palace {
        paths: [
            floor1 :- {|p| p.can_hookshot() && p.can_swim()},
        ],
    },
    floor1 {
        locations: [
            "(B1) Center": Compass @Chest(2[319]),
            "(B1) Raft Room (Left)": RupeeR @Chest(2[620]),
            "(B1) Raft Room (Right)": LiverPurple @Chest(2[621]),
            "(B1) Gyorm": RupeeGold @Chest(2[572]),
            "(B1) Waterfall Room": KeySmall @Key(2[219]),
        ],
        paths: [
            miniboss :- {|p| p.small_keys(COURSE) > 0},
        ],
    },
    miniboss {
        locations: [
            "(B1) Raft Room (Pillar)": KeySmall @Key(2[116]),
        ],
        paths: [
            deep :- {|p| p.small_keys(COURSE) > 1},
        ],
    },
    deep {
        locations: [
            "(B1) Big Chest (Secret)": ClothesBlue @Chest(2[220]),
            "(1F) Water Puzzle": KeySmall @Chest(1[373]),
            "(1F) East Room": KeySmall @Chest(1[299]),
            "(1F) West Room": LiverPurple @Chest(1[170]),
        ],
        paths: [
            end :- {|s| {
                let keys = if s.settings().logic.unsafe_key_placement {
                    2
                } else {
                    3
                };
                s.small_keys(COURSE) > keys
            }},
        ],
    },
    end {
        locations: [
            "(1F) Big Chest (Fire)": KeyBoss @Chest(1[28]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE)},
        ],
    },
    boss {
        locations: [
            "Arrghus": HeartContainer @Heart(1[129]),
        ],
        quest: Portrait::Oren,
    },
}
