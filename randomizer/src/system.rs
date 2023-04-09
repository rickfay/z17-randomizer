use {
    crate::{constants::CONFIG_FILE_NAME, fail, Settings},
    json_comments::StripComments,
    log::info,
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::{
        error::Error as StdError,
        fmt::{self, Display, Formatter},
        fs, io,
        path::{Path, PathBuf},
    },
};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Error {
    inner: Box<dyn StdError + Send + Sync + 'static>,
}

impl Error {
    pub fn new<E>(err: E) -> Self
    where
        E: Into<Box<dyn StdError + Send + Sync + 'static>>,
    {
        Self { inner: err.into() }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(self.inner.as_ref())
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self { inner: err.into() }
    }
}

pub struct System;

impl System {
    pub fn load_preset(name: &str) -> Result<Settings> {
        let file = PathBuf::from("presets").join(format!("{}.json", name));
        info!("Loading preset from:            {}\n", file.display());
        Self::load_json(file)
    }

    pub fn load_config<T: DeserializeOwned>() -> Result<T> {
        let file = PathBuf::from(CONFIG_FILE_NAME);
        if file.exists() {
            Self::load_json(file)
        } else {
            fail!("No config file found at {}", file.to_path_buf().display());
        }
    }

    fn load_json<T: DeserializeOwned>(file: PathBuf) -> Result<T> {
        let file = fs::read_to_string(file)?;
        let stripped = StripComments::new(file.as_bytes());
        let paths = serde_json::from_reader(stripped).map_err(Error::new)?;
        Ok(paths)
    }
}

/// Paths to the game ROM and output directories.
#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    rom: PathBuf,
    output: PathBuf,
}

impl UserConfig {
    /// Generates new paths with the specified ROM and output directory.
    pub fn new(rom: PathBuf, output: PathBuf) -> Self {
        Self { rom, output }
    }

    /// Gets the path of the ROM file.
    pub fn rom(&self) -> &Path {
        &self.rom
    }

    /// Gets the output directory.
    pub fn output(&self) -> &Path {
        &self.output
    }
}
