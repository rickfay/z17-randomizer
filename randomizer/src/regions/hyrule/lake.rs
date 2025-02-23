crate::region! {
    course: FieldLight,
    name: "Lake Hylia",
    color: Name,
    hylia {
        locations: [
            "Lake Hylia Crack": None @Crack(35[76] LakeHylia),
            "Hyrule Hotfoot Crack": None @Crack(36[30] HyruleHotfoot),

            "House of Gales Weather Vane": None @WeatherVane(35[45] HouseOfGalesWV),

            "Ice Rod Cave": RupeeGold @Chest(CaveLight 9[12]),
            "Lake Hylia Dark Cave": RupeePurple @Chest(CaveLight 11[8]),
            "Lake Hylia Eastern Shore": MessageBottle @Heart(36[38]),
            "Lake Hylia Ledge Chest": RupeeR @Chest(35[155]),
            "Lakeside Item Shop (1)": EscapeFruit @None(),
            "Lakeside Item Shop (2)": StopFruit @None(),
            "Lakeside Item Shop (3)": ItemShield @None(),
            "[Mai] Hyrule Hotfoot Rock": Maiamai @Maiamai(36[31]),
            "[Mai] Lake Hylia Island Tile": Maiamai @Maiamai(35[130]),
            "[Mai] Lake Hylia East River": Maiamai @Maiamai(40[24]),
            "[Mai] Lake Hylia Shallow Ring": Maiamai @Maiamai(35[131]),
            "[Mai] Outside Maiamai Cave": Maiamai @Maiamai(35[129]),
        ],
    },
    cave {
        locations: [
            "Maiamai Bow Upgrade": ItemBowLv2 @None(),
            "Maiamai Boomerang Upgrade": ItemBoomerangLv2 @None(),
            "Maiamai Hookshot Upgrade": ItemHookshotLv2 @None(),
            "Maiamai Hammer Upgrade": ItemHammerLv2 @None(),
            "Maiamai Bombs Upgrade": ItemBombLv2 @None(),
            "Maiamai Fire Rod Upgrade": ItemFireRodLv2 @None(),
            "Maiamai Ice Rod Upgrade": ItemIceRodLv2 @None(),
            "Maiamai Tornado Rod Upgrade": ItemTornadeRodLv2 @None(),
            "Maiamai Sand Rod Upgrade": ItemSandRodLv2 @None(),
            "100 Maiamai": SpecialMove @Event(CaveLight/FieldLight_35_Kinsta[23]),
        ],
    },
}
