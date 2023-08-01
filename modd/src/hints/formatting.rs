pub fn a_button() -> String {
    utf16(A_BUTTON)
}

pub fn b_button() -> String {
    utf16(B_BUTTON)
}

pub fn x_button() -> String {
    utf16(X_BUTTON)
}

pub fn y_button() -> String {
    utf16(Y_BUTTON)
}

pub fn l_button() -> String {
    utf16(L_BUTTON)
}

pub fn r_button() -> String {
    utf16(R_BUTTON)
}

pub fn ravio() -> String {
    utf16(RAVIO)
}

pub fn bow() -> String {
    utf16(BOW)
}

pub fn bombs() -> String {
    utf16(BOMBS)
}

pub fn fire_rod() -> String {
    utf16(FIRE_ROD)
}

pub fn price() -> String {
    utf16(PRICE)
}

pub fn choice_2() -> String {
    utf16(CHOICE_2)
}

// Font Symbols - See US/MessageFont.bffnt for mapping and character, glyph, and offset info.
pub const A_BUTTON: &[u16] = &[0xE000];
pub const B_BUTTON: &[u16] = &[0xE001];
pub const X_BUTTON: &[u16] = &[0xE002];
pub const Y_BUTTON: &[u16] = &[0xE003];
pub const L_BUTTON: &[u16] = &[0xE004];
pub const R_BUTTON: &[u16] = &[0xE005];
pub const DPAD: &[u16] = &[0xE006];
pub const DIAMOND: &[u16] = &[0xE016];
pub const HEART: &[u16] = &[0xE017];
pub const ARROW_R: &[u16] = &[0xE019];
pub const ARROW_D: &[u16] = &[0xE01C];
pub const CAROT_L: &[u16] = &[0xE036];
pub const CAROT_R: &[u16] = &[0xE037];
pub const PLUS: &[u16] = &[0xE045];
pub const MINUS: &[u16] = &[0xE046];
pub const RAVIO: &[u16] = &[0xE05E];
pub const E: &[u16] = &[0xE06A];
pub const BOW: &[u16] = &[0xE06C];
pub const BOMBS: &[u16] = &[0xE06D];
pub const FIRE_ROD: &[u16] = &[0xE06E];
pub const DPAD_D: &[u16] = &[0xE07A];
pub const DPAD_R: &[u16] = &[0xE07C];
pub const BANG: &[u16] = &[0xFF01];
pub const QUESTION: &[u16] = &[0xFF1F];
pub const TILDE: &[u16] = &[0xFF5E];

// Controls - Special purpose strings
pub const CHOICE_2: &[u16] = &[0xE, 0x1, 0x6, 0x2, 0xCD02];
pub const CHOICE_3: &[u16] = &[0xE, 0x1, 0x6, 0x2, 0xCD03];
pub const CHOICE_4: &[u16] = &[0xE, 0x1, 0x6, 0x2, 0xCD04];
pub const PRICE: &[u16] = &[0xE, 0x1, 0x5, 0x6, 0x0, 0xFFFF, 0xCD01];

// Referenced Strings
pub const PLAYER_NAME: &[u16] = &[0xE, 0x1, 0x0, 0x0];
pub const SAHASRAHLA: &[u16] = &[0xE, 0x2, 0x0, 0x2, 0x0109];
pub const HOUSE_OF_GALES: &[u16] = &[0xE, 0x2, 0x1, 0x4, 0x0101, 0xCD00];
pub const TOWER_OF_HERA: &[u16] = &[0xE, 0x2, 0x1, 0x4, 0x0102, 0xCD00];
pub const KAKARIKO_VILLAGE: &[u16] = &[0xE, 0x2, 0x1, 0x4, 0x010D, 0xCD2E];
pub const TORNADO_ROD: &[u16] = &[0xE, 0x2, 0x2, 0x4, 0x0102, 0xCD00];
pub const PENDANT_OF_COURAGE: &[u16] = &[0xE, 0x2, 0x2, 0x4, 0x0123, 0xCD00];
pub const MASTER_SWORD: &[u16] = &[0xE, 0x2, 0x2, 0x4, 0x012C, 0xCD00];

fn utf16(v: &[u16]) -> String {
    String::from_utf16(v).unwrap()
}

#[allow(unused)]
pub fn big(text: &str) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64, 0xE, 0x0, 0x2, 0x2, 0x82]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
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
