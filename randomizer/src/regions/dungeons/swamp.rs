crate::region! {
    course: DungeonWater,
    name: "Swamp Palace",
    color: Beige,
    palace {
        locations: [
            "[SP] (B1) Center": Compass @Chest(2[319]),
            "[SP] (B1) Raft Room (Left)": RupeeR @Chest(2[620]),
            "[SP] (B1) Raft Room (Right)": LiverPurple @Chest(2[621]),
            "[SP] (B1) Raft Room (Pillar)": KeySmall @Key(2[116]),
            "[SP] (B1) Gyorm": RupeeGold @Chest(2[572]),
            "[SP] (B1) Waterfall Room": KeySmall @Key(2[219]),
            "[SP] (B1) Big Chest (Secret)": ClothesBlue @BigChest(2[220]),

            "[SP] (1F) Water Puzzle": KeySmall @Chest(1[299]),
            "[SP] (1F) East Room": KeySmall @Chest(1[170]),
            "[SP] (1F) West Room": LiverPurple @Chest(1[373]),
            "[SP] (1F) Big Chest (Fire)": KeyBoss @BigChest(1[28]),

            "[SP] Arrghus": HeartContainer @Heart(1[129]),
            "[SP] Prize": None @None(),
        ],
    },
}
