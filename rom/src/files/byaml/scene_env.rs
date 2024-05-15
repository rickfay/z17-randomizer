use crate::File;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SceneEnvFile {
    pub scene_env: File<SceneEnv>,
}

impl SceneEnvFile {
    pub fn new(scene_env: File<SceneEnv>) -> Self {
        Self { scene_env }
    }

    pub fn scene_env(&self) -> &File<SceneEnv> {
        &self.scene_env
    }

    pub fn scene_env_mut(&mut self) -> &mut File<SceneEnv> {
        &mut self.scene_env
    }

    pub fn into_file(self) -> File<SceneEnv> {
        self.scene_env
    }

    pub fn dump(self) -> Box<[u8]> {
        self.scene_env.serialize().dump()
    }
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneEnv {
    pub AttractionDark: Vec<SceneEnvScene>,
    pub AttractionDark1: Vec<SceneEnvScene>,
    pub AttractionDark2: Vec<SceneEnvScene>,
    pub AttractionDark3: Vec<SceneEnvScene>,
    pub AttractionLight: Vec<SceneEnvScene>,
    pub AttractionLight1: Vec<SceneEnvScene>,
    pub AttractionLight2: Vec<SceneEnvScene>,
    pub AttractionLight4: Vec<SceneEnvScene>,
    pub CaveDark: Vec<SceneEnvScene>,
    pub CaveDark10: Vec<SceneEnvScene>,
    pub CaveDark11: Vec<SceneEnvScene>,
    pub CaveDark12: Vec<SceneEnvScene>,
    pub CaveDark13: Vec<SceneEnvScene>,
    pub CaveDark14: Vec<SceneEnvScene>,
    pub CaveDark15: Vec<SceneEnvScene>,
    pub CaveDark2: Vec<SceneEnvScene>,
    pub CaveDark5: Vec<SceneEnvScene>,
    pub CaveDark5_SW: Vec<SceneEnvScene>,
    pub CaveDark6: Vec<SceneEnvScene>,
    pub CaveDark7: Vec<SceneEnvScene>,
    pub CaveDark8: Vec<SceneEnvScene>,
    pub CaveDark9: Vec<SceneEnvScene>,
    pub CaveDark_SW: Vec<SceneEnvScene>,
    pub CaveLight: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub CaveLight1: String,
    pub CaveLight10: Vec<SceneEnvScene>,
    pub CaveLight11: Vec<SceneEnvScene>,
    pub CaveLight12: Vec<SceneEnvScene>,
    pub CaveLight13: Vec<SceneEnvScene>,
    pub CaveLight14: Vec<SceneEnvScene>,
    pub CaveLight15: Vec<SceneEnvScene>,
    pub CaveLight16: Vec<SceneEnvScene>,
    pub CaveLight17: Vec<SceneEnvScene>,
    pub CaveLight18: Vec<SceneEnvScene>,
    pub CaveLight19: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub CaveLight2: String,
    pub CaveLight20: Vec<SceneEnvScene>,
    pub CaveLight21: Vec<SceneEnvScene>,
    pub CaveLight24: Vec<SceneEnvScene>,
    pub CaveLight25: Vec<SceneEnvScene>,
    pub CaveLight26: Vec<SceneEnvScene>,
    pub CaveLight27: Vec<SceneEnvScene>,
    pub CaveLight29: Vec<SceneEnvScene>,
    pub CaveLight3: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub CaveLight4: String,
    pub CaveLight6: Vec<SceneEnvScene>,
    pub CaveLight7: Vec<SceneEnvScene>,
    pub CaveLight8: Vec<SceneEnvScene>,
    pub CaveLight9: Vec<SceneEnvScene>,
    pub Common: Vec<SceneEnvScene>,
    pub CrossBattle1: Vec<SceneEnvScene>,
    pub CrossBattle4: Vec<SceneEnvScene>,
    pub CrossBattle5: Vec<SceneEnvScene>,
    pub Demo1: Vec<SceneEnvScene>,
    pub Demo10: Vec<SceneEnvScene>,
    pub Demo2: Vec<SceneEnvScene>,
    pub Demo3: Vec<SceneEnvScene>,
    pub Demo4: Vec<SceneEnvScene>,
    pub Demo5: Vec<SceneEnvScene>,
    pub Demo6: Vec<SceneEnvScene>,
    pub Demo7: Vec<SceneEnvScene>,
    pub Demo8: Vec<SceneEnvScene>,
    pub Demo9: Vec<SceneEnvScene>,
    pub DungeonBoss: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub DungeonBoss1: String,
    pub DungeonBoss1_SW: Vec<SceneEnvScene>,
    pub DungeonCastle: Vec<SceneEnvScene>,
    pub DungeonCastle2: Vec<SceneEnvScene>,
    pub DungeonCastle3: Vec<SceneEnvScene>,
    pub DungeonCastle4: Vec<SceneEnvScene>,
    pub DungeonCastle5: Vec<SceneEnvScene>,
    pub DungeonCastle6: Vec<SceneEnvScene>,
    pub DungeonCastle7: Vec<SceneEnvScene>,
    pub DungeonDark1: Vec<SceneEnvScene>,
    pub DungeonDark2: Vec<SceneEnvScene>,
    pub DungeonDark3: Vec<SceneEnvScene>,
    pub DungeonDokuro: Vec<SceneEnvScene>,
    pub DungeonDokuro1: Vec<SceneEnvScene>,
    pub DungeonDokuro2: Vec<SceneEnvScene>,
    pub DungeonEast: Vec<SceneEnvScene>,
    pub DungeonEast1: Vec<SceneEnvScene>,
    pub DungeonEast2: Vec<SceneEnvScene>,
    pub DungeonEast3: Vec<SceneEnvScene>,
    pub DungeonEast_SW: Vec<SceneEnvScene>,
    pub DungeonGanon: Vec<SceneEnvScene>,
    pub DungeonGanon1: Vec<SceneEnvScene>,
    pub DungeonGanon5: Vec<SceneEnvScene>,
    pub DungeonHagure: Vec<SceneEnvScene>,
    pub DungeonHagure1: Vec<SceneEnvScene>,
    pub DungeonHera: Vec<SceneEnvScene>,
    pub DungeonHera1: Vec<SceneEnvScene>,
    pub DungeonIce: Vec<SceneEnvScene>,
    pub DungeonKame: Vec<SceneEnvScene>,
    pub DungeonSand: Vec<SceneEnvScene>,
    pub DungeonWater: Vec<SceneEnvScene>,
    pub DungeonWater1: Vec<SceneEnvScene>,
    pub DungeonWind: Vec<SceneEnvScene>,
    pub DungeonWind2: Vec<SceneEnvScene>,
    pub E3Field10: Vec<SceneEnvScene>,
    pub EnemyAttack: Vec<SceneEnvScene>,
    pub EnemyAttackL: Vec<SceneEnvScene>,
    pub EnemyAttackL32: Vec<SceneEnvScene>,
    pub EnemyAttackL34: Vec<SceneEnvScene>,
    pub EnemyAttackL37: Vec<SceneEnvScene>,
    pub EnemyAttackL39: Vec<SceneEnvScene>,
    pub EnemyAttackM: Vec<SceneEnvScene>,
    pub EnemyAttackS: Vec<SceneEnvScene>,
    pub FieldDark: Vec<SceneEnvScene>,
    pub FieldDark1: Vec<SceneEnvScene>,
    pub FieldDark1_SW: Vec<SceneEnvScene>,
    pub FieldDark20: Vec<SceneEnvScene>,
    pub FieldDark3: Vec<SceneEnvScene>,
    pub FieldDark38: Vec<SceneEnvScene>,
    pub FieldDark3_SW: Vec<SceneEnvScene>,
    pub FieldDark4: Vec<SceneEnvScene>,
    pub FieldDark4_SW: Vec<SceneEnvScene>,
    pub FieldDark5: Vec<SceneEnvScene>,
    pub FieldDark_SW: Vec<SceneEnvScene>,
    pub FieldLight: Vec<SceneEnvScene>,
    pub FieldLight1: Vec<SceneEnvScene>,
    pub FieldLight18_SW: Vec<SceneEnvScene>,
    pub FieldLight1_SW: Vec<SceneEnvScene>,
    pub FieldLight3: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub FieldLight31: String,
    pub FieldLight34: Vec<SceneEnvScene>,
    pub FieldLight34_SW: Vec<SceneEnvScene>,
    pub FieldLight38: Vec<SceneEnvScene>,
    pub FieldLight3_SW: Vec<SceneEnvScene>,
    pub FieldLight4: Vec<SceneEnvScene>,
    pub FieldLight41_SW: Vec<SceneEnvScene>,
    pub FieldLight43: Vec<SceneEnvScene>,
    pub FieldLight44_SW: Vec<SceneEnvScene>,
    pub FieldLight4_SW: Vec<SceneEnvScene>,
    pub FieldLight5: Vec<SceneEnvScene>,
    pub FieldLight_SW: Vec<SceneEnvScene>,
    pub IndoorDark: Vec<SceneEnvScene>,
    pub IndoorDark1: Vec<SceneEnvScene>,
    pub IndoorDark10: Vec<SceneEnvScene>,
    pub IndoorDark11: Vec<SceneEnvScene>,
    pub IndoorDark13: Vec<SceneEnvScene>,
    pub IndoorDark14: Vec<SceneEnvScene>,
    pub IndoorDark15: Vec<SceneEnvScene>,
    pub IndoorDark16: Vec<SceneEnvScene>,
    pub IndoorDark3: Vec<SceneEnvScene>,
    pub IndoorDark4: Vec<SceneEnvScene>,
    pub IndoorDark5: Vec<SceneEnvScene>,
    pub IndoorDark5_SW: Vec<SceneEnvScene>,
    pub IndoorDark9: Vec<SceneEnvScene>,
    pub IndoorLight: Vec<SceneEnvScene>,
    pub IndoorLight1: Vec<SceneEnvScene>,
    pub IndoorLight10: Vec<SceneEnvScene>,
    pub IndoorLight10_SW: Vec<SceneEnvScene>,
    pub IndoorLight11: Vec<SceneEnvScene>,
    pub IndoorLight11_SW: Vec<SceneEnvScene>,
    pub IndoorLight12: Vec<SceneEnvScene>,
    pub IndoorLight12_SW: Vec<SceneEnvScene>,
    pub IndoorLight14: Vec<SceneEnvScene>,
    pub IndoorLight15: Vec<SceneEnvScene>,
    pub IndoorLight15_SW: Vec<SceneEnvScene>,
    pub IndoorLight16: Vec<SceneEnvScene>,
    pub IndoorLight16_SW: Vec<SceneEnvScene>,
    pub IndoorLight17: Vec<SceneEnvScene>,
    pub IndoorLight18: Vec<SceneEnvScene>,
    pub IndoorLight19: Vec<SceneEnvScene>,
    pub IndoorLight2: Vec<SceneEnvScene>,
    pub IndoorLight20: Vec<SceneEnvScene>,
    pub IndoorLight21: Vec<SceneEnvScene>,
    pub IndoorLight3: Vec<SceneEnvScene>,
    pub IndoorLight4: Vec<SceneEnvScene>,
    pub IndoorLight6: Vec<SceneEnvScene>,
    pub IndoorLight7: Vec<SceneEnvScene>,
    pub IndoorLight7_SW: Vec<SceneEnvScene>,
    pub IndoorLight8: Vec<SceneEnvScene>,
    pub IndoorLight9: Vec<SceneEnvScene>,
    pub IndoorLight_SW: Vec<SceneEnvScene>,
    pub TestDemo3: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub TestDemo4: String,
    pub TestDesigner: Vec<SceneEnvScene>,
    #[serde(skip)]
    pub TestDesigner10: String,
    pub TestDesigner12: Vec<SceneEnvScene>,
    pub TestDesigner15: Vec<SceneEnvScene>,
    pub TestDesigner16: Vec<SceneEnvScene>,
    pub TestDesigner5: Vec<SceneEnvScene>,
    pub TestDesigner6: Vec<SceneEnvScene>,
    pub TestImamura2: Vec<SceneEnvScene>,
    pub TestImamura3: Vec<SceneEnvScene>,
    pub TestNakamura1: Vec<SceneEnvScene>,
    pub TestNakamura1_SW: Vec<SceneEnvScene>,
    pub TestNakamura2: Vec<SceneEnvScene>,
    pub TestNakamura3: Vec<SceneEnvScene>,
    pub TestNakamura8: Vec<SceneEnvScene>,
    pub TestNakamura_SW: Vec<SceneEnvScene>,
    pub TestSakasai7: Vec<SceneEnvScene>,
    pub TestShikata: Vec<SceneEnvScene>,
    pub TestTominaga2: Vec<SceneEnvScene>,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneEnvScene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amb: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub areaId: Option<AreaId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch2: Option<String>,
    #[serde(skip)]
    pub come: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cmn: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crs: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eft: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog0: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fog1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damb0: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damb0k: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damb1: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damb1k: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub damb2: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub darkbgsp: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlgt: Option<String>,
    /// Used in Thieves' Hideout, values: 1 or 2. Small light that follows Link?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follow: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isdark: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isfixed: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isfixed_ch: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_pe: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obj: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pe: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peclr: Option<Vec<f32>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stg: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sw: Option<SW>,
}

impl SceneEnvScene {
    pub fn turn_off_lights(&mut self) {
        self.isdark = Some(true);
        self.dlgt = Some("cmn_darkness.bch".to_owned());
        self.darkbgsp = Some(false);
        self.amb = Some(Vec::from([0.0, 0.0, 0.0]));
    }

    pub fn turn_on_lights(&mut self) {
        self.isdark = Some(false);
        self.dlgt = None;
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AreaId {
    String(String),
    Array(Vec<i32>),
}

impl Default for AreaId {
    fn default() -> Self {
        AreaId::String("null".to_owned())
    }
}

pub type SW = (i32, i32, i32, bool);
