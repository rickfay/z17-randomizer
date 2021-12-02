use albw::{Game, Result};

pub fn load() -> Result<Game> {
    Game::load(TEST_ROM_PATH)
}

const TEST_ROM_PATH: &str = "../test.3ds";
