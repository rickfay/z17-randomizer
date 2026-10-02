crate::region! {
    course: FieldLight,
    name: "Southern Ruins",
    color: Name,
    ruins {
        locations: [
            "Southern Ruins Fairy Cave Entrance": None @Door (33[303] SouthernRuinsFairyCaveEntrance),
            "Southern Ruins Fairy Cave Exit": None @Door (CaveLight 26[5] SouthernRuinsFairyCaveExit),
            "Runaway Item Seller Cave Entrance": None @Door (33[314] RunawayItemSellerCaveEntrance),
            "Runaway Item Seller Cave Exit": None @Door (CaveLight 27[3] RunawayItemSellerCaveExit),
            "Southern Ruins Mini Dungeon Entrance": None @Door (33[330] SouthernRuinsMiniDungeonEntrance),
            "Southern Ruins Mini Dungeon Exit": None @Door (AttractionLight 2[9] SouthernRuinsMiniDungeonExit),
            "Southern Ruins Bomb Cave Entrance": None @Door (33[319] SouthernRuinsBombCaveEntrance),
            "Southern Ruins Bomb Cave Exit": None @Door (CaveLight 28[10] SouthernRuinsBombCaveExit),
            "Southern Ruins Pillar Cave Entrance": None @Door (33[316] SouthernRuinsPillarCaveEntrance),
            "Southern Ruins Pillar Cave Exit": None @Door (CaveLight 28[11] SouthernRuinsPillarCaveExit),

            "Hyrule Swamp Pillar Crack": None @Crack(33[298] SwampPillarHyrule),

            "Runaway Item Seller": RupeeSilver @Event(Boot/FieldLight_33_Douguya[0x49]),
            "Southern Ruins Ledge": RupeeSilver @Chest(33[320]),
            "Southern Ruins Pillar Cave": HeartPiece @Heart(33[313]),
            "Flippers Mini-Dungeon": RupeeSilver @Chest(AttractionLight 2[33]),
            "[Mai] Southern Ruins Bomb Cave": Maiamai @Maiamai(CaveLight 28[35]),
            "[Mai] Southern Ruins Pillars": Maiamai @Maiamai(33[291]),
            "[Mai] Outside Flippers Mini-Dungeon": Maiamai @Maiamai(33[290]),
        ],
    },
}
