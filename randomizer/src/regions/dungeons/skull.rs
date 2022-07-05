crate::region! {
    course: DungeonDokuro,
    name: "Skull Woods",
    palace {
        locations: [
            "[SW] (B1) Gibdo Room (Lower)": Compass @Chest(1[100]),
            "[SW] (B1) South Chest": KeySmall @Chest(1[101]),
        ],
        paths: [
            outdoors :- {|s| s.small_keys(COURSE) >= 1},
        ],
    },
    outdoors {
        locations: [
            "[SW] (B1) Gibdo Room (Hole)": RupeeSilver @Chest(1[640]),
            "[SW] (B1) Grate Room": KeySmall @Chest(1[328]),
        ],
        paths: [
            basement2 :- {|s| s.small_keys(COURSE) >= 2},
        ],
    },
    basement2 {
        locations: [
            "[SW] (B2) Moving Platform Room": KeySmall @Chest(2[105]),
        ],
        paths: [
            end :- {|s| s.small_keys(COURSE) >= 3 && s.can_light()},
            boss :- {|s| s.small_keys(COURSE) >= 3 && s.has_boss_key(COURSE)},
        ],
    },
    end {
        locations: [
            "[SW] (B1) Big Chest (Upper)": OreGreen @Chest(1[653]),
            "[SW] (B1) Big Chest (Eyes)": KeyBoss @Chest(1[289]),
        ],
        paths: [
            lorule::skull::chest,
        ],
    },
    boss {
        locations: [
            "[SW] Knucklemaster": HeartContainer @Heart(2[404]),
        ],
        quest: Portrait::Seres,
    },
}
