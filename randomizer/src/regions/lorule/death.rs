crate::region! {
    course: FieldDark,
    name: "Death Mountain",
    mountain {
        locations: [
            "Ledge (East)": RupeeGold @Chest(4[25]) :- can_tornado_rod,
            "Behind Ice Gimos": RupeeSilver @Chest(4[94]) :- can_fire_rod,
        ],
        paths: [
            west :- can_hookshot,
            dungeons::ice::ruins :- can_fire_rod,
            hyrule::death::far_island :- can_tornado_rod,
        ],
    },
    west {
        locations: [
            "Ledge (West)": LiverBlue @Chest(3[108]),
            "Ice Gimos (West)": RupeeSilver @Chest(3[109]) :- can_fire_rod,
        ],
        paths: [
            tower,
        ],
    },
    tower {
        locations: [
            "Treacherous Tower (Intermediate)": HeartPiece @Event(FieldDark_05_GameTower[0x7E]) :- can_hammer,
        ],
    },
}
