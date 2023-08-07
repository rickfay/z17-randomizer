use game::Course::{self, *};

use crate::FillerItem::{self, *};

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
        impl<'hg> From<FillerItem> for HintGhost<'hg> {
            fn from(value: FillerItem) -> Self {
                match value {
                    $($ghost => Self { course: $course, msbt_file: $msbt_file, msg_label: $msg_label },)+
                    _ => macros::fail!("\"{:?}\" is not a Hint Ghost", value),
                }
            }
        }
    }
}

hint_ghost_from! {
    // Lost Woods
    HintGhostLostWoodsMaze1 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg4";
    HintGhostLostWoodsMaze2 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg8";
    HintGhostLostWoodsMaze3 => FieldLight, "FieldLight_00", "lgt_MayoinoHintObake_Msg6";

    // Hyrule
    HintGhostLostWoods => FieldLight, "HintGhostLight", "HintGhost_FieldLight_00_000";
    HintGhostSpectacleRock => FieldLight, "HintGhostLight", "HintGhost_FieldLight_03_002";
    HintGhostTowerOfHeraOutside => FieldLight, "HintGhostLight", "HintGhost_FieldLight_03_003";
    HintGhostFloatingIsland => FieldLight, "HintGhostLight", "HintGhost_FieldLight_05_004";
    HintGhostFireCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_05_005";
    HintGhostMoldormCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_0A_006";
    HintGhostZorasDomain => FieldLight, "HintGhostLight", "HintGhost_FieldLight_0F_007";
    HintGhostFortuneTellerHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_11_008";
    HintGhostSanctuary => FieldLight, "HintGhostLight", "HintGhost_FieldLight_13_009";
    HintGhostGraveyardHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_010";
    HintGhostWaterfallCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_17_011";
    HintGhostWell => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_012";
    HintGhostShadyGuy => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_013";
    HintGhostStylishWoman => FieldLight, "HintGhostLight", "HintGhost_FieldLight_18_014";
    HintGhostBlacksmithCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_22_015";
    HintGhostEasternRuinsPegs => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_016";
    HintGhostEasternRuinsCave => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_017";
    HintGhostEasternRuinsEntrance => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1E_018";
    HintGhostRupeeRushHyrule => FieldLight, "HintGhostLight", "HintGhost_FieldLight_28_019";
    HintGhostCuccos => FieldLight, "HintGhostLight", "HintGhost_FieldLight_29_020";
    HintGhostSouthBridge => FieldLight, "HintGhostLight", "HintGhost_FieldLight_2D_021";
    HintGhostSouthernRuins => FieldLight, "HintGhostLight", "HintGhost_FieldLight_33_022";
    HintGhostHouseOfGalesIsland => FieldLight, "HintGhostLight", "HintGhost_FieldLight_35_023";
    HintGhostHyruleHotfoot => FieldLight, "HintGhostLight", "HintGhost_FieldLight_37_024";
    HintGhostLetter => FieldLight, "HintGhostLight", "HintGhost_FieldLight_37_025";
    HintGhostStreetPassTree => FieldLight, "HintGhostLight", "HintGhost_FieldLight_2B_026";
    HintGhostBlacksmithBehind => FieldLight, "HintGhostLight", "HintGhost_FieldLight_1A_027";
    HintGhostGraveyardLedge => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_028";
    HintGhostDesertEast => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_029";
    HintGhostDesertCenter => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_030";
    HintGhostDesertSouthWest => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_031";
    HintGhostHyruleCastleRocks => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_032";
    HintGhostWitchsHouse => FieldLight, "HintGhostLight", "HintGhost_FieldLight_14_033";

    // Lorule
    HintGhostSkullWoodsCuccos => FieldDark, "HintGhostDark", "HintGhost_FieldDark_02_001";
    HintGhostTreacherousTower => FieldDark, "HintGhostDark", "HintGhost_FieldDark_03_002";
    HintGhostIceRuinsOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_07_003";
    HintGhostLoruleGraveyard => FieldDark, "HintGhostDark", "HintGhost_FieldDark_14_004";
    HintGhostDarkRuinsNorth => FieldDark, "HintGhostDark", "HintGhost_FieldDark_16_005";
    HintGhostSkullWoodsSouth => FieldDark, "HintGhostDark", "HintGhost_FieldDark_18_006";
    HintGhostFortunesChoice => IndoorDark, "HintGhostDark", "HintGhost_FieldDark_18_007";
    HintGhostVeteranThief => IndoorDark, "HintGhostDark", "HintGhost_FieldDark_18_008";
    HintGhostFortuneTellerLorule => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1A_009";
    HintGhostDarkMaze => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1E_010";
    HintGhostRupeeRushLorule => FieldDark, "HintGhostDark", "HintGhost_FieldDark_28_011";
    HintGhostGreatRupeeFairy => FieldDark, "HintGhostDark", "HintGhost_FieldDark_29_012";
    HintGhostOctoballDerby => FieldDark, "HintGhostDark", "HintGhost_FieldDark_2A_013";
    HintGhostVacantHouse => FieldDark, "HintGhostDark", "HintGhost_FieldDark_2C_014";
    HintGhostMiseryMireLedge => FieldDark, "HintGhostDark", "HintGhost_FieldDark_30_015";
    HintGhostSwampPalaceOutsideLeft => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_016";
    HintGhostTurtleBullied => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_017";
    HintGhostTurtleWall => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_018";
    HintGhostTurtleRockOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_35_019";
    HintGhostDarkPalaceOutside => FieldDark, "HintGhostDark", "HintGhost_FieldDark_1E_020";
    HintGhostSwampPalaceOutsideRight => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_021";
    HintGhostMiseryMireBridge => FieldDark, "HintGhostDark", "HintGhost_FieldDark_33_022";
}
