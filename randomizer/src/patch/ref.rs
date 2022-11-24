use albw::course::Id;
use albw::course::Id::*;
use albw::string_struct;


#[allow(non_camel_case_types)]
pub struct MsbfInfo {
    course: Id,
    msbf: Option<&'static str>,
}

impl MsbfInfo {
    pub const EP: Self = Self { msbf: None, course: DungeonEast };
    pub const HG: Self = Self { msbf: None, course: DungeonWind };
    pub const TH: Self = Self { msbf: None, course: DungeonHera };
    pub const PD: Self = Self { msbf: Some(MsbfKey::Dark), course: DungeonDark };
    pub const SP: Self = Self { msbf: Some(MsbfKey::Water), course: DungeonWater };
    pub const SW: Self = Self { msbf: Some(MsbfKey::Dokuro), course: FieldDark };
    pub const TT: Self = Self { msbf: Some(MsbfKey::Hagure), course: IndoorDark };
    pub const TR: Self = Self { msbf: None, course: DungeonKame };
    pub const DP: Self = Self { msbf: Some(MsbfKey::Sand), course: FieldDark };
    pub const IR: Self = Self { msbf: Some(MsbfKey::Ice), course: DungeonIce };

    pub fn get_course(&self) -> Id {
        self.course
    }

    pub fn has_msbf(&self) -> bool {
        self.msbf.is_some()
    }

    pub fn get_msbf(&self) -> Option<&'static str> {
        self.msbf
    }

    pub fn get_path(&self) -> String {
        format!("World/Flow/{}.msbf", self.msbf.unwrap())
    }
}

string_struct! {
    #[allow(non_upper_case_globals)]
    MsbfKey {
        Castle,
        CatchInsect,
        Cave,
        CaveDark10,
        cl_Church_UG,
        CrossBattle,
        CrossBoard,
        CrossForceTalk,
        CrossOldMan,
        Dark,
        Dokuro,
        DoorHouse,
        E3_flow,
        East,
        Ending,
        FieldDark_00_GoldenBeeShop,
        FieldDark_05_GameTower,
        FieldDark_0F_Namazu,
        FieldDark_13_Sinpu,
        FieldDark_14_Danpei,
        FieldDark_16_HagureHouse,
        FieldDark_16_MagicShop,
        FieldDark_17_NpcHinox,
        FieldDark_18_BakudanTouzoku,
        FieldDark_18_BoxManDark,
        FieldDark_18_ItemShop,
        FieldDark_1A_FortuneGirlUra,
        FieldDark_1B_Bakudanya,
        FieldDark_1B_Hilda,
        FieldDark_1E_Sennyukun,
        FieldDark_28_Minigame,
        FieldDark_29_BakudanShop,
        FieldDark_29_HappyFairy,
        FieldDark_2A_GameMaster,
        FieldDark_2C_RaviosDiary,
        FieldDark_33_Daibakudankabe,
        FieldDark_33_Touzoku,
        FieldDark_35_ItemShop,
        FieldDark_35_Kame,
        FieldDark_3A_CrazyMan,
        FieldDark_Tennokoe,
        FieldLight_00_JyohoShop,
        FieldLight_00_Mayoinomori,
        FieldLight_02_KikoriMan,
        FieldLight_03_Kanban,
        FieldLight_05_Climber,
        FieldLight_0A_Kanban,
        FieldLight_0F_Kanban,
        FieldLight_0F_Zora,
        FieldLight_11_FortuneGirl,
        FieldLight_11_Maple,
        FieldLight_12_Maple,
        FieldLight_12_SignBoard,
        FieldLight_13_Danpei,
        FieldLight_13_Medium,
        FieldLight_13_SignBoard,
        FieldLight_13_Sinpu,
        FieldLight_13_Sister,
        FieldLight_14_Danpei,
        FieldLight_14_Maple,
        FieldLight_16_Ending,
        FieldLight_16_MagicShop,
        FieldLight_16_Obaba,
        FieldLight_16_SignBoard,
        FieldLight_17_Kanban,
        FieldLight_18_Bard,
        FieldLight_18_BoxMan,
        FieldLight_18_ClosedHouse,
        FieldLight_18_InsectNet,
        FieldLight_18_ItemShop,
        FieldLight_18_Kakarikoboy,
        FieldLight_18_KakarikoGirl,
        FieldLight_18_MaidSahasulala,
        FieldLight_18_MiddleLady,
        FieldLight_18_MiddleMan,
        FieldLight_18_MilkbarMaster,
        FieldLight_18_MilkbarSoldier,
        FieldLight_18_Rotenshonin,
        FieldLight_18_SahasPupil,
        FieldLight_18_SignBoard,
        FieldLight_18_Soldier,
        FieldLight_18_StandItem,
        FieldLight_18_Touzoku,
        FieldLight_1A_Maple,
        FieldLight_1A_SignBoard,
        FieldLight_1B_BlackSmithKid,
        FieldLight_1B_Commander,
        FieldLight_1B_Hekiga,
        FieldLight_1B_Impa,
        FieldLight_1B_Rakcha,
        FieldLight_1B_Sahasrahla,
        FieldLight_1B_Soldier,
        FieldLight_1B_Zelda,
        FieldLight_1E_Sahasrahla,
        FieldLight_22_BlackSmith,
        FieldLight_22_BlackSmithKid,
        FieldLight_22_BlackSmithWife,
        FieldLight_22_Dwarf,
        FieldLight_22_Maple,
        FieldLight_28_Minigame,
        FieldLight_29_Kokko,
        FieldLight_2A_BlacksmithKid,
        FieldLight_2A_BlacksmithWife,
        FieldLight_2B_AppleTree,
        FieldLight_2B_BlackSmithKid,
        FieldLight_2B_Maple,
        FieldLight_2C_BlackSmithKid,
        FieldLight_2C_GanbariTutorial,
        FieldLight_2C_Rental,
        FieldLight_2C_RentalItem,
        FieldLight_2C_SahasPupil,
        FieldLight_2C_Sahasrahla,
        FieldLight_2C_SignBoard,
        FieldLight_2C_Soldier,
        FieldLight_2D_Maple,
        FieldLight_2D_UnderBridgeStranger,
        FieldLight_2E_Maple,
        FieldLight_33_Douguya,
        FieldLight_35_Douguya,
        FieldLight_35_ItemShop,
        FieldLight_35_Kinsta,
        FieldLight_35_Marutakun,
        FieldLight_35_Zora,
        FieldLight_37_MessageBottle,
        FieldLight_BlacksmithWife,
        FieldLight_HyruleRace,
        FieldLight_Tennokoe,
        FieldLight_WarpEvent,
        FiledDark_22_BlackSmithUra,
        FiledDark_22_BlackSmithWifeUra,
        GameOver,
        Ganon,
        GirigiriGameTest,
        Hagure,
        Hera,
        HintGhost,
        Ice,
        IndoorDark1_ZoraQueen,
        IndoorDark2_Demo080,
        Kame,
        MessageBoard,
        MiniDungeon_FieldDark_2B,
        MiniDungeon_FieldLight_07,
        MiniDungeon_FieldLight_15,
        MiniDungeon_FieldLight_1E,
        MiniDungeon_FieldLight_32,
        MiniDungeon_FieldLight_33,
        NpcClimberTest,
        NpcHinox,
        NpcShadowLink,
        NpcStand,
        npcTest00,
        NpcTestIwata,
        NpcTownEtc,
        Sand,
        Telephone,
        test,
        ToRentalShopBoard,
        Water,
        Wind,
        yamazaki,
        yamazaki2,
    }
}