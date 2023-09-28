use std::{
    borrow::Cow,
    fmt::{self, Display, Formatter, Write},
};

use serde::Serialize;

#[derive(Debug)]
pub struct Text<'a> {
    parts: Vec<Part<'a>>,
}

impl<'a> Text<'a> {
    pub fn build() -> TextBuilder<'a> {
        Default::default()
    }

    pub fn to_game_text(&self) -> String {
        self.parts.iter().fold(String::new(), |mut text, part| {
            part.write(&mut text);
            text
        })
    }

    pub fn to_string(&self) -> Option<String> {
        self.parts.iter().map(Part::to_string).collect()
    }

    pub fn to_owned(&self) -> Text<'static> {
        Text { parts: self.parts.iter().map(Part::to_owned).collect() }
    }
}

impl Serialize for Text<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string().expect("a valid UTF-8 string"))
    }
}

#[derive(Debug, Default)]
pub struct TextBuilder<'a> {
    parts: Vec<Part<'a>>,
}

impl<'a> TextBuilder<'a> {
    pub fn text(mut self, text: &'a str) -> Self {
        self.parts.push(Part::Raw(text.into()));
        self
    }

    pub fn colored(mut self, colored: Colored<'a>) -> Self {
        self.parts.push(Part::Colored(colored));
        self
    }

    pub fn control(mut self, control: Control) -> Self {
        self.parts.push(Part::Control(control));
        self
    }

    pub fn finish(self) -> Text<'a> {
        Text { parts: self.parts }
    }
}

#[derive(Clone, Debug)]
pub enum Part<'a> {
    Raw(Cow<'a, str>),
    Colored(Colored<'a>),
    Control(Control),
}

impl<'a> Part<'a> {
    fn write(&self, string: &mut String) {
        match self {
            Self::Raw(raw) => {
                string.push_str(raw);
            }
            Self::Colored(colored) => {
                write!(string, "{}", colored).expect("the string to be written");
            }
            Self::Control(control) => control.write(string),
        }
    }

    fn to_string(&self) -> Option<String> {
        match self {
            Self::Raw(text) | Self::Colored(Colored { text, .. }) => Some(text.replace('\n', " ")),
            Self::Control(_) => None,
        }
    }

    fn to_owned(&self) -> Part<'static> {
        match self {
            Self::Raw(raw) => Part::Raw(Cow::Owned(raw.clone().into_owned())),
            Self::Colored(colored) => Part::Colored(colored.to_owned()),
            Self::Control(control) => Part::Control(*control),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Colored<'a> {
    color: Color,
    text: Cow<'a, str>,
}

impl<'a> Colored<'a> {
    pub fn new(color: Color, text: &'a str) -> Self {
        Self { color, text: text.into() }
    }

    fn to_owned(&self) -> Colored<'static> {
        Colored { color: self.color, text: Cow::Owned(self.text.clone().into_owned()) }
    }
}

impl<'a> Display for Colored<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for ch in decode_utf16([0x000E, 0x0000, 0x0003, 0x0002, self.color as u16]) {
            ch.fmt(f)?;
        }
        self.text.fmt(f)?;
        for ch in decode_utf16(COLOR_SUFFIX) {
            ch.fmt(f)?;
        }
        Ok(())
    }
}

/// The color to use when displaying a particular piece of text.
#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Color {
    Black = 0x0,
    Gray = 0x1,
    White = 0x2,
    Beige = 0x3,
    Red = 0x4,
    Green = 0x5,
    Blue = 0x6,
    Yellow = 0x7,
    Purple = 0x8,
    Name = 0x9,
    Attention = 0xA,
    Cyan = 0xB,
}

#[derive(Clone, Copy, Debug)]
#[repr(u16)]
pub enum Symbol {
    AButton = 0xE000,
    BButton = 0xE001,
    XButton = 0xE002,
    YButton = 0xE003,
    LButton = 0xE004,
    RButton = 0xE005,
    DPad = 0xE006,
    Diamond = 0xE016,
    Heart = 0xE017,
    ArrowR = 0xE019,
    ArrowD = 0xE01C,
    CarotL = 0xE036,
    CarotR = 0xE037,
    Plus = 0xE045,
    Minus = 0xE046,
    Ravio = 0xE05E,
    E = 0xE06A,
    Bow = 0xE06C,
    Bombs = 0xE06D,
    FireRod = 0xE06E,
    DpadD = 0xE07A,
    DpadR = 0xE07C,
    Bang = 0xFF01,
    Question = 0xFF1F,
    Tilde = 0xFF5E,
}

impl Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        char::from_u32(*self as u32).expect("a valid UTF-16 char").fmt(f)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Control {
    Choice2,
    Choice3,
    Choice4,
    Price,
}

impl Control {
    fn write(&self, string: &mut String) {
        write!(string, "{}", self).expect("the control to be written");
    }
}

impl Display for Control {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(choice) = match self {
            Control::Choice2 => Some(2),
            Control::Choice3 => Some(3),
            Control::Choice4 => Some(4),
            Control::Price => None,
        } {
            for ch in decode_utf16([0x000E, 0x0001, 0x0006, 0x0002, 0xCD00 | choice]) {
                ch.fmt(f)?;
            }
        } else {
            for ch in decode_utf16([0x000E, 0x0001, 0x0005, 0x0006, 0x0000, 0xFFFF, 0xCD01]) {
                ch.fmt(f)?;
            }
        }
        Ok(())
    }
}

fn decode_utf16(iter: impl IntoIterator<Item = u16>) -> impl Iterator<Item = char> {
    char::decode_utf16(iter).map(|ch| ch.expect("a valid UTF-16 codepoint"))
}

const COLOR_SUFFIX: [u16; 5] = [0x000E, 0x0000, 0x0003, 0x0002, 0xFFFF];
