crate::region! {
    course: FieldLight,
    name: "Eastern Ruins",
    ruins {
        paths: [
            hill :- {|p| p.can_use_projectile() || p.can_merge() || (p.glitched() && p.can_lift())},
            lorule::dark::ruins :- can_merge,
        ],
    },
    hill {
        locations: [
            "Merge Treasure Dungeon": RupeeSilver @Chest(AttractionLight 1[4]) :- can_merge,
            "Armos Chest": RupeeR @Chest(20[106]),
            "Hookshot Chest": RupeeR @Chest(20[111]) :- can_hookshot,
            "Merge Chest": RupeeSilver @Chest(20[107]) :- {|p| p.can_merge() || (p.glitched() && (p.can_tornado_rod() || p.can_ledge_boost()))},
            "Cave": HeartPiece @Heart(CaveLight 29[10]) :- {|s| s.can_bomb() && s.can_merge()},
            "Pegs (South)": HeartPiece @Heart(30[41]) :- {|p| p.can_hammer() || (p.glitched() && (p.can_boomerang() || p.can_hookshot() || p.can_tornado_rod()))}, // spare them from SRod
        ],
        paths: [
            dungeons::eastern::palace,
        ],
    },
}
