use game::Course;
use rom::{Language, Result};

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
    for course in Course::iter() {
        iterate(game.course(course).language()?)?;
    }
    Ok(())
}
