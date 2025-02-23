crate::region! {
    course: FieldLight,
    name: "Lost Woods Area",
    color: Name,
    woods {
        locations: [
            "Lost Woods Pillar Crack": None @Crack(1[227] LostWoodsPillar),
            "Rosso's House Crack": None @Crack(IndoorLight 10[19] RossosHouse),

            "Fortune-Teller": HintGlasses @Event(IndoorLight/FieldLight_11_FortuneGirl[4]),
            "Hyrule Hotfoot 75s": RupeeSilver @Event(FieldLight_HyruleRace[0x21]),
            "Hyrule Hotfoot 65s": HeartPiece @Event(FieldLight_HyruleRace[0x14]),
            "Lost Woods Alcove": HeartPiece @Heart(1[46]),
            "Lost Woods Chest": RupeeR @Chest(1[133]),
            "Master Sword Pedestal": ItemSwordLv2 @Chest(34[71]),
            "Rosso Cave": RupeeR @Chest(CaveLight 6[6]),
            "Rosso (1)": PowerGlove @Event(IndoorLight/FieldLight_02_KikoriMan[0]),
            "Rosso (2)": RupeePurple @Chest(IndoorLight 10[25]),
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
