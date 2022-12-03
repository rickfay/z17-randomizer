crate::region! {
    course: DungeonDark,
    name: "Dark Palace",
    palace {
        locations: [
            "[PD] (1F) Near Entrance": RupeeB @Chest(2[23]),
            "[PD] (1F) Narrow Ledge": KeySmall @Key(2[25]),
            "[PD] (1F) Switch Puzzle": Compass @Chest(2[122]),
            "[PD] (1F) Hidden Room (Upper)": RupeePurple @Chest(2[102]),
            "[PD] (1F) Hidden Room (Lower)": LiverBlue @Chest(2[233]),
            "[PD] (B1) Fall From 1F": KeySmall @Key(1[26]),
            "[PD] (B1) Maze": KeySmall @Chest(1[102]),
            "[PD] (B1) Helmasaur Room": KeySmall @Key(1[281]),
            "[PD] (B1) Helmasaur Room (Fall)": LiverYellow @Chest(1[100]),
            "[PD] (2F) Big Chest (Hidden)": OreYellow @BigChest(3[41]),
            "[PD] (2F) Alcove": LiverPurple @Chest(3[269]),
            "[PD] (1F) Fall From 2F": RupeePurple @Chest(2[127]),
            "[PD] (2F) South Hidden Room": RupeeGold @GoldRupee(3[166]),
            "[PD] (B1) Big Chest (Switches)": KeyBoss @BigChest(1[84]),
            "[PD] Gemesaur King": HeartContainer @Heart(1[119]),
            "Dark Palace Prize": None @None(),
        ],
    },
}
