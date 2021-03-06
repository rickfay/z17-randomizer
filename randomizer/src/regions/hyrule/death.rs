crate::region! {
    course: FieldLight,
    name: "Death Mountain",
    mountain {
        locations: [
            "First Cave": RupeeB @Chest(CaveLight 3[58]),
            "Blocked Cave": ItemInsectNetLv2 @Chest(CaveLight 3[59]) :- {|p| p.can_merge() && p.can_bomb()},
            "Fairy Cave": LiverPurple @Chest(CaveLight 3[83]) :- {
                |p| (p.can_merge() && (p.can_bomb() || p.can_hammer()))
                || (p.glitched() && (p.nice_bombs() || (p.can_fire_rod() && (p.can_bomb() || p.can_hammer()))))
            },
        ],
        paths: [
            upper :- {|p| p.can_merge() || p.glitched()},
            lorule::death::west :- can_merge,
        ],
    },
    upper {
        locations: [
            "Death Mountain West Ledge": RupeeR @Chest(3[303]),
            "Rock Cave (Pegs)": LiverPurple @Chest(CaveLight 1[67]) :- can_hammer,
            "Rock Cave (Top)": LiverPurple @Chest(CaveLight 2[166]),
            "Spectacle Rock": HeartPiece @Heart(3[302]),
        ],
        paths: [
            dungeons::tower::hera :- can_hammer,
            east :- can_hookshot,
        ],
    },
    east {
        locations: [
            "Fire Cave Pillar": HeartPiece @Heart(CaveLight 25[9]) :- {|s| s.can_merge() && s.can_hammer()},
            "Bouldering Guy": ItemBottle @Event(FieldLight_05_Climber[0x06])
                :- {|p| p.has_premium_milk() && (p.can_merge() || (p.glitched() && p.can_tornado_rod() && p.has_boots()))},
            "Hookshot Treasure Dungeon": RupeeSilver @Chest(AttractionLight 4[89])
                :- {|p| p.can_hookshot() && p.can_merge()},
        ],
        paths: [
            lorule::death::mountain :- can_merge,
        ],
    },
    far_island {
        locations: [
            "Floating Island": HeartPiece @Heart(4[25]),
        ],
    },
}
