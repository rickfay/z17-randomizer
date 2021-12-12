crate::region! {
    course: FieldDark,
    name: "Graveyard",
    cave {
        locations: [
            "Big Chest": OreRed @Chest(CaveDark 5[18]),
        ],
    },
    field {
        locations: [
            "Field Chest": RupeeSilver @Chest(19[68]),
        ],
        paths: [
            dungeons::graveyard::main :- {|s| s.can_lift_big() && s.can_see_in_dark()},
            hyrule::field::sanctuary_cave :- {|s| s.can_bomb() && s.can_merge()},
        ],
    },
}
