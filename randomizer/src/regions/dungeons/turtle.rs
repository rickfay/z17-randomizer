crate::region! {
    course: DungeonKame,
    name: "Turtle Rock",
    color: Purple,
    rock {
        locations: [
            "[TR] (1F) Center": Compass @Chest(1[115]),
            "[TR] (1F) Grate Chest": RupeePurple @Chest(1[58]),
            "[TR] (1F) Portal Room NW": KeySmall @Key(1[153]),
            "[TR] (1F) Northeast Ledge": KeySmall @Key(1[243]),
            "[TR] (1F) Southeast Chest": RupeePurple @Chest(1[173]),
            "[TR] (1F) Defeat Flamolas": RupeeSilver @Chest(1[220]),
            "[TR] Left Balcony": HeartPiece @Heart(FieldDark 35[54]),

            "[TR] (1F) Under Center": RupeeSilver @SilverRupee(1[114]),
            "[TR] (B1) Under Center": RupeeGold @GoldRupee(2[211]),

            "[TR] (B1) Northeast Room": KeySmall @Key(2[53]),
            "[TR] (B1) Platform": RupeeSilver @Chest(2[183]),
            "[TR] (B1) Grate Chest (Small)": RupeePurple @Chest(2[5]),
            "[TR] (B1) Big Chest (Center)": HyruleShield @BigChest(2[180]),
            "[TR] (B1) Big Chest (Top)": KeyBoss @BigChest(2[29]),

            "[TR] Grinexx": HeartContainer @Heart(3[6]),
            "[TR] Prize": None @None(),
        ],
    },
}
