crate::region! {
    course: DungeonGanon,
    name: "Lorule Castle",
    color: Purple,
    castle {
        locations: [
            "[LC] Portal": None @Portal(IndoorDark 5[14] LoruleCastle),

            "[LC] (1F) Ledge": RupeeR @Chest(1[714]),
            "[LC] (1F) Center": KeySmall @Chest[1[723],1[1586],], // Two actors representing one chest, idfk
            "[LC] (2F) Near Torches": RupeeR @Chest(1[994]),
            "[LC] (2F) Hidden Path": RupeeSilver @Chest(1[995]),
            "[LC] (2F) Ledge": LiverYellow @Chest(1[717]),
            "[LC] (4F) Center": Compass @Chest(1[1002]),
            "[LC] (4F) Hidden Path": ClothesRed @BigChest(1[725]),
            "[LC] Bomb Trial I": RupeeR @Chest(1[1167]),
            "[LC] Bomb Trial II": KeySmall @Chest(1[1115]),
            "[LC] Tile Trial I": LiverBlue @Chest(1[495]),
            "[LC] Tile Trial II": KeySmall @Chest(1[882]),
            "[LC] Lamp Trial": KeySmall @Chest(1[1092]),
            "[LC] Hook Trial I": RupeePurple @Chest(1[1581]),
            "[LC] Hook Trial II": KeySmall @Chest(1[1308]),
            "[LC] Zelda": ItemBowLight @Event(DungeonBoss/Ganon[0x42]),
        ],
    },
}
