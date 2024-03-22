extern crate winres;

fn main() {
    if cfg!(target_os = "windows") || cfg!(target_os = "windows-latest") {
        let mut res = winres::WindowsResource::new();
        res.set_icon("icon.ico");
        res.compile().unwrap();
    }
}
