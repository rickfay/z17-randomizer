crate::region! {
    course: FieldDark,
    name: "Lorule Lake",
    lorule {
        locations: [
            "Chest": RupeeSilver @Chest(28[53]),
        ],
        paths: [
            dungeons::turtle::rock :- {|p| p.can_swim() && p.can_ice_rod()},
        ],
    },
    balcony {
        locations: [
            "Balcony": HeartPiece @Heart(35[54]),
        ],
    },
}
