crate::region! {
    course: CaveLight,
    name: "Sanctuary",
    lobby {
        locations: [
            "Entrance": ItemKandelaar @Chest(18[19]),
        ],
        paths: [
            inside :- {|p| (p.can_light() && p.can_see_in_dark()) || (p.glitched() && p.can_damage())},
        ],
    },
    inside {
        locations: [
            "Lower Chest": RupeeR @Chest(18[45]),
            "Upper Chest": KeySmall @Chest(18[32]),
            "Ledge": HeartPiece @Heart(18[31]) :- can_merge,
        ],
        paths: [
            end :- {|p| p.small_keys(COURSE) > 0},
        ],
    },
    end {
        quest: Sanctuary,
    },
}
