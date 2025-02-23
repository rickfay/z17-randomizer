crate::region! {
    course: FieldDark,
    name: "Skull Woods Area",
    color: Name,
    overworld {
        locations: [
            "Skull Woods Pillar Crack": None @Crack(1[366] SkullWoodsPillar),
            "n-Shaped House Crack": None @Crack(16[92] NShapedHouse),
            "Destroyed House Crack": None @Crack(2[135] DestroyedHouse),

            "Skull Woods Weather Vane": None @WeatherVane(1[270] SkullWoodsWV),

            "Destroyed House": HeartPiece @Heart(2[144]),
            "Mysterious Man": GoldenBeeForSale @None(),
            "n-Shaped House": HeartPiece @Heart(16[124]),
            "[Mai] Destroyed House Tree": Maiamai @Maiamai(2[98]),
            "[Mai] n-Shaped House Wall": Maiamai @Maiamai(16[106]),
            "[Mai] Skull Woods Rock": Maiamai @Maiamai(8[35]),
            "[Mai] Skull Woods Bush": Maiamai @Maiamai(6[110]),
            "[Mai] Skull Woods Dry Pond": Maiamai @Maiamai(10[69]),
            "[Mai] Skull Woods Entrance Wall": Maiamai @Maiamai(9[64]),
            "[Mai] Skull Woods Grass": Maiamai @Maiamai(1[317]),
            "[Mai] Skull Woods Skull": Maiamai @Maiamai(1[318]),
        ],
    },
}
