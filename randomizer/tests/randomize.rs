use std::array;

use randomizer::{settings, Generator, Result};

#[test]
fn it_randomizes_multiple_seeds() -> Result<()> {
    let settings = Default::default();
    for seed in array::IntoIter::new([0, 1337, 0xCAFEDEAD, 0xFFFFFFFF]) {
        Generator::new(&settings, seed).randomize();
    }
    Ok(())
}

#[test]
fn it_works_with_open() -> Result<()> {
    Generator::new(&settings::open_default(), 0).randomize();
    Ok(())
}
