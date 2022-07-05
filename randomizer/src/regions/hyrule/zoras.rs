crate::region! {
    course: FieldLight,
    name: "Zoras Domain",
    domain {
        locations: [
            "Zora's Domain Ledge Chest": RupeeR @Chest(15[35]) :- can_merge,
            "Behind Waterfall": HeartPiece @Heart(CaveLight 13[103]) :- {|p| p.can_swim() || (p.glitched() && (p.can_merge() || p.fake_flippers()))},
            "Zora Queen": ItemMizukaki @Event(CaveLight/FieldLight_0F_Zora[0x6B]) :- {|p| p.can_merge() && p.has_smooth_gem()},
        ],
        paths: [
            kakariko::shady_guy :- can_merge,
        ],
    },
}
