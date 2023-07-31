crate::region! {
    course: FieldLight,
    name: "Graveyard",
    hyrule {
        locations: [
            "Dampe": ItemSwordLv1 @Event(FieldLight_13_Sister[0x1D]),
            "Graveyard Ledge Cave": HeartPiece @Heart(CaveLight 5[2]),
            "Sanctuary Pegs": RupeeSilver @Chest(11[89]),
            "[HS] Entrance": ItemKandelaar @Chest(CaveLight 18[19]),
            "[HS] Ledge": HeartPiece @Heart(CaveLight 18[31]),
            "[HS] Lower Chest": RupeeR @Chest(CaveLight 18[45]),
            "[HS] Upper Chest": KeySmall @Chest(CaveLight 18[32]),
            "[Mai] Hyrule Graveyard Wall": Maiamai @Maiamai(12[120]),
            "[Mai] Sanctuary Wall": Maiamai @Maiamai(11[137]),
        ],
    },
    lorule {
        locations: [
            "Graveyard Peninsula": RupeeSilver @Chest(FieldDark 19[68]),
            "Philosopher's Cave": OreRed @BigChest(CaveDark 5[18]),
            "[LS] Entrance Chest": RupeeG @Chest(AttractionDark 2[219]),
            "[LS] Ledge": KeySmall @Key(AttractionDark 2[31]),
            "[LS] Lower Chest": RupeeB @Chest(AttractionDark 2[45]),
            "[LS] Upper Chest": RupeeR @Chest(AttractionDark 2[32]),
            "[Mai] Lorule Graveyard Big Rock": Maiamai @Maiamai(FieldDark 11[80]),
            "[Mai] Lorule Graveyard Tree": Maiamai @Maiamai(FieldDark 19[67]),
            "[Mai] Lorule Graveyard Wall": Maiamai @Maiamai(FieldDark 12[96]),
        ],
    },
}
