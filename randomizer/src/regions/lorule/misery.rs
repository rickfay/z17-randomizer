crate::region! {
    course: FieldDark,
    name: "Misery Mire",
    mire {
        locations: [
            "Ledge": HeartPiece @Heart(31[82])
                :- {|s| s.can_sand_rod() && s.can_bomb()},
            "Treasure Room": RupeeGold @Chest(AttractionDark 3[7])
                :- {|p| p.can_sand_rod() && p.can_tornado_rod()},
        ],
        paths: [
            dungeons::desert::palace :- {|p| p.can_merge() && p.can_sand_rod()},
        ],
    },
}
