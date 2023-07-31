use rom::{Result, Rom};

pub fn load() -> Result<Rom> {
    Rom::load(TEST_ROM_PATH)
}

const TEST_ROM_PATH: &str = "../test.3ds";
