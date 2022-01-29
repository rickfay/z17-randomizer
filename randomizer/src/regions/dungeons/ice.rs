crate::region! {
    course: DungeonIce,
    name: "Ice Ruins",
    ruins {
        locations: [
            "(1F) Hidden Chest": RupeeGold @Chest(1[1048]),
            "(B3) Grate Chest (Left)": RupeeG @Chest(1[840]),
            "(B3) Grate Chest (Right)": LiverYellow @Chest(1[893]),
            "(B4) Ice Pillar": KeySmall @Key(1[1057]),
            "(B5) Big Chest": KeyBoss @Chest(1[282]),
        ],
        paths: [
            basement1,
        ],
    },
    basement1 {
        locations: [
            "(B1) East Chest": Compass @Chest(1[108]) :- {|p| (p.small_keys(COURSE) >= 1) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
            "(B1) Narrow Ledge": KeySmall @Key(1[98]) :- {|p| (p.small_keys(COURSE) >= 1) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
            "(B1) Upper Chest": RupeeB @Chest(1[1026]) :- {|p| (p.small_keys(COURSE) >= 2) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
        ],
        paths: [
            basement2 :- {|p| (p.small_keys(COURSE) >= 2) || (p.glitched() && p.has_boots())},
        ],
    },
    basement2 {
        locations: [
            "(B3) Big Chest (Puzzle)": GanbariPowerUp @Chest(1[18]),
            "(B4) Switches": RupeeSilver @Chest(1[25]),
            "(B4) Southwest Chest (Fall)": RupeePurple @Chest(1[1122]),
            "(B4) Narrow Platform": LiverPurple @Chest(1[913]),
            "(B2) Far North": RupeeSilver @Chest(1[838]) :- {|p| p.has_stamina_scroll() || (p.glitched() && p.has_boots())},
            "(B4) Southeast Chest (Fall)": KeySmall @Chest(1[273]),
        ],
        paths: [
            boss :- {|p| (p.small_keys(COURSE) >= 3 && p.has_boss_key(COURSE)) || (p.glitched() && p.has_boots())},
        ],
    },
    boss {
        locations: [
            "Dharkstare": HeartContainer @Heart(1[554]),
        ],
        quest: Portrait::Rosso,
    },
}
