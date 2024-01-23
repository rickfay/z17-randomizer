crate::region! {
    course: FieldDark,
    name: "Misery Mire",
    color: Name,
    mire {
        locations: [
            "Misery Mire Exit Portal": None @Portal(37[32] MiseryMireExit),
            "Mire Right Pillar Portal": None @Portal(31[55] MirePillarRight),
            "Mire Left Pillar Portal": None @Portal(31[54] MirePillarLeft),
            "Mire Middle Portal": None @Portal(31[56] MireMiddle),
            "Mire SW Portal": None @Portal(31[58] MireSW),
            "Zaganaga Portal": None @Portal(31[13] Zaganaga),
            "Mire North Portal": None @Portal(31[53] MireNorth),

            "Misery Mire Weather Vane": None @WeatherVane(31[92] MiseryMireWV),

            "Misery Mire Ledge": HeartPiece @Heart(31[82]),
            "Sand Mini-Dungeon": RupeeGold @Chest(AttractionDark 3[7]),
            "[Mai] Misery Mire Rock": Maiamai @Maiamai(37[33]),
            "[Mai] Misery Mire Wall": Maiamai @Maiamai(31[88]),
            "[Mai] Misery Mire Water": Maiamai @Maiamai(31[87]),
        ],
    },
}
