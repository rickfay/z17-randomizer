crate::region! {
    course: DungeonHera,
    name: "Tower of Hera",
    color: Attention,
    hera {
        locations: [
            "[TH] (1F) Outside": RupeePurple @Chest(1[6]),
            "[TH] (1F) Center": Compass @Chest(1[5]),
            "[TH] (3F) Platform": KeySmall @Key(1[244]),
            "[TH] (5F) Red/Blue Switches": RupeeB @Chest(1[251]),
            "[TH] (6F) Left Mole": KeySmall @Key(1[334]),
            "[TH] (6F) Right Mole": LiverPurple @Chest(1[694]),
            "[TH] (7F) Outside (Ledge)": RupeeSilver @Chest(1[793]),
            "[TH] (8F) Fairy Room": RupeePurple @Chest(1[838]),
            "[TH] (11F) Big Chest": KeyBoss @BigChest(1[741]),
            "[TH] Moldorm": HeartContainer @Heart(1[772]),
            "[TH] Prize": PendantPower @None(),
        ],
    },
}
