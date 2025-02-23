crate::region! {
    course: FieldLight,
    name: "Desert",
    color: Name,
    mystery {
        locations: [
            "Misery Mire Entrance Crack": None @Crack(37[29] MiseryMireEntrance),
            "Desert Right Pillar Crack": None @Crack(31[42] DesertPillarRight),
            "Desert Left Pillar Crack": None @Crack(31[41] DesertPillarLeft),
            "Desert Middle Crack": None @Crack(31[43] DesertMiddle),
            "Desert SW Crack": None @Crack(31[45] DesertSW),
            "Desert Palace Crack": None @Crack(31[16] DesertPalace),
            "Desert North Crack": None @Crack(31[40] DesertNorth),

            "Desert Palace Weather Vane": None @WeatherVane(31[72] DesertPalaceWV),

            "[Mai] Buried in the Desert": Maiamai @Maiamai(31[67]),
            "[Mai] Buried near Desert Palace": Maiamai @Maiamai(31[66]),
            "[Mai] Southern Ruins Big Rock": Maiamai @Maiamai(37[30]),
        ],
    },
}
