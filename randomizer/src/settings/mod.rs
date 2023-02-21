pub mod entrance_shuffle_setting;
pub mod logic;
pub mod logic_mode;
pub mod pedestal_setting;
pub mod settings;

pub(crate) const fn is_false(b: &bool) -> bool {
    *b == false
}
pub(crate) const fn seven() -> u8 {
    7
}
pub(crate) const fn r#true() -> bool {
    true
}
