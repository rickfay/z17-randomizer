crate::region! {
    course: FieldLight,
    name: "Hyrule Field",
    main {
        locations: [
            //"Delivery": PackageSword @None() where settings: !settings.items.captains_sword.is_skipped(),
            "Dampe": ItemSwordLv1 @Event(FieldLight_13_Sister[0x1D]),
            "Rosso Cave": RupeeR @Chest(CaveLight 6[6]) :- {|p| p.can_hammer() || (p.glitched() && (p.can_boomerang() || p.can_hookshot()))},
            "Sanctuary Pegs": RupeeSilver @Chest(11[89]) :- can_hammer,
            "Treasure Room": RupeeSilver @Chest(AttractionLight 5[24]) :- {|p| p.can_bomb() && p.can_merge()},
            "Behind Blacksmith": HeartPiece @Heart(17[95]) :- {|p| p.can_merge() || (p.glitched() && p.can_ledge_boost())},
            "Blacksmith Cave": HeartPiece @Heart(CaveLight 16[1]) :- {|p| p.can_lift_big() || (p.glitched() && p.can_ledge_boost())},
            "Blacksmith": ItemSwordLv3 @Event(IndoorLight/FieldLight_22_BlackSmith[0x16])
                :- {|p| p.lorule() && p.ore() >= 2},
            "Castle Rocks": HeartPiece @Heart(18[209]) :- can_lift,
            "Rosso": PowerGlove @Chest(IndoorLight 10[7]) :- {|p| p.did_eastern() || p.lorule()},
        ],
        paths: [
            rentals,
            sanctuary::lobby :- {|p| p.sword() || p.can_lift() || p.can_fire_rod() || p.can_ice_rod() || p.can_bomb() || p.can_lamp() || p.has_boots()},
            lost::woods,
            death::mountain :- {|p| p.can_lift() || p.glitched()},
            zoras::domain,
            kakariko::village,
            eastern::ruins,
            southern::ruins,
            lake::hylia,
            post_eastern :- did_eastern,
            castle :- has_master_sword,
            lorule::field::main :- lorule,
            lorule::graveyard::field :- lorule,
            lorule::field::ledge :- lorule,
        ],
    },
    post_eastern {
        locations: [
            "Clean Rocks": RupeePurple @Chest(IndoorLight 10[25]) :- can_lift,
            "Irene": ItemRentalBomb @Event[
                FieldLight_11_Maple[0x06],
                FieldLight_12_Maple[0x08],
                FieldLight_12_Maple[0x26],
                FieldLight_2D_Maple[0x07],
            ],
            "Woods": ItemRentalTornadeRod @Event(FieldLight_2A_BlacksmithWife[0x15]),
        ],
    },
    rentals {
        locations: [
            "Ravio (1)": ItemRentalIceRod @Shop(Ravio(0)),
            "Ravio (2)": ItemRentalHookShot @Shop(Ravio(5)),
            "Ravio (3)": Pouch @Shop(Ravio(2)),
            "Ravio (4)": ItemBell @Shop(Ravio(3)),
            "Ravio (5)": RingHekiga @Shop(Ravio(8)),
            "Ravio (6)": ItemRentalSandRod @Shop(Ravio(1)) :- osfala,
            "Ravio (7)": ItemRentalHammer @Shop(Ravio(7)),
            "Ravio (8)": ItemRentalBoomerang @Shop(Ravio(6)),
            "Ravio (9)": ItemRentalFireRod @Shop(Ravio(4)),
        ],
        paths: [
            rupee_rush,
        ],
    },
    rupee_rush {
        locations: [
            "Rupee Rush": HeartPiece @Event(FieldLight_28_Minigame[0x26]),
        ],
    },
    castle {
        locations: [
            "Castle (Indoors)": RupeeB @Chest(IndoorLight 12[48]),
            "Castle Balcony": RupeePurple @Chest(18[224]),
        ],
    },
    sanctuary_cave {
        locations: [
            "Sanctuary Cave": HeartPiece @Heart(CaveLight 5[2]),
        ],
    },
}
