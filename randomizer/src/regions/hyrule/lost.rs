crate::region! {
    course: FieldLight,
    name: "Lost Woods Region",
    woods {
        locations: [
            "Fortune-Teller": HintGlasses @Event(IndoorLight/FieldLight_11_FortuneGirl[4]),
            "Hyrule Hotfoot (First Race)": RupeeSilver @Event(FieldLight_HyruleRace[0x21]),
            "Hyrule Hotfoot (Second Race)": HeartPiece @Event(FieldLight_HyruleRace[0x14]),
            "Lost Woods Alcove": HeartPiece @Heart(1[46]),
            "Lost Woods Big Rock Chest": RupeeR @Chest(1[133]),
            "Master Sword Pedestal": ItemSwordLv2 @Chest(34[71]),
            "Rosso": PowerGlove @Chest(IndoorLight 10[7]),
            "Rosso Cave": RupeeR @Chest(CaveLight 6[6]),
            "Rosso Rocks": RupeePurple @Chest(IndoorLight 10[25]),

            "[Mai] Rosso Wall": Maiamai @Maiamai(2[98]),
            "[Mai] Lost Woods Path Rock": Maiamai @Maiamai(8[29]),
            "[Mai] Lost Woods Bush": Maiamai @Maiamai(1[239]),
            "[Mai] Lost Woods Rock": Maiamai @Maiamai(1[47]),
            "[Mai] Fortune-Teller Tent": Maiamai @Maiamai(9[90]),
            "[Mai] Moldorm Ledge": Maiamai @Maiamai(6[132]),
            "[Mai] Small Pond": Maiamai @Maiamai(10[62]),
            "[Mai] Lost Woods Tree": Maiamai @Maiamai(1[229]),
        ],
    },
}
