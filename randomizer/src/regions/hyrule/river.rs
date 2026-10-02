crate::region! {
    course: FieldLight,
    name: "River Area",
    color: Name,
    area {
        locations: [
            "Hyrule Sewers Entrance": None @Door (12[75] HyruleSewersEntrance),
            "Hyrule Sewers Exit": None @Door (CaveLight 18[106] HyruleSewersExit),
            "Graveyard Ledge Cave Entrance": None @Door (12[125] GraveyardLedgeCaveEntrance),
            "Graveyard Ledge Cave Exit": None @Door (CaveLight 5[4] GraveyardLedgeCaveExit),
            "Zora's Domain Entrance": None @Door (7[13] ZorasDomainEntrance),
            "Zora's Domain Exit": None @Door (CaveLight 7[112] ZorasDomainExit),
            "Waterfall Cave Entrance": None @Door (15[38] WaterfallCaveEntrance),
            "Waterfall Cave Exit": None @Door (CaveLight 13[50] WaterfallCaveExit),
            "Witch's House Entrance": None @Door (14[48] WitchHouseEntrance),
            "Witch's House Exit": None @Door (IndoorLight 2[8] WitchHouseExit),
            "River Mini Dungeon Entrance": None @Door (13[37] RiverMiniDungeonEntrance),
            "River Mini Dungeon Exit": None @Door (AttractionLight 5[3] RiverMiniDungeonExit),
            "Witch Cave Back Entrance": None @Door (14[75] WitchCaveBackEntrance),
            "Witch Cave Back Exit": None @Door (CaveLight 30[4] WitchCaveBackExit),

            "Sanctuary Crack": None @Crack(IndoorLight 11[21] Sanctuary),
            "Hyrule Graveyard Ledge Crack": None @Crack(12[107] GraveyardLedgeHyrule),
            "Hyrule Waterfall Crack": None @Crack(13[30] WaterfallHyrule),
            "Zora's Domain Crack": None @Crack(15[41] ZorasDomain),

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
