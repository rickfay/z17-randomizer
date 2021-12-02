crate::region! {
    course: FieldLight,
    name: "Eastern Ruins",
    ruins {
        paths: [
            hill :- {|p| p.can_use_projectile() || p.can_merge()},
            lorule::dark::ruins :- lorule,
        ],
    },
    hill {
        locations: [
            "Treasure Room": RupeeSilver @Chest(AttractionLight 1[4]) :- can_merge,
            "Armos Chest": RupeeR @Chest(20[106]),
            "Hookshot Chest": RupeeR @Chest(20[111]) :- can_hookshot,
            "Merge Chest": RupeeSilver @Chest(20[107]) :- can_merge,
            "Cave": HeartPiece @Heart(CaveLight 29[10]) :- {|s| s.can_bomb() && s.can_merge()},
            "Pegs (South)": HeartPiece @Heart(30[41]) :- can_hammer,
        ],
        paths: [
            dungeons::eastern::palace,
        ],
    },
}
