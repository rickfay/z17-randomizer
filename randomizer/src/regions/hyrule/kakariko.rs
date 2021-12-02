crate::region! {
    course: FieldLight,
    name: "Kakariko Village",
    village {
        locations: [
            "Well (Chest)": RupeeR @Chest(CaveLight 4[6]),
            "Well (Upper)": HeartPiece @Heart(CaveLight 4[8]),
            "Jail": RupeeSilver @Chest(IndoorLight 3[3]) :- can_merge,
        ],
        paths: [
            post_sanc :- did_sanctuary,
        ],
    },
    post_sanc {
        locations: [
            "Merchant (Left)": ItemBottle @Shop(Merchant(0)),
            "Bee Guy": ItemInsectNet @Event(IndoorLight/FieldLight_18_InsectNet[0xB]) :- has_bottle,
            "Bee Guy (Golden Bee)": BadgeBee @Event(IndoorLight/FieldLight_18_InsectNet[0x1F])
                :- {|s| {
                    let can_catch = if s.settings().logic.require_golden_bee_for_sale {
                        false
                    } else {
                        s.can_insect_net()
                    };
                    s.has_bottle() && (can_catch || s.lorule())
                }},
            "Fortune Teller": HintGlasses @Event(IndoorLight/FieldLight_11_FortuneGirl[4]),
            "Milk Bar Owner": MilkMatured @Event(IndoorLight/FieldLight_18_MilkbarMaster[5]) :- has_message,
            "Cucco Ranch": HeartPiece @Event(FieldLight_29_Kokko[0x67]),
        ],
    },
    shady_guy {
        locations: [
            "Shady Guy": DashBoots @Event(FieldLight_18_Touzoku[0x12]),
            "Merchant (Right)": ItemStoneBeauty @Shop(Merchant(2)),
        ],
        paths: [
            lake::hotfoot,
        ],
    },
    closed {
        locations: [
            "Stylish Woman": HeartPiece @Event(IndoorLight/FieldLight_18_ClosedHouse[4]),
        ],
    },
}
