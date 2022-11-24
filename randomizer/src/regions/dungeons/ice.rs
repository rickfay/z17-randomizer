crate::region! {
    course: DungeonIce,
    name: "Ice Ruins",
    ruins {
        locations: [
            "[IR] (1F) Hidden Chest": RupeeGold @Chest(1[1048]),
            "[IR] (B3) Grate Chest (Left)": RupeeG @Chest(1[840]),
            "[IR] (B3) Grate Chest (Right)": LiverYellow @Chest(1[893]),
            "[IR] (B4) Ice Pillar": KeySmall @Key(1[1057]),
            "[IR] (B5) Big Chest": KeyBoss @Chest(1[282]),
            "[IR] (B1) East Chest": Compass @Chest(1[108]),
            "[IR] (B1) Narrow Ledge": KeySmall @Key(1[98]),
            "[IR] (B1) Upper Chest": RupeeB @Chest(1[1026]),
            "[IR] (B3) Big Chest (Puzzle)": GanbariPowerUp @Chest(1[18]),
            "[IR] (B4) Switches": RupeeSilver @Chest(1[25]),
            "[IR] (B4) Southwest Chest (Fall)": RupeePurple @Chest(1[1122]),
            "[IR] (B4) Narrow Platform": LiverPurple @Chest(1[913]),
            "[IR] (B2) Far North": RupeeSilver @Chest(1[838]),
            "[IR] (B4) Southeast Chest (Fall)": KeySmall @Chest(1[273]),
            "[IR] Dharkstare": HeartContainer @Heart(1[554]),
            "Ice Ruins Prize": None @None(),
        ],
    },
}
