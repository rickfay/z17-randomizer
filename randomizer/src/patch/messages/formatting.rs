use lazy_static::lazy_static;

// Symbols
lazy_static! {
    pub(crate) static ref SYMBOL_A_BUTTON: String = String::from_utf16(&[0xE000, 0x002E]).unwrap();
    pub(crate) static ref SYMBOL_B_BUTTON: String = unimplemented!();
    pub(crate) static ref SYMBOL_RAVIO: String = String::from_utf16(&[0xE05E]).unwrap();
    pub(crate) static ref SYMBOL_BOMBS: String = String::from_utf16(&[0xE06D]).unwrap();
    pub(crate) static ref SYMBOL_FIRE_ROD: String = String::from_utf16(&[0xE06E]).unwrap();
    pub(crate) static ref SYMBOL_BOW: String = String::from_utf16(&[0xE06C]).unwrap();
}

// Controls - Special purpose strings
lazy_static! {
    pub(crate) static ref CHOICE_2: String =
        String::from_utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD02]).unwrap();
    pub(crate) static ref CHOICE_3: String =
        String::from_utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD03]).unwrap();
    pub(crate) static ref CHOICE_4: String =
        String::from_utf16(&[0xE, 0x1, 0x6, 0x2, 0xCD04]).unwrap();
    pub(crate) static ref PRICE: String = utf16_str(&[0xE, 0x1, 0x5, 0x6, 0x0, 0xFFFF, 0xCD01]);
}

// Referenced Strings
lazy_static! {
    pub(crate) static ref PLAYER_NAME: String = String::from_utf16(&[0xE, 0x1, 0x0, 0x0]).unwrap();
    pub(crate) static ref SAHASRAHLA: String =
        String::from_utf16(&[0xE, 0x2, 0x0, 0x2, 0x0109]).unwrap();
    pub(crate) static ref HOUSE_OF_GALES: String =
        String::from_utf16(&[0xE, 0x2, 0x1, 0x4, 0x0101, 0xCD00]).unwrap();
    pub(crate) static ref TOWER_OF_HERA: String =
        String::from_utf16(&[0xE, 0x2, 0x1, 0x4, 0x0102, 0xCD00]).unwrap();
    pub(crate) static ref KAKARIKO_VILLAGE: String =
        String::from_utf16(&[0xE, 0x2, 0x1, 0x4, 0x010D, 0xCD2E]).unwrap();
    pub(crate) static ref TORNADO_ROD: String =
        String::from_utf16(&[0xE, 0x2, 0x2, 0x4, 0x0102, 0xCD00]).unwrap();
    pub(crate) static ref PENDANT_OF_COURAGE: String =
        String::from_utf16(&[0xE, 0x2, 0x2, 0x4, 0x0123, 0xCD00]).unwrap();
    pub(crate) static ref MASTER_SWORD: String =
        String::from_utf16(&[0xE, 0x2, 0x2, 0x4, 0x012C, 0xCD00]).unwrap();
}

fn utf16_str(v: &[u16]) -> String {
    String::from_utf16(v).unwrap()
}

pub(crate) fn big(text: &str) -> String {
    let prefix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64, 0xE, 0x0, 0x2, 0x2, 0x82]).unwrap();
    let suffix = String::from_utf16(&[0xE, 0x0, 0x2, 0x2, 0x64]).unwrap();
    format!("{}{}{}", prefix, text, suffix)
}

pub(crate) fn troll() -> String {
    purple(SYMBOL_RAVIO.as_str())
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
