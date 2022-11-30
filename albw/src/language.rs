use std::{
    cell::Ref,
    collections::{BTreeMap, HashSet},
    marker::PhantomData,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::{course, files::{sarc::Sarc, FromFile}, flow::{Flow, FlowMut}, Error, File, Result};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FlowChart {
    pub load: Load,
    pub table: Table,
}

impl FlowChart {
    pub fn load(&self) -> &Load {
        &self.load
    }

    pub fn load_mut(&mut self) -> &mut Load {
        &mut self.load
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Load(
    pub BTreeMap<String, Vec<String>>
);

impl Load {
    pub fn boot(&self) -> Result<&[String]> {
        Ok(&self
            .0
            .get("Boot")
            .ok_or_else(|| Error::new("Boot key not found."))?)
    }

    pub fn course(&self, id: course::Id) -> Option<&[String]> {
        self.0.get(id.as_str()).map(AsRef::as_ref)
    }

    pub fn add_entry(&mut self, id: &str, entry: &str) {
        let load_entry = self.0.entry(String::from(id)).or_insert_with(|| Vec::new());

        // Manually check for dupe - Choosing not to switch to a HashSet to minimize file changes
        if !load_entry.contains(&String::from(entry)) {
            load_entry.push(String::from(entry));
        }
    }

    pub fn remove_entry(&mut self, id: &str, entry: &str) {
        let thing = self.0.get_mut(id);
        if thing.is_some() {
            thing.unwrap().retain(|x| *x != String::from(entry));
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Table {
    entry: Vec<(String, usize)>,
    file: Vec<String>,
}

#[derive(Debug)]
pub struct Language {
    pub flow: HashSet<String>,
    pub archive: File<Sarc>,
}

impl Language {
    pub(crate) fn new<F>(flow: F, archive: File<Sarc>) -> Self
        where
            F: IntoIterator<Item=String>,
    {
        Self {
            flow: flow.into_iter().collect(),
            archive,
        }
    }

    pub fn flow(&self) -> Loaded<Flow> {
        Loaded::new(&self.flow, &self.archive)
    }

    pub fn flow_mut(&mut self) -> LoadedMut<FlowMut> {
        LoadedMut::new(&mut self.flow, &mut self.archive)
    }

    pub fn flow_inject(&mut self, name: &str, file: File<Box<[u8]>>) -> Result<()> {
        self.flow.insert(String::from(name));
        self.archive.get_mut().add(file)
    }

    pub fn into_archive(self) -> File<Sarc> {
        self.archive.map(Sarc::compress)
    }

    pub fn dump<P>(self, path: P) -> Result<()>
        where
            P: AsRef<Path>,
    {
        self.archive.map(Sarc::compress).dump(path)
    }
}

#[derive(Debug)]
pub struct Loaded<'a, T> {
    set: &'a HashSet<String>,
    archive: &'a File<Sarc>,
    phantom: PhantomData<T>,
}

impl<'a, T> Loaded<'a, T> {
    pub fn new(set: &'a HashSet<String>, archive: &'a File<Sarc>) -> Self {
        Self {
            set,
            archive,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> Loaded<'a, T>
    where
        T: FromFile<PathArgs=str, Input=Ref<'a, [u8]>> + 'a,
{
    pub fn iter<'b: 'a>(&'b self) -> impl Iterator<Item=Result<File<T>>> + 'b {
        self.set
            .iter()
            .map(move |name| self.archive.get().read_from_file(name.as_str()))
    }

    pub fn get(&self, name: &str) -> Option<Result<File<T>>> {
        self.set
            .contains(name)
            .then(|| self.archive.get().read_from_file(name))
    }

    pub fn extract(&self, name: &str) -> Option<File<Box<[u8]>>> {
        self.archive.get().extract(name).ok()
    }
}

#[derive(Debug)]
pub struct LoadedMut<'a, T> {
    set: &'a mut HashSet<String>,
    archive: &'a mut File<Sarc>,
    phantom: PhantomData<T>,
}

impl<'a, T> LoadedMut<'a, T> {
    pub fn new(set: &'a mut HashSet<String>, archive: &'a mut File<Sarc>) -> Self {
        Self {
            set,
            archive,
            phantom: PhantomData,
        }
    }

    pub fn get_mut<'s>(&'s mut self, name: &str) -> Option<Result<File<T>>>
        where
            T: FromFile<PathArgs=str, Input=&'s mut [u8]> + 's,
    {
        self.set
            .contains(name)
            .then(move || self.archive.get_mut().open_from_file(name))
    }
}
