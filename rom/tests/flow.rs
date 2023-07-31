use rom::{Language, Result};
use strum::IntoEnumIterator;

mod common;

fn iterate(language: Language) -> Result<()> {
    for flow in language.flow().iter() {
        for step in flow?.get().steps().iter() {
            step?;
        }
    }
    Ok(())
}

#[test]
fn it_works() -> Result<()> {
    let game = common::load()?;
    iterate(game.boot()?)?;
    for course in game::Course::iter() {
        iterate(game.course(course).language()?)?;
    }
    Ok(())
}