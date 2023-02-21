crate::region! {
    course: FieldLight,
    name: "Eastern Ruins",
    ruins {
        locations: [
            "Bird Lover": ItemBottle @Event(FieldLight_2D_UnderBridgeStranger[0x2A]),
            "Eastern Ruins Armos Chest": RupeeR @Chest(20[106]),
            "Eastern Ruins Cave": HeartPiece @Heart(CaveLight 29[10]),
            "Eastern Ruins Hookshot Chest": RupeeR @Chest(20[111]),
            "Eastern Ruins Merge Chest": RupeeSilver @Chest(20[107]),
            "Eastern Ruins Peg Circle": HeartPiece @Heart(30[41]),
            "Eastern Ruins Treasure Dungeon": RupeeSilver @Chest(AttractionLight 1[4]),
            "[Mai] Atop Eastern Rocks": Maiamai @Maiamai(22[42]),
            "[Mai] Eastern Ruins Big Rock": Maiamai @Maiamai(30[69]),
            "[Mai] Eastern Ruins Green Tree": Maiamai @Maiamai(29[44]),
            "[Mai] Eastern Ruins Wall": Maiamai @Maiamai(20[144]),
            "[Mai] Eastern Ruins Yellow Tree": Maiamai @Maiamai(20[145]),
            "[Mai] Southern Bridge River": Maiamai @Maiamai(28[45]),
        ],
    },
}
