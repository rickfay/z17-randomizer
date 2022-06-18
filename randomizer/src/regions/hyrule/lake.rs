crate::region! {
    course: FieldLight,
    name: "Lake Hylia",
    hylia {
        locations: [
            "Torch Cave": RupeePurple @Chest(CaveLight 11[8]) :- can_light,
            "Lake Hylia Ledge Chest": RupeeR @Chest(35[155]) :- can_merge,
            "Bird Lover": ItemBottle @Event(FieldLight_2D_UnderBridgeStranger[0x2A]) :- can_swim,
            "Secret Cave": RupeeGold @Chest(CaveLight 9[12]) :- can_bomb,
            "Shore": MessageBottle @None() :- {|p| p.can_swim() || (p.glitched() && p.can_ledge_boost())},
        ],
        paths: [
            island :- {|p| p.can_swim() || (p.glitched() && (p.fake_flippers() || (p.can_ice_rod() && p.can_hookshot())))},
            hotfoot :- did_eastern,
            lorule::lake::lorule :- can_merge,
        ],
    },
    island {
        paths: [
            dungeons::house::gales :- can_tornado_rod,
        ],
    },
    hotfoot {
        locations: [
            "Hyrule Hotfoot": HeartPiece @Event(FieldLight_HyruleRace[0x14]) :-
                {|p| p.has_boots() && p.did_eastern()}, // Present after Irene Trigger
        ],
    },
}
