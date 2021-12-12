crate::region! {
    course: FieldDark,
    name: "Lorule Lake",
    lorule {
        locations: [
            "Chest": RupeeSilver @Chest(28[53]),
        ],
        paths: [
            dungeons::turtle::rock :- {|p| p.can_ice_rod() && (p.can_swim() || (p.glitched() && p.fake_flippers()))},
        ],
    },
    balcony {
        locations: [
            "Balcony": HeartPiece @Heart(35[54]),
        ],
    },
}
