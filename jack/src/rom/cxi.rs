use {
    crate::rom::{exheader::ExHeader, romfs::RomFs},
    bytey::{FromBytesOwned, TryFromBytesOwned},
    data_encoding::HEXUPPER,
    log::{error, info},
    macros::fail::pause,
    ring::digest::{Context, SHA256},
    std::{
        error, fs,
        io::{BufReader, Read, Seek},
        path::Path,
        process::exit,
    },
};

#[derive(Debug)]
pub struct Cxi<R> {
    file: R,
    id: u64,
    offset: u32,
}

impl<R> Cxi<R>
where
    R: Read + Seek,
{
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn exheader(&mut self) -> Result<ExHeader, Box<dyn std::error::Error>> {
        Ok(ExHeader::read_from_offset(
            &mut self.file,
            self.offset + SIGNATURE_LEN as u32 + HEADER_LEN as u32,
        )?)
    }

    pub fn try_into_romfs(mut self) -> std::io::Result<RomFs<R>> {
        let media_units = u32::read_from_offset(&mut self.file, self.offset + 0x1B0)?;
        RomFs::load(self.file, self.offset + from_media_units(media_units))
    }
}

impl Cxi<fs::File> {
    pub fn open<P>(path: P) -> Result<Self, Box<dyn error::Error>>
    where
        P: AsRef<Path>,
    {
        let path = path.as_ref();
        let mut file = match fs::File::open(path) {
            Ok(file) => file,
            Err(_) => {
                error!("Couldn't load ROM from: \"{}\"", path.display());
                error!("Please check that config.json points to a valid ROM.");
                pause();
                exit(1);
            }
        };

        //validate_rom(&file);

        bytey::typedef! { struct NCSD: TryFromBytes<'_> [HEADER_LEN] {
            #b"NCSD",
            [8] id: u64,
            [0x20] offset: u32,
        }}
        let header = NCSD::try_read_from_offset(&mut file, SIGNATURE_LEN)?;
        let offset = from_media_units(header.offset);
        bytey::typedef! { struct NCCH: TryFromBytes<'_> [HEADER_LEN] {
            #b"NCCH",
            [8] id: u64,
            [0x18] program_id: u64,
        }}
        let ncch = NCCH::try_read_from_offset(&mut file, offset + SIGNATURE_LEN as u32)?;
        assert_eq!(ncch.id, header.id);
        assert_eq!(ncch.program_id, header.id);
        Ok(Self { file, id: header.id, offset })
    }
}

// todo redo validation, this isn't meaningful
#[allow(unused)]
fn validate_rom(file: &fs::File) {
    //info!("Calculating Checksum...");

    let mut reader = BufReader::new(file);
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    let checksum = &HEXUPPER.encode(context.finish().as_ref());
    info!("SHA256 Checksum:                {}", checksum);

    // let valid_checksum = String::from("4071DC95F6948669C7A13D378509D5A224E167B77CF8FD2E163484BA9AF8B64D");
    // let encrypted_checksum = String::from("tbd"); // TODO determine this value
    //
    // if valid_checksum.eq(checksum) {
    //     info!("ROM is valid.");
    // } else {
    //     if encrypted_checksum.eq(checksum) {
    //         error!("ROM is encrypted. Please decrypt this ROM before using the randomizer.");
    //     } else {
    //         error!("Invalid checksum: {}", checksum);
    //         error!("ROM is invalid. Please provide a decrypted, North American ROM of The Legend of Zelda: A Link Between Worlds.");
    //     }
    //
    //     pause();
    //     std::process::exit(1);
    // }
}

fn from_media_units(media_units: u32) -> u32 {
    media_units << MEDIA_UNIT_SHIFT
}

const SIGNATURE_LEN: u64 = 0x100;
const HEADER_LEN: usize = 0x100;
const MEDIA_UNIT_SHIFT: u8 = 9;
