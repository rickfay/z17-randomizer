crate::region! {
    course: FieldDark,
    name: "Misery Mire",
    color: Name,
    mire {
        locations: [
            "Misery Mire Exit Crack": None @Crack(37[32] MiseryMireExit),
            "Mire Right Pillar Crack": None @Crack(31[55] MirePillarRight),
            "Mire Left Pillar Crack": None @Crack(31[54] MirePillarLeft),
            "Mire Middle Crack": None @Crack(31[56] MireMiddle),
            "Mire SW Crack": None @Crack(31[58] MireSW),
            "Zaganaga Crack": None @Crack(31[13] Zaganaga),
            "Mire North Crack": None @Crack(31[53] MireNorth),

            "Misery Mire Weather Vane": None @WeatherVane(31[92] MiseryMireWV),

            "Misery Mire Ledge": HeartPiece @Heart(31[82]),
            "Sand Mini-Dungeon": RupeeGold @Chest(AttractionDark 3[7]),
            "[Mai] Misery Mire Rock": Maiamai @Maiamai(37[33]),
            "[Mai] Misery Mire Wall": Maiamai @Maiamai(31[88]),
            "[Mai] Misery Mire Water": Maiamai @Maiamai(31[87]),
        ],
    },
}
