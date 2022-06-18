crate::region! {
    course: FieldDark,
    name: "Skull Woods",
    woods {
        locations: [
            "Canyon House": HeartPiece @Heart(16[124]) :- can_merge,
            "Cucco Shack": HeartPiece @Heart(2[144]),
        ],
        paths: [
            dungeons::skull::palace :- can_see_in_dark,
        ],
    },
    chest {
        locations: [
            "Skull Woods Outdoor Chest": ItemKandelaarLv2 @Chest(1[515]),
        ],
    },
}
