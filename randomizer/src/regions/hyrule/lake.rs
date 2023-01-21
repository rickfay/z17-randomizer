crate::region! {
    course: FieldLight,
    name: "Lake Hylia",
    hylia {
        locations: [
            "Torch Cave": RupeePurple @Chest(CaveLight 11[8]),
            "Lake Hylia Ledge Chest": RupeeR @Chest(35[155]),
            "Bird Lover": ItemBottle @Event(FieldLight_2D_UnderBridgeStranger[0x2A]),
            "Secret Cave": RupeeGold @Chest(CaveLight 9[12]),
            "Shore": MessageBottle @None(),
            "Hyrule Hotfoot - First Race": RupeeSilver @Event(FieldLight_HyruleRace[0x21]),
            "Hyrule Hotfoot - Second Race": HeartPiece @Event(FieldLight_HyruleRace[0x14]),
        ],
    },
}
