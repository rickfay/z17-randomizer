crate::region! {
    course: FieldDark,
    name: "Lorule Death Mountain",
    color: Name,
    mountain {
        locations: [
            "Behind Ice Gimos": RupeeSilver @Chest(4[94]),
            "Ice Gimos Fight": RupeeSilver @Chest(3[109]),
            "Lorule Mountain E Ledge": RupeeGold @Chest(4[25]),
            "Lorule Mountain W Ledge": LiverBlue @Chest(3[108]),
            "Treacherous Tower Advanced (1)": ItemKandelaarLv2 @Event(FieldDark_05_GameTower[0x8D]),
            "Treacherous Tower Advanced (2)": ItemInsectNetLv2 @Event(FieldDark_05_GameTower[0x98]),
            "Treacherous Tower Intermediate": HeartPiece @Event(FieldDark_05_GameTower[0x7E]),
            "[Mai] Lorule Mountain E Big Rock": Maiamai @Maiamai(4[24]),
            "[Mai] Lorule Mountain E Skull": Maiamai @Maiamai(4[22]),
            "[Mai] Lorule Mountain E Wall": Maiamai @Maiamai(4[23]),
            "[Mai] Lorule Mountain W Big Rock": Maiamai @Maiamai(3[43]),
            "[Mai] Lorule Mountain W Skull": Maiamai @Maiamai(3[42]),
            "[Mai] Outside Ice Ruins": Maiamai @Maiamai(5[20]),
        ],
    },
}
