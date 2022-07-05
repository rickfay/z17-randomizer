crate::region! {
    course: FieldLight,
    name: "Southern Ruins",
    ruins {
        locations: [
            "Runaway Item Seller": RupeeSilver @Event(Boot/FieldLight_33_Douguya[0x49]) :- can_bomb,
            "Behind Pillars": HeartPiece @Heart(33[313]) :- can_bomb,
            "Treasure Room": RupeeSilver @Chest(AttractionLight 2[33])
                :- {|p|
                (p.can_lift_big() && p.can_swim() && p.can_hookshot() && p.can_boomerang())
                || (p.glitched() && ((p.can_lift_big() || p.can_ice_rod()) && ((p.can_hookshot() && p.can_swim()) || p.nice_bombs())))
                },
            "Southern Ruins Ledge": RupeeSilver @Chest(33[320]) :- can_merge,
        ],
        paths: [
            lorule::misery::mire :- can_merge,
        ],
    },
}
