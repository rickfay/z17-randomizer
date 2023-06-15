crate::region! {
    course: FieldLight,
    name: "Zora's River",
    color: Name,
    river {
        locations: [
            "Queen Oren": ItemMizukaki @Event(CaveLight/FieldLight_0F_Zora[0x6B]),
            "Waterfall Cave": HeartPiece @Heart(CaveLight 13[103]),
            "Zora's Domain Ledge": RupeeR @Chest(15[35]),
            "Zora's River Treasure Dungeon": RupeeSilver @Chest(AttractionLight 5[24]),
            "[Mai] Inside Witch's House": Maiamai @Maiamai(IndoorLight 2[12]),
            "[Mai] Under Wooden Bridge": Maiamai @Maiamai(19[39]),
            "[Mai] Waterfall Ledge Wall": Maiamai @Maiamai(13[28]),
            "[Mai] Zora's Domain South Wall": Maiamai @Maiamai(15[26]),
            "[Mai] Zora's Domain Water": Maiamai @Maiamai(7[25]),
        ],
    },
}
