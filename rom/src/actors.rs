use {
    crate::{files::sarc::Sarc, File, Result},
    std::path::Path,
};

#[derive(Debug)]
pub struct Actors {
    archive: File<Sarc>,
    dirty: bool,
}

impl Actors {
    pub(crate) fn new(archive: File<Sarc>) -> Self {
        Self { archive, dirty: false }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.archive.get().contains(format!("World/Actor/{}.bch", name)).unwrap_or(false)
    }

    pub fn get_actor_bch(&self, name: &str) -> Result<Actor> {
        self.archive.get().extract(format!("World/Actor/{}.bch", name))
    }

    pub fn get_actor_kcl(&self, name: &str) -> Result<Actor> {
        self.archive.get().extract(format!("World/Actor/{}.kcl", name))
    }

    pub fn get_map_actor_bch(&self, name: &str) -> Result<Actor> {
        self.archive.get().extract(format!("World/MapActor/{}.bch", name))
    }

    pub fn get_map_actor_kcl(&self, name: &str) -> Result<Actor> {
        self.archive.get().extract(format!("World/MapActor/{}.kcl", name))
    }

    pub fn add(&mut self, actor: Actor) -> Result<()> {
        self.dirty = true;
        self.archive.get_mut().add(actor)
    }

    pub fn update(&mut self, actor: Actor) -> Result<()> {
        self.dirty = true;
        self.archive.get_mut().update(actor)
    }

    pub fn into_archive(self) -> Option<File<Sarc>> {
        self.dirty.then(|| self.archive.map(Sarc::compress))
    }

    pub fn dump<P>(self, path: P) -> Result<()>
    where
        P: AsRef<Path>,
    {
        if self.dirty {
            self.archive.map(Sarc::compress).dump(path)?;
        }
        Ok(())
    }
}

pub type Actor = File<Box<[u8]>>;
