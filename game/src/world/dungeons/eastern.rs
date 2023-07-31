crate::region! {
    course: DungeonEast,
    name: "Eastern Palace",
    palace {
        locations: [
            "[EP] (1F) Merge Chest": RupeeSilver @Chest(1[244]),
            "[EP] (1F) Left Door Chest": RupeeR @Chest(1[132]),
            "[EP] (1F) Popo Room": Compass @Chest(1[61]),
            "[EP] (1F) Secret Room": RupeeR @Chest(1[142]),
            "[EP] (1F) Switch Room": KeySmall @Chest(1[74]),
            "[EP] (2F) Ball Room": LiverPurple @Chest(2[147]),
            "[EP] (2F) Defeat Popos": LiverPurple @Chest(2[115]),
            "[EP] (2F) Switch Room": KeySmall @Chest(2[52]),
            "[EP] (2F) Big Chest": KeyBoss @BigChest(2[44]),
            "[EP] Yuga (1)": ItemRentalBow @Event(East[0x1C]),
            "[EP] Yuga (2)": HeartContainer @Heart(3[94]),
            "[EP] (3F) Escape Chest": RupeeSilver @Chest(3[25]),
            "[EP] (1F) Escape Chest": RupeePurple @Chest(1[235]),
            "Eastern Palace Prize": PendantCourage @None(), // @Event(FieldLight/FieldLight_1B_Sahasrahla[0x17]),
        ],
    },
}
