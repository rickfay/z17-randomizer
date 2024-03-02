crate::region! {
    course: FieldLight,
    name: "Graveyard",
    color: Purple,
    lorule {
        locations: [
            "Philosopher's Cave Crack": None @Crack(CaveDark 5[8] Philosopher),
            "Lorule Graveyard Ledge Crack": None @Crack(FieldDark 12[20] GraveyardLedgeLorule),

            "Graveyard Weather Vane": None @WeatherVane(FieldDark 12[97] GraveyardWV),

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
