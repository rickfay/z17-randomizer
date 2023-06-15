use {crate::hints::formatting::*, HintColor::*};

/// The color to use when displaying a particular piece of hinted text.
pub enum HintColor {
    Black,
    Gray,
    White,
    Beige,
    Red,
    Green,
    Blue,
    Yellow,
    Purple,
    Name,
    Attention,
    YugaTalking,
}

impl HintColor {
    pub(crate) fn format(&self, text: &str) -> String {
        match self {
            Black => black(text),
            Gray => gray(text),
            White => white(text),
            Beige => beige(text),
            Red => red(text),
            Green => green(text),
            Blue => blue(text),
            Yellow => yellow(text),
            Purple => purple(text),
            Name => name(text),
            Attention => attention(text),
            YugaTalking => yuga_talking(text),
        }
    }
}
