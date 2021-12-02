crate::region! {
    course: FieldLight,
    name: "Lake Hylia",
    hylia {
        locations: [
            "Torch Cave": RupeePurple @Chest(CaveLight 11[8]) :- can_light,
            "Ledge Chest": RupeeR @Chest(35[155]) :- can_merge,
            "Bird Lover": ItemBottle @Event(FieldLight_2D_UnderBridgeStranger[0x2A]) :- can_swim,
            "Secret Cave": RupeeGold @Chest(CaveLight 9[12]) :- can_bomb,
            "Shore": MessageBottle @None() :- can_swim,
        ],
        paths: [
            island :- can_swim,
            lorule::lake::lorule :- lorule,
        ],
    },
    island {
        paths: [
            field::rentals,
            dungeons::house::gales :- can_tornado_rod,
        ],
    },
    hotfoot {
        locations: [
            "Hyrule Hotfoot": HeartPiece @Event(FieldLight_HyruleRace[0x14]) :-
                {|p| p.has_boots() && p.has_master_sword()},
        ],
    },
}
