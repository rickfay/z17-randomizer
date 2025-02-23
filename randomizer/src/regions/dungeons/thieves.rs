crate::region! {
    course: DungeonHagure,
    name: "Thieves' Hideout",
    color: Beige,
    hideout {
        locations: [
            "[TT] (B1) Jail Cell": RupeePurple @Chest(1[1323]),
            "[TT] (B1) Grate Chest": RupeePurple @Chest(1[576]),
            "[TT] (B2) Grate Chest (Fall)": RupeePurple @Chest(1[1292]),
            "[TT] (B2) Switch Puzzle Room": LiverPurple @Chest(1[949]),
            "[TT] (B2) Jail Cell": Compass @Chest(1[283]),
            "[TT] (B2) Eyegores": KeySmall @Chest(1[543]),
            "[TT] (B1) Behind Wall": RupeeSilver @Chest(1[1359]),
            "[TT] (B1) Big Chest (Entrance)": KeyBoss @BigChest(1[580]),
            "[TT] (B3) Underwater": RupeeSilver @Chest(1[667]),
            "[TT] (B3) Big Chest (Hidden)": OreBlue @BigChest(1[657]),
            "[TT] Stalblind": HeartContainer @Heart(IndoorDark 15[12]),
            "[TT] Prize": None @None(),
        ],
    },
}
