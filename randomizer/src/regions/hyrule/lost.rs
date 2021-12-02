crate::region! {
    course: FieldLight,
    name: "Lost Woods",
    woods {
        locations: [
            "Pedestal": ItemSwordLv2 @Chest(34[71]) :- has_three_pendants,
            "Alcove": HeartPiece @Heart(1[46]) :- can_merge,
            "Chest": RupeeR @Chest(1[133]) :- can_lift_big,
        ],
        paths: [
            lorule::skull::woods :- lorule,
        ],
    },
}
