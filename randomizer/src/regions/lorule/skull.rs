crate::region! {
    course: FieldDark,
    name: "Skull Woods",
    woods {
        locations: [
            "Alcove": HeartPiece @Heart(16[124]) :- can_merge,
            "Balcony": HeartPiece @Heart(2[144]),
        ],
        paths: [
            dungeons::skull::palace,
        ],
    },
    chest {
        locations: [
            "Chest": RupeePurple @Chest(1[515]),
        ],
    },
}
