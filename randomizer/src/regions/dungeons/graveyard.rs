crate::region! {
    course: AttractionDark,
    name: "Lorule Sanctuary",
    main {
        locations: [
            "[LS] Entrance Chest": RupeeG @Chest(2[219]),
            "[LS] Lower Chest": RupeeB @Chest(2[45]),
            "[LS] Upper Chest": RupeeR @Chest(2[32]),
            "[LS] Ledge": KeySmall @Key(2[31]),
        ],
        paths: [
            lorule::graveyard::cave :- {|s| s.small_keys(COURSE) >= 1},
        ],
    },
}
