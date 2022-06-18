crate::region! {
    course: FieldDark,
    name: "Lorule Field",
    main {
        locations: [
            "Boots Treasure Dungeon": RupeeGold @Chest(AttractionDark 1[12]) :- {|p| p.has_boots() || (p.glitched() && (p.has_master_sword() || p.can_bomb() || p.can_boomerang()))},
            "Vacant House": ItemBottle @Chest(IndoorDark 11[31]),
            "Rupee Rush (Lorule)": HeartPiece @Event(FieldDark_28_Minigame[0x26]),
            "Great Rupee Fairy": ItemBottle @Event(CaveDark/Cave[0x24]),
            "Big Bomb Cave": RupeeSilver @Chest(CaveDark 2[4]),
            "Octoball Derby": HeartPiece @Event(FieldDark_2A_GameMaster[0x21]),
            "Blacksmith (Lorule)": ItemSwordLv4 @Event(IndoorDark/FiledDark_22_BlackSmithUra[0x1F]) :- {|p| p.ore() >= 4},
        ],
        paths: [
            village,
            swamp,
            dungeons::castle::lorule :- has_seven_portraits,
        ],
    },
    village {
        paths: [
            dungeons::thieves::hideout,
            hyrule::kakariko::closed :- can_merge,
        ],
    },
    swamp {
        locations: [
            "Swamp Cave (Left)": RupeeSilver @Chest(CaveDark 3[3]),
            "Swamp Cave (Middle)": HeartPiece @Heart(CaveDark 3[8]),
            "Swamp Cave (Right)": RupeeGold @Chest(CaveDark 3[6]),
        ],
        paths: [
            dungeons::swamp::palace,
        ],
    },
    thief_girl {
        locations: [
            "Thief Girl Cave": RupeeSilver @Chest(CaveDark 15[9]),
        ],
    },
    ledge {
        locations: [
            "Hookshot Ledge": RupeeSilver @Chest(32[95]) :- can_hookshot,
        ],
    },
}
