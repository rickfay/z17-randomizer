crate::region! {
    course: FieldDark,
    name: "Misery Mire",
    mire {
        locations: [
            "Ledge": HeartPiece @Heart(31[82]) :- {
                |s| (s.can_sand_rod() && s.can_bomb()) ||
                (s.glitched() && (s.can_bomb() || (s.can_fire_rod() && s.has_boots())))
            },
            "Treasure Room": RupeeGold @Chest(AttractionDark 3[7])
                :- {|p| p.can_sand_rod() && p.can_tornado_rod()},
        ],
        paths: [
            dungeons::desert::palace,
        ],
    },
}
