crate::region! {
    course: DungeonHagure,
    name: "Thieves Hideout",
    hideout {
        locations: [
            "(B1) Jail Cell": RupeePurple @Chest(1[1323]) :- can_merge,
            "(B1) Grate Chest": RupeePurple @Chest(1[576]),
            "(B2) Grate Chest (Fall)": RupeePurple @Chest(1[1292]),
        ],
        paths: [
            basement2 :- can_merge,
        ],
    },
    basement2 {
        locations: [
            "(B2) Switch Puzzle Room": LiverPurple @Chest(1[949]),
            "(B2) Jail Cell": Compass @Chest(1[283]),
            "(B2) Eyegores": KeySmall @Chest(1[543]),
        ],
        paths: [
            escape :- {|p| p.small_keys(COURSE) > 0 && p.can_swim()},
        ],
    },
    escape {
        locations: [
            "(B1) Behind Wall": RupeeSilver @Chest(1[1359]),
            "(B1) Big Chest (Entrance)": KeyBoss @Chest(1[580]),
            "(B3) Underwater": RupeeSilver @Chest(1[667]),
            "(B3) Big Chest (Hidden)": OreBlue @Chest(1[657]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE) && p.can_merge()},
        ],
    },
    boss {
        locations: [
            "Stalblind": HeartContainer @Heart(IndoorDark 15[12]),
        ],
        paths: [
            lorule::field::thief_girl,
        ],
        quest: Portrait::Osfala,
    },
}
