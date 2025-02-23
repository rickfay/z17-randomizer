crate::region! {
    course: DungeonSand,
    name: "Desert Palace",
    color: Name,
    palace {
        locations: [
            "[DP] (1F) Entrance": RupeeB @Chest(1[78]),

            "[DP] (1F) Sand Room (South)": LiverBlue @Chest(1[565]),
            "[DP] (1F) Sand Switch Room": Compass @Chest(1[289]),
            "[DP] (1F) Sand Room (North)": KeySmall @Chest(1[371]),
            "[DP] (1F) Behind Rocks": KeySmall @Chest(1[349]),
            "[DP] (1F) Big Chest (Behind Wall)": PowerfulGlove @BigChest(1[70]),

            "[DP] (2F) Under Rock (Left)": RupeeSilver @Chest(2[550]),
            "[DP] (2F) Beamos Room": RupeeSilver @Chest(2[276]),
            "[DP] (2F) Under Rock (Right)": RupeeSilver @Chest(2[548]),
            "[DP] (2F) Under Rock (Ball Room)": RupeeSilver @Chest(2[545]),
            "[DP] (2F) Big Chest (Puzzle)": KeyBoss @BigChest(2[35]),
            "[DP] (2F) Red/Blue Switches": KeySmall @Chest(2[462]),

            "[DP] (2F) Leever Room": KeySmall @Chest(2[257]),

            "[DP] (3F) Behind Falling Sand": RupeeSilver @Chest(3[195]),
            "[DP] (3F) Armos Room": KeySmall @Chest(3[110]),

            "[DP] Zaganaga": HeartContainer @Heart(FieldDark 31[83]),
            "[DP] Prize": None @None(),
        ],
    },
}
