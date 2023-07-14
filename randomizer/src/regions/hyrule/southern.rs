crate::region! {
    course: FieldLight,
    name: "Southern Ruins",
    color: Name,
    ruins {
        locations: [
            "Runaway Item Seller": RupeeSilver @Event(Boot/FieldLight_33_Douguya[0x49]),
            "Southern Ruins Ledge": RupeeSilver @Chest(33[320]),
            "Southern Ruins Pillar Cave": HeartPiece @Heart(33[313]),
            "Southern Ruins Treasure Dungeon": RupeeSilver @Chest(AttractionLight 2[33]),
            "[Mai] Southern Ruins Bomb Cave": Maiamai @Maiamai(CaveLight 28[35]),
            "[Mai] Southern Ruins Pillars": Maiamai @Maiamai(33[291]),
            "[Mai] Outside Flippers Dungeon": Maiamai @Maiamai(33[290]),
        ],
    },
}
