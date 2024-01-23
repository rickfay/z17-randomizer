crate::region! {
    course: FieldDark,
    name: "Lorule Death Mountain",
    color: Name,
    mountain {
        locations: [
            "Lorule Death West Portal": None @Portal(3[45] DeathWestLorule),
            "Lorule Floating Island Portal": None @Portal(4[29] FloatingIslandLorule),
            "Lorule Rosso's Ore Mine Portal": None @Portal(4[70] RossosOreMineLorule),

            "Treacherous Tower Weather Vane": None @WeatherVane(3[52] TreacherousTowerWV),
            "Death Mountain (Lorule) Weather Vane": None @WeatherVane(4[68] DeathMountainLoruleWV),
            "Ice Ruins Weather Vane": None @WeatherVane(5[23] IceRuinsWV),

            "Behind Ice Gimos": RupeeSilver @Chest(4[94]),
            "Ice Gimos Fight": RupeeSilver @Chest(3[109]),
            "Ice Cave Ledge": RupeeGold @Chest(4[25]),
            "Lorule Mountain W Ledge": LiverBlue @Chest(3[108]),
            "Treacherous Tower Advanced (1)": ItemKandelaarLv2 @Event(FieldDark_05_GameTower[0x8D]),
            "Treacherous Tower Advanced (2)": ItemInsectNetLv2 @Event(FieldDark_05_GameTower[0x98]),
            "Treacherous Tower Intermediate": HeartPiece @Event(FieldDark_05_GameTower[0x7E]),
            "[Mai] Lorule Mountain E Big Rock": Maiamai @Maiamai(4[24]),
            "[Mai] Ice Cave Ledge": Maiamai @Maiamai(4[22]),
            "[Mai] Lorule Mountain E Wall": Maiamai @Maiamai(4[23]),
            "[Mai] Lorule Mountain W Big Rock": Maiamai @Maiamai(3[43]),
            "[Mai] Lorule Mountain W Skull": Maiamai @Maiamai(3[42]),
            "[Mai] Outside Ice Ruins": Maiamai @Maiamai(5[20]),
        ],
    },
}
