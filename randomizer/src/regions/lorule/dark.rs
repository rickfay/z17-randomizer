crate::region! {
    course: FieldDark,
    name: "Dark Ruins",
    ruins {
        locations: [
            "Lake Chest": RupeeSilver @Chest(35[228]),
            "Maze Chest": RupeeR @Chest(20[79]),
            "Maze Ledge": HeartPiece @Heart(20[172]),
            "Hinox (1)": RupeeB @Event(CaveDark/FieldDark_17_NpcHinox [0x02]),
            "Hinox (2)": RupeeR @Event(CaveDark/FieldDark_17_NpcHinox [0x06]),
            "Hinox (3)": RupeePurple @Event(CaveDark/FieldDark_17_NpcHinox [0x4C]),
            "Hinox (4)": RupeeSilver @Event(CaveDark/FieldDark_17_NpcHinox [0x0A]),
            "Hinox (5)": RupeeSilver @Event(CaveDark/FieldDark_17_NpcHinox [0x11]),
            "Hinox (6)": RupeeGold @Event(CaveDark/FieldDark_17_NpcHinox [0x16]),
        ],
        paths: [
            dungeons::dark::palace :- {|s| s.can_bomb() && s.can_see_in_dark()},
        ],
    },
}
