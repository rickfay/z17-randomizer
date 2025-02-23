#![allow(non_upper_case_globals)]
use lazy_static::lazy_static;

// Font Symbols - See US/MessageFont.bffnt for mapping and character, glyph, and offset info.
lazy_static! {
    pub(crate) static ref A_BUTTON: String = utf16(&[0xE000]);
    pub(crate) static ref B_BUTTON: String = utf16(&[0xE001]);
    pub(crate) static ref X_BUTTON: String = utf16(&[0xE002]);
    pub(crate) static ref Y_BUTTON: String = utf16(&[0xE003]);
    pub(crate) static ref L_BUTTON: String = utf16(&[0xE004]);
    pub(crate) static ref R_BUTTON: String = utf16(&[0xE005]);
    pub(crate) static ref DPAD: String = utf16(&[0xE006]);
    pub(crate) static ref DIAMOND: String = utf16(&[0xE016]);
    pub(crate) static ref HEART: String = utf16(&[0xE017]);
    pub(crate) static ref ARROW_R: String = utf16(&[0xE019]);
    pub(crate) static ref ARROW_D: String = utf16(&[0xE01C]);
    pub(crate) static ref CAROT_L: String = utf16(&[0xE036]);
    pub(crate) static ref CAROT_R: String = utf16(&[0xE037]);
    pub(crate) static ref PLUS: String = utf16(&[0xE045]);
    pub(crate) static ref MINUS: String = utf16(&[0xE046]);
    pub(crate) static ref RAVIO: String = utf16(&[0xE05E]);
    pub(crate) static ref e: String = utf16(&[0xE06A]);
    pub(crate) static ref SYMBOL_BOW: String = utf16(&[0xE06C]);
    pub(crate) static ref SYMBOL_BOMBS: String = utf16(&[0xE06D]);
    pub(crate) static ref SYMBOL_FIRE_ROD: String = utf16(&[0xE06E]);
    pub(crate) static ref DPAD_D: String = utf16(&[0xE07A]);
    pub(crate) static ref DPAD_R: String = utf16(&[0xE07C]);
    pub(crate) static ref BANG: String = utf16(&[0xFF01]);
    pub(crate) static ref QUESTION: String = utf16(&[0xFF1F]);
    pub(crate) static ref TILDE: String = utf16(&[0xFF5E]);
}

// Controls - Special purpose strings
lazy_static! {
    pub(crate) static ref CHOICE_2: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD02]);
    pub(crate) static ref CHOICE_3: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD03]);
    pub(crate) static ref CHOICE_4: String = utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD04]);
    pub(crate) static ref PRICE: String = utf16(&[0xE, 0x1, 0x5, 0x6, 0x0, 0xFFFF, 0xCD01]);
}

// Referenced Strings
lazy_static! {
    pub(crate) static ref PLAYER_NAME: String = utf16(&[0xE, 0x1, 0x0, 0x0]);
    pub(crate) static ref SAHASRAHLA: String = utf16(&[0xE, 0x2, 0x0, 0x2, 0x0109]);
    pub(crate) static ref HOUSE_OF_GALES: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x0101, 0xCD00]);
    pub(crate) static ref TOWER_OF_HERA: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x0102, 0xCD00]);
    pub(crate) static ref KAKARIKO_VILLAGE: String = utf16(&[0xE, 0x2, 0x1, 0x4, 0x010D, 0xCD2E]);
    pub(crate) static ref ICE_ROD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x0, 0xCD01]);
    pub(crate) static ref SAND_ROD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x1, 0xCD01]);
    pub(crate) static ref TORNADO_ROD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x2, 0xCD01]);
    pub(crate) static ref BOMBS: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x3, 0xCD01]);
    pub(crate) static ref FIRE_ROD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x4, 0xCD01]);
    pub(crate) static ref HOOKSHOT: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x5, 0xCD01]);
    pub(crate) static ref BOOMERANG: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x6, 0xCD01]);
    pub(crate) static ref HAMMER: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x7, 0xCD01]);
    pub(crate) static ref BOW: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x8, 0xCD01]);
    pub(crate) static ref TRIFORCE_OF_COURAGE: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x46, 0xCD00]);
    pub(crate) static ref PENDANT_OF_COURAGE: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x0123, 0xCD00]);
    pub(crate) static ref MASTER_SWORD: String = utf16(&[0xE, 0x2, 0x2, 0x4, 0x012C, 0xCD00]);
}

fn utf16(v: &[u16]) -> String {
    String::from_utf16(v).unwrap()
}

#[allow(unused)]
pub(crate) fn big(text: &str) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64, 0xE, 0x0, 0x2, 0x2, 0x82]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
}

#[allow(unused)]
pub(crate) fn troll() -> String {
    purple(RAVIO.as_str())
}

/// Black Color: 0x262626FF
pub(crate) fn black(text: &str) -> String {
    color(text, 0x0)
}

/// Gray Color: 0x808080FF
pub(crate) fn gray(text: &str) -> String {
    color(text, 0x1)
}

/// White Color: 0xFFFFFFFF
pub(crate) fn white(text: &str) -> String {
    color(text, 0x2)
}

/// Beige Color: 0x855C2FFF
pub(crate) fn beige(text: &str) -> String {
    color(text, 0x3)
}

/// Red Color: 0x591710FF <br />
/// Note: [`attention`] is what's typically used in-game for red text.
pub(crate) fn red(text: &str) -> String {
    color(text, 0x4)
}

/// Green Color: 0x006400FF
pub(crate) fn green(text: &str) -> String {
    color(text, 0x5)
}

/// Blue Color: 0x375960FF <br />
/// Note: [`name`] is what's typically used in-game for blue text.
pub(crate) fn blue(text: &str) -> String {
    color(text, 0x6)
}

/// Yellow Text: 0xBAA800FF
/// Used for Yuga's general text
pub(crate) fn yellow(text: &str) -> String {
    color(text, 0x7)
}

/// Purple Text: 0x3A1B4CFF
pub(crate) fn purple(text: &str) -> String {
    color(text, 0x8)
}

/// Name Color: 0x003F97FF <br />
/// This blue coloring is what the game usually uses to highlight names/places/etc.
pub(crate) fn name(text: &str) -> String {
    color(text, 0x9)
}

/// Attention Color: 0xF92300FF <br />
/// The game typically uses this over the [`red`] color option.
pub(crate) fn attention(text: &str) -> String {
    color(text, 0xA)
}

/// YugaTalking Color: 0x4AF0D1FF <br />
/// This is a Cyan color used for highlighted words when Yuga is speaking.
pub(crate) fn yuga_talking(text: &str) -> String {
    color(text, 0xB)
}

fn color(text: &str, index: u16) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x3, 0x2, index]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x3, 0x2, 0xFFFF]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
}
