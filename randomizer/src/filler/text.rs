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
fn credits_text(rng: &mut StdRng) -> String {
    let choices = [
        // "THE LEGEND OF ZELDA", // Can't be longer than this
        "GG",
        "YOUR AD HERE",
        "A WINNER IS YOU",
        "AT LEAST IT'S OVER",
        "CONFORM",
        "CAT",
        "LOOK, TWINSIES!!",
        "YOU DID THE THING",
        "OH HI",
        "WE'RE MULTIPLYING",
        "hi",
        "COOL COOL COOL",
    ];

    String::from(choices[rng.gen_range(0..choices.len())])
}
