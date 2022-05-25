crate::region! {
    course: FieldDark,
    name: "Skull Woods",
    woods {
        locations: [
            "Alcove": HeartPiece @Heart(16[124]) :- can_merge,
            "Balcony": HeartPiece @Heart(2[144]),
        ],
        paths: [
            dungeons::skull::palace :- can_see_in_dark,
        ],
    },
    chest {
        locations: [
            "Chest": ItemKandelaarLv2 @Chest(1[515]),
        ],
    },
}
