crate::region! {
    course: CaveLight,
    name: "Sanctuary",
    lobby {
        locations: [
            "[HS] Entrance": ItemKandelaar @Chest(18[19]),
        ],
        paths: [
            inside :- {|p| (p.can_light() && p.can_lamp()) || (p.glitched() && p.can_damage())},
        ],
    },
    inside {
        locations: [
            "[HS] Lower Chest": RupeeR @Chest(18[45]),
            "[HS] Upper Chest": KeySmall @Chest(18[32]),
            "[HS] Ledge": HeartPiece @Heart(18[31]) :- can_merge,
        ],
        paths: [
            end :- {|p| p.small_keys(COURSE) > 0},
        ],
    },
    end {
        quest: Sanctuary,
    },
}
