crate::region! {
    course: FieldLight,
    name: "Hyrule Castle Area",
    color: Name,
    main {
        locations: [
            "Your House Crack": None @Crack(27[68] YourHouse),
            "Hyrule Right Paradox Crack": None @Crack(32[60] ParadoxRightHyrule),
            "Hyrule Left Paradox Crack": None @Crack(32[56] ParadoxLeftHyrule),

            "Your House Weather Vane": None @WeatherVane(27[48] YourHouseWV),

            "Blacksmith": ItemSwordLv3 @Event(IndoorLight/FieldLight_22_BlackSmith[0x16]),
            "Blacksmith Cave": HeartPiece @Heart(CaveLight 16[1]),
            "Blacksmith Ledge": HeartPiece @Heart(17[95]),
            "Blacksmith Table": PackageSword @Chest(IndoorLight 19[12]),
            "Cucco Mini-Dungeon": RupeeSilver @SilverRupee(AttractionLight 3[9]),
            "Haunted Grove Stump": Pouch @Event(FieldLight_2A_BlacksmithWife[0x15]),
            "Hyrule Castle Rocks": HeartPiece @Heart(18[209]),
            "[Mai] Behind Your House": Maiamai @Maiamai(27[112]),
            "[Mai] Blacksmith Tiles": Maiamai @Maiamai(21[122]),
            "[Mai] Blacksmith Tree": Maiamai @Maiamai(17[73]),
            "[Mai] Outside Cucco Mini-Dungeon": Maiamai @Maiamai(32[59]),
            "[Mai] Haunted Grove Tree": Maiamai @Maiamai(25[118]),
            "[Mai] Hyrule Castle Tiles": Maiamai @Maiamai(18[307]),
            "[Mai] Hyrule Castle Tree": Maiamai @Maiamai(18[305]),
            "[Mai] Your House Tree": Maiamai @Maiamai(26[49]),
        ],
    },
}
