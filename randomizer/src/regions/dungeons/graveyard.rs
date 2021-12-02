crate::region! {
    course: AttractionDark,
    name: "Graveyard",
    main {
        locations: [
            "Entrance": RupeeG @Chest(2[219]),
            "Lower Chest": RupeeB @Chest(2[45]),
            "Upper Chest": RupeeR @Chest(2[32]),
            "Ledge": KeySmall @Key(2[31]),
        ],
        paths: [
            lorule::graveyard::cave :- {|s| s.small_keys(COURSE) > 0},
        ],
    },
}
