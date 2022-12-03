crate::region! {
    course: FieldDark,
    name: "Lorule Field",
    main {
        locations: [
            "Boots Treasure Dungeon": RupeeGold @BigChest(AttractionDark 1[12]),
            "Vacant House": ItemBottle @Chest(IndoorDark 11[31]),
            "Rupee Rush (Lorule)": HeartPiece @Event(FieldDark_28_Minigame[0x26]),
            "Great Rupee Fairy": ItemBottle @Event(CaveDark/Cave[0x24]),
            "Big Bomb Cave": RupeeSilver @Chest(CaveDark 2[4]),
            "Octoball Derby": HeartPiece @Event(FieldDark_2A_GameMaster[0x21]),
            "Blacksmith (Lorule)": ItemSwordLv4 @Event(IndoorDark/FiledDark_22_BlackSmithUra[0x1F]),
            "Swamp Cave (Left)": RupeeSilver @Chest(CaveDark 3[3]),
            "Swamp Cave (Middle)": HeartPiece @Heart(CaveDark 3[8]),
            "Swamp Cave (Right)": RupeeGold @Chest(CaveDark 3[6]),
            "Hookshot Ledge": RupeeSilver @Chest(32[95]),
            "Thief Girl Cave": RupeeSilver @Chest(CaveDark 15[9]),
            "Chamber of Sages - Osfala": ItemRentalSandRodFirst @Event(CaveDark/CaveDark10[226]),
        ],
    },
}
