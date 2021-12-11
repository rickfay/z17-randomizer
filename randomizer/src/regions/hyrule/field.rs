crate::region! {
    course: FieldLight,
    name: "Hyrule Field",
    main {
        locations: [
            "Delivery": PackageSword @None() where settings: !settings.items.captains_sword.is_skipped(),
            "Dampe": ItemSwordLv1 @Event(FieldLight_13_Sister[0x1D]),
            "Rosso Cave": RupeeR @Chest(CaveLight 6[6]) :- can_hammer,
            "Sanctuary Pegs": RupeeSilver @Chest(11[89]) :- can_hammer,
            "Treasure Room": RupeeSilver @Chest(AttractionLight 5[24])
                :- {|p| p.can_bomb() && p.can_merge()},
            "Behind Blacksmith": HeartPiece @Heart(17[95]) :- can_merge,
            "Blacksmith Cave": HeartPiece @Heart(CaveLight 16[1]) :- can_lift_big,
            "Blacksmith": ItemSwordLv3 @Event(IndoorLight/FieldLight_22_BlackSmith[0x16])
                :- {|p| p.lorule() && p.ore() >= 2},
            "Castle Rocks": HeartPiece @Heart(18[209]) :- can_lift,
        ],
        paths: [
            sanctuary::lobby :- {|p| p.sword() || p.can_lift() || p.can_fire_rod() || p.can_ice_rod() || p.can_bomb() || p.can_lamp() || p.has_boots()},
            lost::woods,
            death::mountain, // :- can_lift,
            zoras::domain :- can_merge,
            kakariko::village,
            eastern::ruins,
            southern::ruins,
            lake::hylia,
            post_sanc :- did_sanctuary,
            post_eastern :- did_eastern,
            castle :- {|s| s.is_barrier_up() && s.has_master_sword()},
            lorule::field::main :- lorule,
            lorule::graveyard::field :- lorule,
            lorule::field::ledge :- lorule,
        ],
    },
    post_sanc {
        locations: [
            "Thanks": RingRental @Event(IndoorLight/FieldLight_2C_Rental[0xED])
                where settings: !settings.items.first_bracelet.is_skipped(),
            "Ravio (5)": ItemRentalBow @Shop(Ravio(8)),
        ],
        paths: [
            kakariko::post_sanc,
        ],
    },
    post_eastern {
        locations: [
            "Rosso": PowerGlove @Chest(IndoorLight 10[7]),
            "Clean Rocks": RupeePurple @Chest(IndoorLight 10[25]) :- can_lift,
            "Irene": ItemBell @Event[
                FieldLight_11_Maple[0x06],
                FieldLight_12_Maple[0x08],
                FieldLight_12_Maple[0x26],
                FieldLight_2D_Maple[0x07],
            ],
            "Woods": Pouch @Event(FieldLight_2A_BlacksmithWife[0x15]),
        ],
    },
    rentals {
        locations: [
            "Ravio (1)": ItemRentalIceRod @Shop(Ravio(0)),
            "Ravio (2)": ItemRentalHookShot @Shop(Ravio(5)),
            "Ravio (3)": ItemRentalTornadeRod @Shop(Ravio(2)),
            "Ravio (4)": ItemRentalBomb @Shop(Ravio(3)),
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
        paths: [
            castle_top :- can_merge,
        ],
    },
    castle_top {
        quest: Lorule,
    },
    sanctuary_cave {
        locations: [
            "Sanctuary Cave": HeartPiece @Heart(CaveLight 5[2]),
        ],
    },
}
