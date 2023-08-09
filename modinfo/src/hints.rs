use std::collections::HashMap;

use game::HintGhost;
use serde::{ser::SerializeMap, Serialize};

use crate::text::Text;

type HintMap = HashMap<HintGhost, Text<'static>>;

#[derive(Debug, Default, Serialize)]
pub struct Hints {
    #[serde(serialize_with = "serialize_ghosts")]
    pub ghosts: HintMap,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bow_of_light: Option<Text<'static>>,
}

impl Hints {
    pub fn is_empty(&self) -> bool {
        self.ghosts.is_empty() && self.bow_of_light.is_none()
    }
}

fn serialize_ghosts<S>(ghosts: &HintMap, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut map = serializer.serialize_map(Some(ghosts.len()))?;
    for (ghost, text) in ghosts.iter() {
        map.serialize_entry(hint_ghost_name(ghost), text)?;
    }
    map.end()
}

pub fn hint_ghost_name(ghost: &HintGhost) -> &'static str {
    match ghost {
        HintGhost::LostWoodsMaze1 => "Lost Woods Maze Ghost 1",
        HintGhost::LostWoodsMaze2 => "Lost Woods Maze Ghost 2",
        HintGhost::LostWoodsMaze3 => "Lost Woods Maze Ghost 3",
        HintGhost::LostWoods => "Lost Woods Ghost",
        HintGhost::SpectacleRock => "Spectacle Rock Ghost",
        HintGhost::TowerOfHeraOutside => "Outside Tower of Hera Ghost",
        HintGhost::FloatingIsland => "Floating Island Ghost",
        HintGhost::FireCave => "Fire Cave Ghost",
        HintGhost::MoldormCave => "Moldorm Cave Ghost",
        HintGhost::ZorasDomain => "Zora's Domain Ghost",
        HintGhost::FortuneTellerHyrule => "Hyrule Fortune-Teller Ghost",
        HintGhost::Sanctuary => "Sanctuary Ghost",
        HintGhost::GraveyardHyrule => "Hyrule Graveyard Ghost",
        HintGhost::WaterfallCave => "Waterfall Cave Ghost",
        HintGhost::Well => "Kakariko Well Ghost",
        HintGhost::ShadyGuy => "Shady Guy Ghost",
        HintGhost::StylishWoman => "Stylish Woman Ghost",
        HintGhost::BlacksmithCave => "Blacksmith Cave Ghost",
        HintGhost::EasternRuinsPegs => "Eastern Ruins Pegs Ghost",
        HintGhost::EasternRuinsCave => "Eastern Ruins Cave Ghost",
        HintGhost::EasternRuinsEntrance => "Eastern Ruins Entrance Ghost",
        HintGhost::RupeeRushHyrule => "Hyrule Rupee Rush Ghost",
        HintGhost::Cuccos => "Dodge the Cuccos Ghost",
        HintGhost::SouthBridge => "Southern Bridge Ghost",
        HintGhost::SouthernRuins => "Southern Ruins Ghost",
        HintGhost::HouseOfGalesIsland => "House of Gales Island Ghost",
        HintGhost::HyruleHotfoot => "Hyrule Hotfoot Ghost",
        HintGhost::Letter => "Letter in a Bottle Ghost",
        HintGhost::StreetPassTree => "StreetPass Tree Ghost",
        HintGhost::BlacksmithBehind => "Behind Blacksmith Ghost",
        HintGhost::GraveyardLedge => "Graveyard Ledge Ghost",
        HintGhost::DesertEast => "Desert East Ghost",
        HintGhost::DesertCenter => "Desert Center Ghost",
        HintGhost::DesertSouthWest => "Desert South West Ghost",
        HintGhost::HyruleCastleRocks => "Hyrule Castle Rocks Ghost",
        HintGhost::WitchsHouse => "Witch's House Ghost",

        HintGhost::SkullWoodsCuccos => "Skull Woods Cuccos Ghost",
        HintGhost::TreacherousTower => "Treacherous Tower Ghost",
        HintGhost::IceRuinsOutside => "Ice Ruins Outside Ghost",
        HintGhost::LoruleGraveyard => "Lorule Graveyard Ghost",
        HintGhost::DarkRuinsNorth => "Dark Ruins North Ghost",
        HintGhost::SkullWoodsSouth => "Skull Woods South Ghost",
        HintGhost::FortunesChoice => "Fortune's Choice Ghost",
        HintGhost::VeteranThief => "Veteran Thief Ghost",
        HintGhost::FortuneTellerLorule => "Lorule Fortune-Teller Ghost",
        HintGhost::DarkMaze => "Dark Maze Ghost",
        HintGhost::RupeeRushLorule => "Lorule Rupee Rush Ghost",
        HintGhost::GreatRupeeFairy => "Great Rupee Fairy Ghost",
        HintGhost::OctoballDerby => "Octoball Derby Ghost",
        HintGhost::VacantHouse => "Vacant House Ghost",
        HintGhost::MiseryMireLedge => "Misery Mire Ledge Ghost",
        HintGhost::SwampPalaceOutsideLeft => "Swamp Palace Outside Left Ghost",
        HintGhost::TurtleBullied => "Turtle Bullied Ghost",
        HintGhost::TurtleWall => "Turtle Wall Ghost",
        HintGhost::TurtleRockOutside => "Turtle Rock Outside Ghost",
        HintGhost::DarkPalaceOutside => "Dark Palace Outside Ghost",
        HintGhost::SwampPalaceOutsideRight => "Swamp Palace Outside Right Ghost",
        HintGhost::MiseryMireBridge => "Misery Mire Bridge Ghost",
    }
}
