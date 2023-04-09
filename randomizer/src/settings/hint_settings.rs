use {
    crate::{fail, settings::hint_settings::HintGhostPrice::*},
    rand::{rngs::StdRng, Rng},
    serde::{
        de::{Error, Visitor},
        Deserialize, Deserializer, Serialize,
    },
    std::{
        fmt::{Display, Formatter},
        str::FromStr,
    },
};

pub const HINT_GHOST_PRICE_MAX: u32 = 9999;
pub const HINT_GHOST_PRICE_RANDOM_MIN: u32 = 0;
pub const HINT_GHOST_PRICE_RANDOM_MAX: u32 = 150;

/// Price setting for Hints purchased from Hint Ghosts
#[derive(Debug, Copy, Clone, Eq, Hash, PartialOrd, PartialEq, Serialize)]
pub enum HintGhostPrice {
    /// Free Hints, skips the "Buy a Hint" dialog
    Free,
    /// Hints for Sale at a static price value
    Price(u32),
    /// Hints for sale at a random price between a set min and max range
    Random(u32, u32),
}

impl HintGhostPrice {
    /// Gets the price for Hints
    pub fn get(&self) -> u32 {
        match self {
            Free => 0,
            Price(price) => *price,
            Random(_, _) => fail!("HintGhostPrice::get is not valid for variant: Random"),
        }
    }

    /// Gets a random price for a Hint between the set min and max, using the provided [`StdRng`].
    pub fn rand(self, rng: &mut StdRng) -> u32 {
        if let Random(min, max) = self {
            rng.gen_range(min..=max)
        } else {
            fail!("HintGhostPrice::rand is not valid for variant: {:?}", self)
        }
    }
}

impl Display for HintGhostPrice {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Free => "Free".to_owned(),
            Price(price) => format!("{} {}", price, if *price == 1 { "Rupee" } else { "Rupees" }),
            Random(min, max) => format!("Random ({}-{})", min, max),
        })
    }
}

impl Default for HintGhostPrice {
    fn default() -> Self {
        Price(50)
    }
}

struct HintGhostPriceDeserializer;

// fixme this doesn't actually support a non-string number in JSON, this isn't a good approach
impl<'de> Visitor<'de> for HintGhostPriceDeserializer {
    type Value = HintGhostPrice;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("\"Free\", \"Random\", or an integer between 0 and 9999.")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let value = value.to_uppercase();
        let value = value.as_str();
        match value {
            "FREE" => Ok(Free),
            "RANDOM" => Ok(Random(HINT_GHOST_PRICE_RANDOM_MIN, HINT_GHOST_PRICE_RANDOM_MAX)),
            _ => match u32::from_str(value) {
                Ok(value) => {
                    if value == 0 {
                        Ok(Free)
                    } else if (1..=HINT_GHOST_PRICE_MAX).contains(&value) {
                        Ok(Price(value))
                    } else {
                        fail!(
                            "Hint Ghost Price \"{}\" was not in range: 0..={}",
                            value,
                            HINT_GHOST_PRICE_MAX
                        )
                    }
                }
                Err(_) => fail!(
                    "Hint Ghost Price could not be parsed from: \"{}\"\n\n\
                    Valid values for the Hint Ghost Price are: \"Free\", \"Random\", or an integer in the range: 0..={}",
                    value,
                    HINT_GHOST_PRICE_MAX
                ),
            },
        }
    }
}

impl<'de> Deserialize<'de> for HintGhostPrice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(HintGhostPriceDeserializer)
    }
}
