crate::region! {
    course: FieldLight,
    name: "Sanctuary / River Area",
    color: Name,
    river {
        locations: [
            "Hyrule Waterfall Portal": None @Portal(13[30] WaterfallHyrule),
            "Zora's Domain Portal": None @Portal(15[41] ZorasDomain),

            "Sanctuary Weather Vane": None @WeatherVane(11[129] SanctuaryWV),
            "Witch's House Weather Vane": None @WeatherVane(14[61] WitchsHouseWV),

            "Queen Oren": ItemMizukaki @Event(CaveLight/FieldLight_0F_Zora[0x6B]),
            "River Mini-Dungeon": RupeeSilver @Chest(AttractionLight 5[24]),
            "Waterfall Cave": HeartPiece @Heart(CaveLight 13[103]),
            "Zora's Domain Ledge": RupeeR @Chest(15[35]),
            "[Mai] South of Zora's Domain": Maiamai @Maiamai(15[26]),
            "[Mai] Waterfall Ledge": Maiamai @Maiamai(13[28]),
            "[Mai] Witch's House": Maiamai @Maiamai(IndoorLight 2[12]),
            "[Mai] Wooden Bridge": Maiamai @Maiamai(19[39]),
            "[Mai] Zora's Domain": Maiamai @Maiamai(7[25]),
        ],
    },
}
