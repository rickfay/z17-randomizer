use albw::{course, Language, Result};

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
    for course in course::Id::iter() {
        iterate(game.course(course).language()?)?;
    }
    Ok(())
}
