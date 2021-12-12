crate::region! {
    course: FieldLight,
    name: "Eastern Ruins",
    ruins {
        paths: [
            hill :- {|p| p.can_use_projectile() || p.can_merge() || (p.glitched() && p.can_lift())},
            lorule::dark::ruins :- lorule,
        ],
    },
    hill {
        locations: [
            "Treasure Room": RupeeSilver @Chest(AttractionLight 1[4]) :- can_merge,
            "Armos Chest": RupeeR @Chest(20[106]),
            "Hookshot Chest": RupeeR @Chest(20[111]) :- can_hookshot,
            "Merge Chest": RupeeSilver @Chest(20[107]) :- {|p| p.can_merge() || (p.glitched() && (p.can_tornado_rod() || p.can_fire_rod() || p.can_bomb()))},
            "Cave": HeartPiece @Heart(CaveLight 29[10]) :- {|s| s.can_bomb() && s.can_merge()},
            "Pegs (South)": HeartPiece @Heart(30[41]) :- {|p| p.can_hammer() || (p.glitched() && (p.can_boomerang() || p.can_hookshot() || p.can_sand_rod()))}, // spare them from TRod
        ],
        paths: [
            dungeons::eastern::palace,
        ],
    },
}
