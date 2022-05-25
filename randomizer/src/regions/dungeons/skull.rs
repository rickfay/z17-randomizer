crate::region! {
    course: DungeonDokuro,
    name: "Skull Woods",
    palace {
        locations: [
            "(B1) Gibdo Room (Lower)": Compass @Chest(1[100]),
            "(B1) South Chest": KeySmall @Chest(1[101]),
        ],
        paths: [
            outdoors :- {|s| s.small_keys(COURSE) >= 1},
        ],
    },
    outdoors {
        locations: [
            "(B1) Gibdo Room (Hole)": RupeeSilver @Chest(1[640]),
            "(B1) Grate Room": KeySmall @Chest(1[328]),
        ],
        paths: [
            basement2 :- {|s| s.small_keys(COURSE) >= 2},
        ],
    },
    basement2 {
        locations: [
            "(B2) Moving Platform Room": KeySmall @Chest(2[105]),
        ],
        paths: [
            end :- {|s| s.small_keys(COURSE) >= 3 && s.can_light()},
            boss :- {|s| s.small_keys(COURSE) >= 3 && s.has_boss_key(COURSE)},
        ],
    },
    end {
        locations: [
            "(B1) Big Chest (Upper)": OreGreen @Chest(1[653]),
            "(B1) Big Chest (Eyes)": KeyBoss @Chest(1[289]),
        ],
        paths: [
            lorule::skull::chest,
        ],
    },
    boss {
        locations: [
            "Knucklemaster": HeartContainer @Heart(2[404]),
        ],
        quest: Portrait::Seres,
    },
}
