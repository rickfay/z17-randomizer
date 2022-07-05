crate::region! {
    course: DungeonWater,
    name: "Swamp Palace",
    palace {
        paths: [
            dungeon :- {|p| p.can_hookshot() && p.can_swim()},
        ],
    },
    dungeon {
        locations: [
            "[SP] (B1) Center": Compass @Chest(2[319]),
            "[SP] (B1) Raft Room (Left)": RupeeR @Chest(2[620]),
            "[SP] (B1) Raft Room (Right)": LiverPurple @Chest(2[621]),
            "[SP] (B1) Raft Room (Pillar)": KeySmall @Key(2[116]),
            "[SP] (B1) Gyorm": RupeeGold @Chest(2[572]),
            "[SP] (B1) Waterfall Room": KeySmall @Key(2[219]),

            "[SP] (B1) Big Chest (Secret)": ClothesBlue @Chest(2[220]) :- {|s| s.small_keys(COURSE) >= 2 && s.can_damage() && (s.can_merge() || s.glitched())},
            "[SP] (1F) Water Puzzle": KeySmall @Chest(1[373]) :- {|s| s.can_merge() && ((s.small_keys(COURSE) >= 2 && s.can_damage()) || (s.glitched() && s.can_ice_rod()))},
            "[SP] (1F) East Room": KeySmall @Chest(1[299]) :- {|s| s.can_merge() && ((s.small_keys(COURSE) >= 2 && s.can_damage()) || (s.glitched() && s.can_ice_rod()))},
            "[SP] (1F) West Room": LiverPurple @Chest(1[170]) :- {|s| s.can_merge() && ((s.small_keys(COURSE) >= 2 && s.can_damage()) || (s.glitched() && s.can_ice_rod()))},

            "[SP] (1F) Big Chest (Fire)": KeyBoss @Chest(1[28]) :- {|s|
                (s.can_merge() && (s.small_keys(COURSE) >= 4 && s.can_damage())
                || (s.small_keys(COURSE) >= 2 && s.can_damage() && (s.can_tornado_rod() || s.can_ice_rod())))
                || (s.glitched() && ((s.can_merge() && s.can_ice_rod()) || s.has_boots()))
            },
        ],
        paths: [
            boss :- {|s| (s.small_keys(COURSE) >= 4 && s.has_boss_key(COURSE) && s.can_damage() && s.can_merge())
                || (s.glitched() && s.can_ice_rod() && ((s.has_boss_key(COURSE) && s.can_merge()) || s.can_tornado_rod()))},
        ],
    },
    boss {
        locations: [
            "[SP] Arrghus": HeartContainer @Heart(1[129]),
        ],
        quest: Portrait::Oren,
    },
}
