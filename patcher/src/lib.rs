use std::path::PathBuf;
use {
    jack::{rom::Rom, JackFile},
    macros::fail,
    seed::{settings::Settings, Seed},
    std::{collections::HashMap, error::Error, fs::File},
};

mod archive;
mod code;
mod language;
pub(crate) mod util;
mod world;

///
pub fn generate_patch(seed: &Seed, input: PathBuf, output: PathBuf) -> Result<(), Box<dyn Error>> {
    let mut rom = Rom::load(input)?;

    let mut patches = HashMap::new();
    patches.extend(code::patch(seed, &mut rom));
    patches.extend(archive::patch(seed, &mut rom));
    patches.extend(language::patch(seed, &mut rom));
    patches.extend(world::patch(seed, &mut rom));

    write_patches(patches)
}

///
fn write_patches(patches: HashMap<String, Vec<u8>>) -> Result<(), Box<dyn Error>> {
    todo!()
}
