crate::region! {
    course: FieldLight,
    name: "Lost Woods",
    woods {
        locations: [
            "Pedestal": ItemSwordLv2 @Chest(34[71]) :- has_three_pendants,
            "Alcove": HeartPiece @Heart(1[46]) :- {|p| p.can_merge() || (p.glitched() && ((p.can_boomerang() || p.can_hookshot()) && (p.can_escape())))},
            "Chest": RupeeR @Chest(1[133]) :- {|p| p.can_lift_big() || (p.glitched() && ((p.can_boomerang() || p.can_hookshot()) && (p.can_escape())))},
        ],
        paths: [
            lorule::skull::woods :- can_merge,
        ],
    },
}
