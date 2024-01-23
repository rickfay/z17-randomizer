crate::region! {
    course: FieldDark,
    name: "Dark Ruins",
    color: Name,
    ruins {
        locations: [
            "Lorule Waterfall Portal": None @Portal(13[60] WaterfallLorule),
            "Dark Ruins Pillar Portal": None @Portal(22[26] DarkRuinsPillar),
            "Dark Ruins SE Portal": None @Portal(30[27] DarkRuinsSE),
            "Ku's Domain Portal": None @Portal(15[110] KusDomain),

            "Dark Palace Weather Vane": None @WeatherVane(20[210] DarkPalaceWV),

            "Dark Ruins Lakeview Chest": RupeeSilver @Chest(35[228]),
            "Dark Maze Chest": RupeeR @Chest(20[79]),
            "Dark Maze Ledge": HeartPiece @Heart(20[172]),
            "Hinox (1)": RupeeB @Event(CaveDark/FieldDark_17_NpcHinox [0x02]),
            "Hinox (2)": RupeeR @Event(CaveDark/FieldDark_17_NpcHinox [0x06]),
            "Hinox (3)": RupeePurple @Event(CaveDark/FieldDark_17_NpcHinox [0x4C]),
            "Hinox (4)": RupeeSilver @Event(CaveDark/FieldDark_17_NpcHinox [0x0A]),
            "Hinox (5)": RupeeSilver @Event(CaveDark/FieldDark_17_NpcHinox [0x11]),
            "Hinox (6)": RupeeGold @Event(CaveDark/FieldDark_17_NpcHinox [0x16]),
            "Ku's Domain Fight": RupeeSilver @Chest(7[55]),
            "[Mai] Ku's Domain Grass": Maiamai @Maiamai(15[10]),
            "[Mai] Ku's Domain Water": Maiamai @Maiamai(7[12]),
            "[Mai] Dark Ruins Waterfall": Maiamai @Maiamai(13[71]),
            "[Mai] Dark Maze Entrance Wall": Maiamai @Maiamai(20[206]),
            "[Mai] Dark Maze Center Wall": Maiamai @Maiamai(20[207]),
            "[Mai] Dark Ruins Bonk Rocks": Maiamai @Maiamai(22[25]),
            "[Mai] Dark Ruins West Tree": Maiamai @Maiamai(28[39]),
            "[Mai] Dark Ruins East Tree": Maiamai @Maiamai(29[57]),
            "[Mai] Dark Ruins South Wall": Maiamai @Maiamai(30[35]),
            "[Mai] Outside Hinox Cave": Maiamai @Maiamai(14[56]),
        ],
    },
}
