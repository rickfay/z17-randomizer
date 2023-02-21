crate::region! {
    course: FieldLight,
    name: "Death Mountain",
    mountain {
        locations: [
            "Bouldering Guy": ItemBottle @Event(FieldLight_05_Climber[0x06]),
            "Death Mountain Open Cave": RupeeB @Chest(CaveLight 3[58]),
            "Death Mountain Blocked Cave": RupeePurple @Chest(CaveLight 3[59]),
            "Death Mountain Treasure Dungeon": RupeeSilver @Chest(AttractionLight 4[89]),
            "Death Mountain Fairy Cave": LiverPurple @Chest(CaveLight 3[83]),
            "Death Mountain West Highest Cave": LiverPurple @Chest(CaveLight 2[166]),
            "Death Mountain West Ledge": RupeeR @Chest(3[303]),
            "Donkey Cave Pegs": LiverPurple @Chest(CaveLight 1[67]),
            "Fire Cave Pillar": HeartPiece @Heart(CaveLight 25[9]),
            "Spectacle Rock": HeartPiece @Heart(3[302]),
            "Floating Island": HeartPiece @Heart(4[25]),
            "[Mai] Death Mountain East Ledge Rock": Maiamai @Maiamai(4[63]),
            "[Mai] Death Mountain West Rock": Maiamai @Maiamai(3[357]),
            "[Mai] Death Mountain West Wall": Maiamai @Maiamai(3[358]),
            "[Mai] Outside Death Mountain Treasure Dungeon": Maiamai @Maiamai(5[23]),
            "[Mai] Rosso's Ore Mine Rock": Maiamai @Maiamai(4[64]),
        ],
    },
}
