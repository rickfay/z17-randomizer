crate::region! {
    course: FieldLight,
    name: "River Area",
    color: Name,
    area {
        locations: [
            "Sanctuary Portal": None @Portal(IndoorLight 11[21] Sanctuary),
            "Hyrule Graveyard Ledge Portal": None @Portal(12[107] GraveyardLedgeHyrule),
            "Hyrule Waterfall Portal": None @Portal(13[30] WaterfallHyrule),
            "Zora's Domain Portal": None @Portal(15[41] ZorasDomain),

            "Sanctuary Weather Vane": None @WeatherVane(11[129] SanctuaryWV),
            "Witch's House Weather Vane": None @WeatherVane(14[61] WitchsHouseWV),

            "Dampe": ItemSwordLv1 @Event(FieldLight_13_Sister[0x1D]),
            "Graveyard Ledge Cave": HeartPiece @Heart(CaveLight 5[2]),
            "Sanctuary Pegs": RupeeSilver @Chest(11[89]),
            "Queen Oren": ItemMizukaki @Event(CaveLight/FieldLight_0F_Zora[0x6B]),
            "River Mini-Dungeon": RupeeSilver @Chest(AttractionLight 5[24]),
            "Waterfall Cave": HeartPiece @Heart(CaveLight 13[103]),
            "Zora's Domain Ledge": RupeeR @Chest(15[35]),

            "[HS] Entrance": ItemKandelaar @Chest(CaveLight 18[19]),
            "[HS] Ledge": HeartPiece @Heart(CaveLight 18[31]),
            "[HS] Lower Chest": RupeeR @Chest(CaveLight 18[45]),
            "[HS] Upper Chest": KeySmall @Chest(CaveLight 18[32]),

            "[Mai] Hyrule Graveyard Wall": Maiamai @Maiamai(12[120]),
            "[Mai] Sanctuary Wall": Maiamai @Maiamai(11[137]),

            "[Mai] South of Zora's Domain": Maiamai @Maiamai(15[26]),
            "[Mai] Waterfall Ledge": Maiamai @Maiamai(13[28]),
            "[Mai] Witch's House": Maiamai @Maiamai(IndoorLight 2[12]),
            "[Mai] Wooden Bridge": Maiamai @Maiamai(19[39]),
            "[Mai] Zora's Domain": Maiamai @Maiamai(7[25]),
        ],
    },
}
