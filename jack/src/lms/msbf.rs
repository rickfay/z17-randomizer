use {
    crate::lms::{msbf::MsbfNodeType::*, Label},
    byteorder::{BigEndian, ByteOrder, LittleEndian, ReadBytesExt},
    log::warn,
    macros::{fail, string_constants},
    std::{
        collections::BTreeMap,
        io::{BufRead, BufReader, Cursor, Read, Seek, SeekFrom},
        str::from_utf8,
    },
};

/// Flow Chart Files
pub struct MsbfFile {
    nodes: Vec<MsbfNode>,
    branch_paths: Vec<Vec<u16>>,
    flw3_strings: Vec<String>,
    fen1_labels: Vec<Label>,
}

impl From<Vec<u8>> for MsbfFile {
    fn from(bytes: Vec<u8>) -> Self {
        // MsgFlwBn Header
        let mut buf = BufReader::new(Cursor::new(&bytes));
        assert_eq!(
            &buf.read_u64::<BigEndian>().unwrap().to_be_bytes(),
            b"MsgFlwBn",
            "MsgFlwBn Magic"
        );

        // FLW3 Header
        buf.seek(SeekFrom::Start(0x20)).unwrap();
        assert_eq!(&buf.read_u32::<BigEndian>().unwrap().to_be_bytes(), b"FLW3", "FLW3 Magic");
        let flw3_len = buf.read_u16::<LittleEndian>().unwrap() as usize;

        buf.seek(SeekFrom::Start(0x30)).unwrap();
        let num_nodes = buf.read_u16::<LittleEndian>().unwrap() as usize;
        let num_branches = buf.read_u16::<LittleEndian>().unwrap() as usize;

        // FLW3 Nodes
        buf.seek(SeekFrom::Start(0x30)).unwrap();
        let mut nodes = Vec::with_capacity(num_nodes);
        let mut branch_count_map = BTreeMap::new();
        for _ in 0..num_nodes {
            let node_buf = &mut [0; 0x10];
            buf.read_exact(node_buf).unwrap();
            let node = MsbfNode { bytes: *node_buf };
            if node.get_type() == Branch {
                branch_count_map.insert(node.get_branch(), node.get_count());
            }

            nodes.push(node);
        }

        // FLW3 Branches
        let mut branch_paths = Vec::with_capacity(num_branches);
        let mut branch_idx = 0;
        let mut warned = false;
        for (branch, count) in branch_count_map {
            if !warned && branch != branch_idx {
                warn!("MSBF branch indexes not continuous");
                warned = true;
            }
            branch_idx += 1;

            let mut branch_path = Vec::with_capacity(count as usize);
            for _ in 0..count {
                branch_path.push(buf.read_u16::<LittleEndian>().unwrap())
            }

            branch_paths.push(branch_path);
        }

        // FLW3 Strings...? IDK Honestly what these are for or do, just preserving them AS-IS for now
        let current_pos =
            buf.seek(SeekFrom::Current(0)).expect("Could not get current position!") as usize;
        let mut flw3_strings = vec![];
        while current_pos < flw3_len {
            let str_buffer = &mut Vec::new();
            let str_len = buf.read_until(0, str_buffer).unwrap();
            flw3_strings.push(
                from_utf8(&str_buffer[..str_len - 1])
                    .map(|s| s.to_owned())
                    .expect(&format!("Error reading MSBF Strings")),
            );
        }

        let flw3_padding_amt = 0x10 - (flw3_len % 0x10);
        let fen1_start = (0x30 + flw3_len + flw3_padding_amt) as u64;

        // FEN1 Header
        buf.seek(SeekFrom::Start(fen1_start)).unwrap();
        assert_eq!(&buf.read_u32::<BigEndian>().unwrap().to_be_bytes(), b"FEN1", "FEN1 Magic");
        let fen1_len = buf.read_u16::<LittleEndian>().unwrap() as u64;

        // FEN1 Hash Table
        let fen1_hash_start = fen1_start + 0x10;
        let fen1_end = (fen1_hash_start + fen1_len) as usize;
        buf.seek(SeekFrom::Start(fen1_hash_start)).unwrap();

        let fen1_hash_len = buf.read_u32::<LittleEndian>().unwrap();
        assert_eq!(fen1_hash_len, 0x3B, "FEN1 size was not 0x3B");

        // different strat from MSBT files... skip reading the hash structure entirely, just read the strings
        // and reconstruct the hash table from them later

        let fen1_labels_start = fen1_start + 0x10 + (fen1_hash_len as u64 * 0x8);
        let mut fen1_labels = vec![];
        let mut current_pos = fen1_labels_start as usize;

        while current_pos < fen1_end {
            let label_len = bytes[current_pos] as usize;
            current_pos += 1;
            let label = from_utf8(&bytes[current_pos..current_pos + label_len])
                .expect(&format!("Error reading FEN1 labels"))
                .to_owned();
            current_pos += label_len;
            let item_index = LittleEndian::read_u32(&bytes[current_pos..current_pos + 4]);
            current_pos += 4;
            fen1_labels.push(Label { label, item_index });
        }

        Self { nodes, branch_paths, flw3_strings, fen1_labels }
    }
}

impl Into<Vec<u8>> for MsbfFile {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}

impl MsbfFile {
    fn get_branch_paths(&mut self, branch: usize) -> &mut Vec<u16> {
        self.branch_paths.get_mut(branch).expect(&format!("No branch with index: {}", branch))
    }
}

/// A Node representing a single step within the flow
struct MsbfNode {
    bytes: [u8; 0x10],
}

impl MsbfNode {
    fn get_bytes_mut(&mut self) -> &mut [u8; 0x10] {
        &mut self.bytes
    }

    fn get_type(&self) -> MsbfNodeType {
        self.bytes[0].into()
    }

    fn set_type(&mut self, node_type: MsbfNodeType) {
        self.bytes[0] = node_type.to_u8();
    }

    fn get_arg1(&self) -> u8 {
        self.bytes[1]
    }

    fn set_arg1(&mut self, arg1: u8) {
        self.bytes[1] = arg1;
    }

    fn get_arg2(&self) -> u8 {
        self.bytes[2]
    }

    fn set_arg2(&mut self, arg2: u8) {
        self.bytes[2] = arg2;
    }

    fn get_arg3(&self) -> u8 {
        self.bytes[3]
    }

    fn set_arg3(&mut self, arg3: u8) {
        self.bytes[3] = arg3;
    }

    fn get_value(&self) -> u32 {
        self.bytes[4] as u32
            | (self.bytes[5] as u32) << 8
            | (self.bytes[6] as u32) << 0x10
            | (self.bytes[7] as u32) << 0x18
    }

    fn set_value(&mut self, value: u32) {
        self.bytes[4..8].swap_with_slice(&mut value.to_le_bytes());
    }

    fn get_next(&self) -> u16 {
        self.bytes[8] as u16 | (self.bytes[9] as u16) << 8
    }

    fn set_next(&mut self, next: u16) {
        self.bytes[8..0xA].swap_with_slice(&mut next.to_le_bytes());
    }

    fn get_command(&self) -> u16 {
        self.bytes[0xA] as u16 | (self.bytes[0xB] as u16) << 8
    }

    fn set_command(&mut self, command: u16) {
        self.bytes[0xA..0xC].swap_with_slice(&mut command.to_le_bytes());
    }

    fn get_count(&self) -> u16 {
        self.bytes[0xC] as u16 | (self.bytes[0xD] as u16) << 8
    }

    fn set_count(&mut self, count: u16) {
        self.bytes[0xC..0xE].swap_with_slice(&mut count.to_le_bytes());
    }

    fn get_branch(&self) -> u16 {
        self.bytes[0xE] as u16 | (self.bytes[0xF] as u16) << 8
    }

    fn set_branch(&mut self, branch: u16) {
        self.bytes[0xE..0x10].swap_with_slice(&mut branch.to_le_bytes());
    }
}

/// Types for an [`MsbfNode`]
#[derive(PartialEq)]
enum MsbfNodeType {
    /// Display a textbox
    Message,
    /// Executes some sort of conditional and then chooses the next step forward based on the result
    Branch,
    /// Arbitrary "do a thing" step
    Event,
    /// Starting nodes
    Entry,
    /// Unclear
    Goto,
}

impl MsbfNodeType {
    fn to_u8(self) -> u8 {
        match self {
            Message => 1,
            Branch => 2,
            Event => 3,
            Entry => 4,
            Goto => 5,
        }
    }
}

impl From<u8> for MsbfNodeType {
    fn from(value: u8) -> Self {
        match value {
            1 => Message,
            2 => Branch,
            3 => Event,
            4 => Entry,
            5 => Goto,
            _ => fail!("Invalid MsbfNodeType: {}", value),
        }
    }
}

string_constants! {
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
