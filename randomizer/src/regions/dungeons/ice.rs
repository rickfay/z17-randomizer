crate::region! {
    course: DungeonIce,
    name: "Ice Ruins",
    ruins {
        locations: [
            "[IR] (1F) Hidden Chest": RupeeGold @Chest(1[1048]),
            "[IR] (B3) Grate Chest (Left)": RupeeG @Chest(1[840]),
            "[IR] (B3) Grate Chest (Right)": LiverYellow @Chest(1[893]),
            "[IR] (B4) Ice Pillar": KeySmall @Key(1[1057]),
            "[IR] (B5) Big Chest": KeyBoss @Chest(1[282]),
        ],
        paths: [
            basement1,
        ],
    },
    basement1 {
        locations: [
            "[IR] (B1) East Chest": Compass @Chest(1[108]) :- {|p| (p.small_keys(COURSE) >= 1) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
            "[IR] (B1) Narrow Ledge": KeySmall @Key(1[98]) :- {|p| (p.small_keys(COURSE) >= 1) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
            "[IR] (B1) Upper Chest": RupeeB @Chest(1[1026]) :- {|p| (p.small_keys(COURSE) >= 2) || (p.glitched() && p.has_boots() && p.can_tornado_rod())},
        ],
        paths: [
            basement2 :- {|p| (p.small_keys(COURSE) >= 2) || (p.glitched() && p.has_boots())},
        ],
    },
    basement2 {
        locations: [
            "[IR] (B3) Big Chest (Puzzle)": GanbariPowerUp @Chest(1[18]),
            "[IR] (B4) Switches": RupeeSilver @Chest(1[25]),
            "[IR] (B4) Southwest Chest (Fall)": RupeePurple @Chest(1[1122]),
            "[IR] (B4) Narrow Platform": LiverPurple @Chest(1[913]),
            "[IR] (B2) Far North": RupeeSilver @Chest(1[838]) :- {|p| p.has_stamina_scroll() || (p.glitched() && p.has_boots())},
            "[IR] (B4) Southeast Chest (Fall)": KeySmall @Chest(1[273]),
        ],
        paths: [
            boss :- {|p| (p.small_keys(COURSE) >= 3 && p.has_boss_key(COURSE)) || (p.glitched() && p.has_boots())},
        ],
    },
    boss {
        locations: [
            "[IR] Dharkstare": HeartContainer @Heart(1[554]),
        ],
        quest: Portrait::Rosso,
    },
}
