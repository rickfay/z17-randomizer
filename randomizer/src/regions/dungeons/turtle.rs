crate::region! {
    course: DungeonKame,
    name: "Turtle Rock",
    rock {
        locations: [
            "[TR] (1F) Center": Compass @Chest(1[115]),
            "[TR] (1F) Grate Chest": RupeePurple @Chest(1[58]),
            "[TR] (1F) Portal Room (Northwest)": KeySmall @Key(1[153]),
            "[TR] (1F) Northeast Ledge": KeySmall @Key(1[243]),
            "[TR] (1F) Southeast Chest": RupeePurple @Chest(1[173]),
            "[TR] (1F) Defeat Flamolas": RupeeSilver @Chest(1[220]),
            "[TR] (B1) Northeast Room": KeySmall @Key(2[53]),
            "[TR] (B1) Platform": RupeeSilver @Chest(2[183]),
            "[TR] (B1) Grate Chest (Small)": RupeePurple @Chest(2[5]),
            "[TR] (B1) Big Chest (Center)": HyruleShield @Chest(2[180]) :- {|s| s.can_hit_shielded_switch() || s.glitched()},  // Throw skull to hit switch
            "[TR] (B1) Big Chest (Top)": KeyBoss @Chest(2[29]) :- {|s| s.small_keys(COURSE) >= 1 && (s.can_hit_shielded_switch() || s.glitched())}, // Throw skull to hit switch
        ],
        paths: [
            boss :- {|s| (s.small_keys(COURSE) >= 3 && s.has_boss_key(COURSE)) || (s.glitched() && s.can_tornado_rod() && s.nice_bombs())},
            lorule::lake::balcony,
        ],
    },
    boss {
        locations: [
            "[TR] Grinexx": HeartContainer @Heart(3[6]),
        ],
        quest: Portrait::Impa,
    },
}
