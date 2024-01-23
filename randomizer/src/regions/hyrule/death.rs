crate::region! {
    course: FieldLight,
    name: "Death Mountain",
    color: Name,
    mountain {
        locations: [
            "Hyrule Death West Portal": None @Portal(3[361] DeathWestHyrule),
            "Hyrule Floating Island Portal": None @Portal(4[111] FloatingIslandHyrule),
            "Hyrule Rosso's Ore Mine Portal": None @Portal(4[65] RossosOreMineHyrule),

            "Death Mountain (Hyrule) Weather Vane": None @WeatherVane(3[349] DeathMountainHyruleWV),
            "Tower of Hera Weather Vane": None @WeatherVane(3[335] TowerOfHeraWV),

            "Bouldering Guy": ItemBottle @Event(FieldLight_05_Climber[0x06]),
            "Death Mountain Open Cave": RupeeB @Chest(CaveLight 3[58]),
            "Death Mountain Blocked Cave": RupeePurple @Chest(CaveLight 3[59]),
            "Death Mountain Fairy Cave": LiverPurple @Chest(CaveLight 3[83]),
            "Death Mountain West Highest Cave": LiverPurple @Chest(CaveLight 2[166]),
            "Donkey Cave": LiverPurple @Chest(CaveLight 1[67]),
            "Donkey Cave Ledge": RupeeR @Chest(3[303]),
            "Fire Cave Pillar": HeartPiece @Heart(CaveLight 25[9]),
            "Floating Island": HeartPiece @Heart(4[25]),
            "Hookshot Mini-Dungeon": RupeeSilver @Chest(AttractionLight 4[89]),
            "Spectacle Rock": HeartPiece @Heart(3[302]),
            "[Mai] Death Mountain Base Rock": Maiamai @Maiamai(3[357]),
            "[Mai] Fire Cave Ledge": Maiamai @Maiamai(4[63]),
            "[Mai] Death Mountain West Ledge": Maiamai @Maiamai(3[358]),
            "[Mai] Outside Hookshot Mini-Dungeon": Maiamai @Maiamai(5[23]),
            "[Mai] Rosso's Ore Mine": Maiamai @Maiamai(4[64]),
        ],
    },
}
