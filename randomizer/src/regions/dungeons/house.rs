crate::region! {
    course: DungeonWind,
    name: "House of Gales",
    color: Blue,
    gales {
        locations: [
            "[HG] (1F) Torches": RupeeR @Chest(1[365]),
            "[HG] (1F) Switch Room": LiverPurple @Chest(1[331]),
            "[HG] (1F) Fire Bubbles": KeySmall @Chest(1[44]),
            "[HG] (1F) West Room": Compass @Chest(1[286]),
            "[HG] (1F) West Room Secret": RupeeSilver @Chest(1[69]),
            "[HG] (2F) Big Chest": KeyBoss @BigChest(2[72]),
            "[HG] (2F) Narrow Ledge": KeySmall @Key(2[180]),
            "[HG] (2F) Fire Ring": KeySmall @Key(2[97]),
            "[HG] (3F) Rat Room": KeySmall @Chest(3[405]),
            "[HG] (3F) Fire Bubbles": RupeePurple @Chest(3[548]),
            "[HG] Margomill": HeartContainer @Heart(3[458]),
            "[HG] Prize": PendantWisdom @None(),
        ],
    },
}
