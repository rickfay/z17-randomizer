use {
    crate::{
        byaml::{flow_chart::FlowChart, get_item::GetItem},
        rom::{cxi::Cxi, exheader::ExHeader, region::TitleId, romfs::RomFs},
        sead::SzsFile,
        JackFile,
    },
    log::{error, info},
    macros::fail::pause,
    std::{collections::HashMap, error, fs::File, ops::Deref, path::PathBuf, process::exit},
};

mod code;
mod cxi;
mod exheader;
pub mod fs;
mod region;
mod romfs;

/// Source game ROM
pub struct Rom {
    exheader: ExHeader,
    romfs: RomFs<File>,
}

impl Rom {
    pub fn read<T>(&mut self, filename: &str) -> std::io::Result<JackFile<T>>
    where
        T: From<Vec<u8>> + Into<Vec<u8>>
    {
        self.romfs.read(filename)
    }
}

impl Rom {
    /// Load the ROM and do initial parsing and validation
    pub fn load(path: PathBuf) -> Result<Self, Box<dyn error::Error>> {
        let mut cxi = Cxi::open(&path)?;

        let title_id = TitleId::from(cxi.id());
        info!("Detected ROM: {}", title_id);
        match title_id {
            TitleId::US => {}
            _ => {
                error!(
                    "Unsupported ROM detected. Sorry, only the US version of ALBW is currently supported."
                );
                pause();
                exit(1);
            }
        }

        let exheader = cxi.exheader()?;
        let romfs = cxi.try_into_romfs()?;

        Ok(Self { exheader, romfs })
    }
}

const NCSD_SIGNATURE_LEN: u64 = 0x100;
const MEDIA_UNIT_SIZE: u32 = 0x200;
const EXHEADER_LEN: usize = 0x400;
