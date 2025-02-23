use game::{
    self,
    Course::{self, *},
};

#[derive(Eq, PartialEq, Hash)]
pub(crate) struct HintGhost<'hg> {
    pub(crate) course: Course,
    pub(crate) msbt_file: &'hg str,
    pub(crate) msg_label: &'hg str,
}

macro_rules! hint_ghost_from {
    (
        $(#[$attr:meta])*
        $($ghost:ident => $course:ident, $msbt_file:literal, $msg_label:literal;)+
    ) => {
        $(#[$attr])*
        impl<'hg> From<::game::ghosts::HintGhost> for HintGhost<'hg> {
            fn from(value: ::game::ghosts::HintGhost) -> Self {
                match value {
                    $(::game::ghosts::HintGhost::$ghost => Self { course: $course, msbt_file: $msbt_file, msg_label: $msg_label },)+
                }
            }
        }
    }
}

hint_ghost_from! {
    // Lost Woods Maze (3)
    LostWoodsMaze1 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg4";
    LostWoodsMaze2 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg8";
    LostWoodsMaze3 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg6";

    // Hyrule (33)
    LostWoods => FieldLight, "HintGhostLight", "HintGhost_FieldLight_00_000";
    SpectacleRock => FieldLight, "HintGhostLight", "HintGhost_FieldLight_03_002";
    TowerOfHeraOutside => FieldLight, "HintGhostLight", "HintGhost_FieldLight_03_003";
    FloatingIsland => FieldLight, "HintGhostLight", "HintGhost_FieldLight_05_004";
    FireCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_05_005";
    MoldormCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_0A_006";
    ZorasDomain => FieldLight, "HintGhostLight", "HintGhost_FieldLight_0F_007";
    FortuneTellerHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_11_008";
    Sanctuary => FieldLight, "HintGhostLight", "HintGhost_FieldLight_13_009";
    GraveyardHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_010";
    WaterfallCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_17_011";
    Well => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_012";
    ShadyGuy => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_013";
    StylishWoman => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_014";
    BlacksmithCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_22_015";
    EasternRuinsPegs => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_016";
    EasternRuinsCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_017";
    EasternRuinsEntrance => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_018";
    RupeeRushHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_28_019";
    Cuccos => FieldLight, "HintGhostLight", "HintGhost_FieldLight_29_020";
    SouthBridge => FieldLight, "HintGhostLight", "HintGhost_FieldLight_2D_021";
    SouthernRuins => FieldLight, "HintGhostLight", "HintGhost_FieldLight_33_022";
    HouseOfGalesIsland => FieldLight, "HintGhostLight", "HintGhost_FieldLight_35_023";
    HyruleHotfoot => FieldLight, "HintGhostLight", "HintGhost_FieldLight_37_024";
    Letter => FieldLight, "HintGhostLight", "HintGhost_FieldLight_37_025";
    StreetPassTree => FieldLight, "HintGhostLight", "HintGhost_FieldLight_2B_026";
    BlacksmithBehind => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1A_027";
    GraveyardLedge => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_028";
    DesertEast => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_029";
    DesertCenter => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_030";
    DesertSouthWest => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_031";
    HyruleCastleRocks => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_032";
    WitchsHouse => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_033";

    // Lorule (22)
    SkullWoodsCuccos => FieldDark, "HintGhostDark", "HintGhost_FieldDark_02_001";
    TreacherousTower => FieldDark, "HintGhostDark", "HintGhost_FieldDark_03_002";
    IceRuinsOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_07_003";
    LoruleGraveyard => FieldDark, "HintGhostDark", "HintGhost_FieldDark_14_004";
    DarkRuinsNorth => FieldDark, "HintGhostDark", "HintGhost_FieldDark_16_005";
    SkullWoodsSouth => FieldDark, "HintGhostDark", "HintGhost_FieldDark_18_006";
    FortunesChoice => IndoorDark, "HintGhostDark", "HintGhost_FieldDark_18_007"; // Indoors
    VeteranThief => IndoorDark, "HintGhostDark", "HintGhost_FieldDark_18_008"; // Indoors
    FortuneTellerLorule => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1A_009";
    DarkMaze => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1E_010";
    RupeeRushLorule => FieldDark, "HintGhostDark", "HintGhost_FieldDark_28_011";
    GreatRupeeFairy => FieldDark, "HintGhostDark", "HintGhost_FieldDark_29_012";
    OctoballDerby => FieldDark, "HintGhostDark", "HintGhost_FieldDark_2A_013";
    VacantHouse => FieldDark, "HintGhostDark", "HintGhost_FieldDark_2C_014";
    MiseryMireLedge => FieldDark, "HintGhostDark", "HintGhost_FieldDark_30_015";
    SwampPalaceOutsideLeft => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_016";
    TurtleBullied => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_017";
    TurtleWall => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_018";
    TurtleRockOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_019";
    DarkPalaceOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1E_020";
    SwampPalaceOutsideRight => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_021";
    MiseryMireBridge => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_022";
}
