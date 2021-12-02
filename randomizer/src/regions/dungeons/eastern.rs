crate::region! {
    course: DungeonEast,
    name: "Eastern Palace",
    palace {
        locations: [
            "(1F) Outside (East)": RupeeSilver @Chest(1[244]) :- can_merge,
            "(1F) Near Entrance": RupeeR @Chest(1[132]) :- has_ranged_attack,
        ],
        paths: [
            floor1 :- {|p| p.has_ranged_attack() || p.can_merge()},
        ],
    },
    floor1 {
        locations: [
            "(1F) Defeat Popos": Compass @Chest(1[61]) :- can_damage,
            "(1F) Hidden Door": RupeeR @Chest(1[142]) :- can_damage,
            "(1F) Switch Puzzle": KeySmall @Chest(1[74]) :- can_use_projectile,
        ],
        paths: [
            floor2 :- {|s| s.small_keys(COURSE) > 0 && s.can_damage()},
        ],
    },
    floor2 {
        locations: [
            "(2F) Ball Room": LiverPurple @Chest(2[147]),
            "(2F) Defeat Popos": LiverPurple @Chest(2[115]),
            "(2F) Switch Room": KeySmall @Chest(2[52]) :- has_ranged_attack,
        ],
        paths: [
            boss_key :- {|p| p.has_ranged_attack() && p.small_keys(COURSE) > 1},
        ],
    },
    boss_key {
        locations: [
            "(2F) Big Chest": KeyBoss @Chest(2[44]),
        ],
        paths: [
            boss :- {|p| p.has_boss_key(COURSE) && p.can_use_projectile()},
        ],
    },
    boss {
        locations: [
            "(3F) After Cutscene": RingHekiga @Event(East[0x1C]),
        ],
        paths: [
            post_boss :- can_merge,
        ],
    },
    post_boss {
        locations: [
            "Yuga": HeartContainer @Heart(3[94]),
            "(3F) Outside (North)": RupeeSilver @Chest(3[25]),
            "(1F) Outside (West)": RupeePurple @Chest(1[235]),
        ],
        quest: Pendant::Courage,
    },
}
