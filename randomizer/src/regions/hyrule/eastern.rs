crate::region! {
    course: FieldLight,
    name: "Eastern Ruins",
    color: Name,
    ruins {
        locations: [
            "Eastern Ruins Pillar Crack": None @Crack(22[33] EasternRuinsPillar),
            "Eastern Ruins SE Crack": None @Crack(30[54] EasternRuinsSE),
            "Hyrule River Crack": None @Crack(29[46] RiverHyrule),

            "Eastern Palace Weather Vane": None @WeatherVane(20[57] EasternPalaceWV),

            "Bird Lover": ItemBottle @Event(FieldLight_2D_UnderBridgeStranger[0x2A]),
            "Eastern Ruins Armos Chest": RupeeR @Chest(20[106]),
            "Eastern Ruins Cave": HeartPiece @Heart(CaveLight 29[10]),
            "Eastern Ruins Hookshot Chest": RupeeR @Chest(20[111]),
            "Eastern Ruins Merge Chest": RupeeSilver @Chest(20[107]),
            "Eastern Ruins Peg Circle": HeartPiece @Heart(30[41]),
            "Merge Mini-Dungeon": RupeeSilver @Chest(AttractionLight 1[4]),
            "[Mai] Eastern Ruins Bonk Rocks": Maiamai @Maiamai(22[42]),
            "[Mai] Eastern Ruins Rock": Maiamai @Maiamai(30[69]),
            "[Mai] Eastern Ruins Green Tree": Maiamai @Maiamai(29[44]),
            "[Mai] Eastern Ruins River": Maiamai @Maiamai(28[45]),
            "[Mai] Eastern Ruins Wall": Maiamai @Maiamai(20[144]),
            "[Mai] Eastern Ruins Yellow Tree": Maiamai @Maiamai(20[145]),
        ],
    },
}
