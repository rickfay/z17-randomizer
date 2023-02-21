use {
    crate::{constants::CONFIG_FILE_NAME, fail},
    log::info,
    serde::{de::DeserializeOwned, Deserialize, Serialize},
    std::{
        error::Error as StdError,
        fmt::{self, Display, Formatter},
        fs, io,
        marker::PhantomData,
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

/// An abstraction over platform-specific functionality.
#[derive(Debug)]
pub struct System<P> {
    config: PathBuf,
    presets: PhantomData<P>,
}

impl<P> System<P> {
    pub fn new() -> Result<Self>
    where
        P: Serialize,
    {
        Ok(Self { config: PathBuf::from(""), presets: PhantomData })
    }

    pub fn preset(&self, name: &str) -> Result<P>
    where
        P: DeserializeOwned,
    {
        let path = self.config.join("presets").join(format!("{}.json", name));
        info!("Loading preset from:            {}\n", path.display());
        serde_json::from_slice(&fs::read(path)?).map_err(Error::new)
    }

    pub fn load_config(&self) -> Result<Paths> {
        let file = self.config.join(CONFIG_FILE_NAME);
        if file.exists() {
            Ok(serde_json::from_slice::<Paths>(&fs::read(file)?).map_err(Error::new)?)
        } else {
            fail!("No config file found at {}", file.to_path_buf().display());
        }
    }
}

/// Paths to the game ROM and output directories.
#[derive(Debug, Serialize, Deserialize)]
pub struct Paths {
    rom: PathBuf,
    output: PathBuf,
}

impl Paths {
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
