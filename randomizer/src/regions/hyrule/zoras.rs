crate::region! {
    course: FieldLight,
    name: "Zoras Domain",
    domain {
        locations: [
            "Chest": RupeeR @Chest(15[35]) :- can_merge,
            "Behind Waterfall": HeartPiece @Heart(CaveLight 13[103]) :- can_swim,
            "Zora Queen": ItemMizukaki @Event(CaveLight/FieldLight_0F_Zora[0x6B]) :- has_smooth_gem,
        ],
        paths: [
            kakariko::shady_guy,
            field::rentals :- has_smooth_gem,
        ],
    },
}
