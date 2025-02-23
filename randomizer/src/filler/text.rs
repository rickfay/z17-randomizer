//! Text chosen randomly for a seed, usually flavor text.

use crate::Result;
use crate::Text;
use rand::prelude::StdRng;
use rand::Rng;

/// Text generation
pub(crate) fn generate(rng: &mut StdRng) -> Result<Text> {
    let credits = credits_text(rng);
    Ok(Text { credits })
}

/// Credits headline text, large red text between both Triforces
#[rustfmt::skip]
fn credits_text(rng: &mut StdRng) -> String {

    // Does support the new line character \n, but the 2nd line will render with the line through it
    // 2 lines desired: Use an extra \n at the beginning to make a 3 line comment
    // 3 lines works just fine.
    let choices = [
     // "THE LEGEND OF ZELDA", // Can't be longer than this
        "SIX WHOLE TRIANGLES",
        "GG",
        "YOUR AD HERE",
        "A WINNER IS YOU",
        "CONFORM.",
        "CAT",
        "LOOK, TWINSIES!!",
        "YOU DID THE THING",
        "OH HI",
        "hi",
        "COOL COOL COOL",
    ];

    String::from(choices[rng.gen_range(0..choices.len())])
}
