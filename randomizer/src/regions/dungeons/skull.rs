crate::region! {
    course: DungeonDokuro,
    name: "Skull Woods",
    woods {
        locations: [
            "[SW] (B1) Gibdo Room (Lower)": Compass @Chest(1[100]),
            "[SW] (B1) South Chest": KeySmall @Chest(1[101]),
            "[SW] (B1) Gibdo Room (Hole)": RupeeSilver @Chest(1[640]),
            "[SW] (B1) Grate Room": KeySmall @Chest(1[328]),
            "[SW] (B2) Moving Platform Room": KeySmall @Chest(2[105]),
            "[SW] (B1) Big Chest (Upper)": OreGreen @BigChest(1[653]),
            "[SW] (B1) Big Chest (Eyes)": KeyBoss @BigChest(1[289]),
            "[SW] Knucklemaster": HeartContainer @Heart(2[404]),
            "Skull Woods Prize": None @None(),
        ],
    },
}
