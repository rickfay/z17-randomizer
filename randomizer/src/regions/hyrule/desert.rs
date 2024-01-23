crate::region! {
    course: FieldLight,
    name: "Desert Area",
    color: Name,
    mystery {
        locations: [
            "Misery Mire Entrance Portal": None @Portal(37[29] MiseryMireEntrance),
            "Desert Right Pillar Portal": None @Portal(31[42] DesertPillarRight),
            "Desert Left Pillar Portal": None @Portal(31[41] DesertPillarLeft),
            "Desert Middle Portal": None @Portal(31[43] DesertMiddle),
            "Desert SW Portal": None @Portal(31[45] DesertSW),
            "Desert Palace Portal": None @Portal(31[16] DesertPalace),
            "Desert North Portal": None @Portal(31[40] DesertNorth),

            "Desert Palace Weather Vane": None @WeatherVane(31[72] DesertPalaceWV),

            "[Mai] Buried in the Desert": Maiamai @Maiamai(31[67]),
            "[Mai] Buried near Desert Palace": Maiamai @Maiamai(31[66]),
            "[Mai] Southern Ruins Big Rock": Maiamai @Maiamai(37[30]),
        ],
    },
}
