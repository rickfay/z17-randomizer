crate::region! {
    course: DungeonHagure,
    name: "Thieves Hideout",
    hideout {
        locations: [
            "[T'H] (B1) Jail Cell": RupeePurple @Chest(1[1323]),
            "[T'H] (B1) Grate Chest": RupeePurple @Chest(1[576]),
            "[T'H] (B2) Grate Chest (Fall)": RupeePurple @Chest(1[1292]),
            "[T'H] (B2) Switch Puzzle Room": LiverPurple @Chest(1[949]),
            "[T'H] (B2) Jail Cell": Compass @Chest(1[283]),
            "[T'H] (B2) Eyegores": KeySmall @Chest(1[543]),
            "[T'H] (B1) Behind Wall": RupeeSilver @Chest(1[1359]),
            "[T'H] (B1) Big Chest (Entrance)": KeyBoss @Chest(1[580]),
            "[T'H] (B3) Underwater": RupeeSilver @Chest(1[667]),
            "[T'H] (B3) Big Chest (Hidden)": OreBlue @Chest(1[657]),
            "Stalblind": HeartContainer @Heart(IndoorDark 15[12]),
            "Thieves' Hideout Prize": None @None(),
        ],
    },
}
