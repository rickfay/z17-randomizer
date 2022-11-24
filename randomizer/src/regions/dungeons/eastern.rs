crate::region! {
    course: DungeonEast,
    name: "Eastern Palace",
    palace {
        locations: [
            "[EP] (1F) Outside (East)": RupeeSilver @Chest(1[244]),
            "[EP] (1F) Near Entrance": RupeeR @Chest(1[132]),
            "[EP] (1F) Defeat Popos": Compass @Chest(1[61]),
            "[EP] (1F) Hidden Door": RupeeR @Chest(1[142]),
            "[EP] (1F) Switch Puzzle": KeySmall @Chest(1[74]),
            "[EP] (2F) Ball Room": LiverPurple @Chest(2[147]),
            "[EP] (2F) Defeat Popos": LiverPurple @Chest(2[115]),
            "[EP] (2F) Switch Room": KeySmall @Chest(2[52]),
            "[EP] (2F) Big Chest": KeyBoss @Chest(2[44]),
            "[EP] (3F) After Cutscene": ItemRentalBow @Event(East[0x1C]),
            "[EP] Yuga": HeartContainer @Heart(3[94]),
            "[EP] (3F) Outside (North)": RupeeSilver @Chest(3[25]),
            "[EP] (1F) Outside (West)": RupeePurple @Chest(1[235]),
            "Eastern Palace Prize": PendantCourage @None(), // @Event(FieldLight/FieldLight_1B_Sahasrahla[0x17]),
        ],
    },
}
