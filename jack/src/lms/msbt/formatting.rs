#![allow(non_upper_case_globals)]

use lazy_static::lazy_static;

// Font Symbols - See US/MessageFont.bffnt for mapping and character, glyph, and offset info.
lazy_static! {
    pub static ref A_BUTTON: String = utf16(&[0xE000]);
    pub static ref B_BUTTON: String = utf16(&[0xE001]);
    pub static ref X_BUTTON: String = utf16(&[0xE002]);
    pub static ref Y_BUTTON: String = utf16(&[0xE003]);
    pub static ref L_BUTTON: String = utf16(&[0xE004]);
    pub static ref R_BUTTON: String = utf16(&[0xE005]);
    pub static ref DPAD: String = utf16(&[0xE006]);
    pub static ref DIAMOND: String = utf16(&[0xE016]);
    pub static ref HEART: String = utf16(&[0xE017]);
    pub static ref ARROW_R: String = utf16(&[0xE019]);
    pub static ref ARROW_D: String = utf16(&[0xE01C]);
    pub static ref CAROT_L: String = utf16(&[0xE036]);
    pub static ref CAROT_R: String = utf16(&[0xE037]);
    pub static ref PLUS: String = utf16(&[0xE045]);
    pub static ref MINUS: String = utf16(&[0xE046]);
    pub static ref RAVIO: String = utf16(&[0xE05E]);
    pub static ref e: String = utf16(&[0xE06A]);
    pub static ref BOW: String = utf16(&[0xE06C]);
    pub static ref BOMBS: String = utf16(&[0xE06D]);
    pub static ref FIRE_ROD: String = utf16(&[0xE06E]);
    pub static ref DPAD_D: String = utf16(&[0xE07A]);
    pub static ref DPAD_R: String = utf16(&[0xE07C]);
    pub static ref BANG: String = utf16(&[0xFF01]);
    pub static ref QUESTION: String = utf16(&[0xFF1F]);
    pub static ref TILDE: String = utf16(&[0xFF5E]);
}

// Controls - Special purpose strings
lazy_static! {
    pub static ref CHOICE_2: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD02]);
    pub static ref CHOICE_3: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD03]);
    pub static ref CHOICE_4: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD04]);
    pub static ref PRICE: String = utf16(&[0xE, 0x1, 0x5, 0x6, 0x0, 0xFFFF, 0xCD01]);
}

// Referenced Strings
lazy_static! {
    pub static ref PLAYER_NAME: String = utf16(&[0xE, 0x1, 0x0, 0x0]);
    pub static ref SAHASRAHLA: String = utf16(&[0xE, 0x2, 0x0, 0x2, 0x0109]);
    pub static ref HOUSE_OF_GALES: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x0101, 0xCD00]);
    pub static ref TOWER_OF_HERA: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x0102, 0xCD00]);
    pub static ref KAKARIKO_VILLAGE: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x010D, 0xCD2E]);
    pub static ref TORNADO_ROD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x0102, 0xCD00]);
    pub static ref PENDANT_OF_COURAGE: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x0123, 0xCD00]);
    pub static ref MASTER_SWORD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x012C, 0xCD00]);
}

fn utf16(v: &[u16]) -> String {
    String::from_utf16(v).unwrap()
}

#[allow(unused)]
pub fn big(text: &str) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64, 0xE, 0x0, 0x2, 0x2, 0x82]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
}

#[allow(unused)]
pub fn troll() -> String {
    purple(RAVIO.as_str())
}

/// Black Color: 0x262626FF
pub fn black(text: &str) -> String {
    color(text, 0x0)
}

/// Gray Color: 0x808080FF
pub fn gray(text: &str) -> String {
    color(text, 0x1)
}

/// White Color: 0xFFFFFFFF
pub fn white(text: &str) -> String {
    color(text, 0x2)
}

/// Beige Color: 0x855C2FFF
pub fn beige(text: &str) -> String {
    color(text, 0x3)
}

/// Red Color: 0x591710FF <br />
/// Note: [`attention`] is what's typically used in-game for red text.
pub fn red(text: &str) -> String {
    color(text, 0x4)
}

/// Green Color: 0x006400FF
pub fn green(text: &str) -> String {
    color(text, 0x5)
}

/// Blue Color: 0x375960FF <br />
/// Note: [`name`] is what's typically used in-game for blue text.
pub fn blue(text: &str) -> String {
    color(text, 0x6)
}

/// Yellow Text: 0xBAA800FF
/// Used for Yuga's general text
pub fn yellow(text: &str) -> String {
    color(text, 0x7)
}

/// Purple Text: 0x3A1B4CFF
pub fn purple(text: &str) -> String {
    color(text, 0x8)
}

/// Name Color: 0x003F97FF <br />
/// This blue coloring is what the game usually uses to highlight names/places/etc.
pub fn name(text: &str) -> String {
    color(text, 0x9)
}

/// Attention Color: 0xF92300FF <br />
/// The game typically uses this over the [`red`] color option.
pub fn attention(text: &str) -> String {
    color(text, 0xA)
}

/// YugaTalking Color: 0x4AF0D1FF <br />
/// This is a Cyan color used for highlighted words when Yuga is speaking.
pub fn yuga_talking(text: &str) -> String {
    color(text, 0xB)
}

fn color(text: &str, index: u16) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x3, 0x2, index]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x3, 0x2, 0xFFFF]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
}
