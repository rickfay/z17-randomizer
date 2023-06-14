#![feature(drain_filter)]

use std::ops::{Deref, DerefMut};

pub mod byaml;
pub mod item;
pub mod lms;
pub mod rom;
pub mod sead;

/// Generic Jack File
pub struct JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    path: String,
    file: T,
}

impl<T> JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    pub fn new(path: &str, file: T) -> Self {
        Self { path: path.to_owned(), file }
    }

    pub fn get_path(&self) -> &str {
        &self.path
    }
}

impl<T> Into<Vec<u8>> for JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    fn into(self) -> Vec<u8> {
        self.file.into()
    }
}

impl<T> Deref for JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}

impl<T> DerefMut for JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}

impl<T> AsRef<T> for JackFile<T>
where
    T: Into<Vec<u8>> + From<Vec<u8>>,
{
    fn as_ref(&self) -> &T {
        &self.deref()
    }
}
