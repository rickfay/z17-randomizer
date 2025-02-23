use std::env;
use std::io;
// use winres::WindowsResource;

fn main() -> io::Result<()> {
    // Windows
    if env::var_os("CARGO_CFG_WINDOWS").is_some() {
        // FIXME this is creating problems for some reason, commenting out until it can be fixed.
        // let mut res = WindowsResource::new();
        // res.set_icon("icon.ico");
        // res.compile()?;
    }

    // macOS - todo
    // unix  - todo

    Ok(())
}
