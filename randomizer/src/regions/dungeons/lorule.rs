crate::region! {
    course: DungeonGanon,
    name: "Lorule Castle",
    color: Purple,
    castle {
        locations: [
            "[LC] (1F) Ledge": RupeeR @Chest(1[714]),
            "[LC] (1F) Center": KeySmall @Chest[1[723],1[1586],], // Two actors representing one chest, idfk
            "[LC] (2F) Near Torches": RupeeR @Chest(1[994]),
            "[LC] (2F) Hidden Path": RupeeSilver @Chest(1[995]),
            "[LC] (2F) Ledge": LiverYellow @Chest(1[717]),
            "[LC] (4F) Center": Compass @Chest(1[1002]),
            "[LC] (4F) Hidden Path": ClothesRed @BigChest(1[725]),
            "[LC] (3F) Bomb Trial Center Chest": RupeeR @Chest(1[1167]),
            "[LC] (3F) Big Bomb Flower Chest": KeySmall @Chest(1[1115]),
            "[LC] (3F) Merge Trial Free Chest": LiverBlue @Chest(1[495]),
            "[LC] (3F) Spike Ball Chest": KeySmall @Chest(1[882]),
            "[LC] (4F) Lamp Trial Chest": KeySmall @Chest(1[1092]),
            "[LC] (4F) Lava Switch Chest": RupeePurple @Chest(1[1581]),
            "[LC] (4F) Eyeball Chest": KeySmall @Chest(1[1308]),
            "Zelda": ItemBowLight @Event(DungeonBoss/Ganon[0x42]),
        ],
    },
}
