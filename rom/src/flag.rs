#[derive(Debug, Copy, Clone)]
pub enum Flag {
    React(u16),   // 0 - Reactions with system objects, not persisted
    Session(u16), // 1 - Flag persists until game is reset (I think?)
    Two(u16),     // 2 - ???
    Course(u16),  // 3 - Course-specific, shared between scenes of the same course
    Event(u16),   // 4 - Global
}

impl Flag {
    pub fn get_type(self) -> u8 {
        match self {
            Flag::React(_) => 0,
            Flag::Session(_) => 1,
            Flag::Two(_) => 2,
            Flag::Course(_) => 3,
            Flag::Event(_) => 4,
        }
    }

    pub fn get_value(self) -> u16 {
        match self {
            Flag::React(flag) => flag,
            Flag::Session(flag) => flag,
            Flag::Two(flag) => flag,
            Flag::Course(flag) => flag,
            Flag::Event(flag) => flag,
        }
    }

    pub fn into_pair(self) -> (u8, u16) {
        match self {
            Flag::React(flag) => (0, flag),
            Flag::Session(flag) => (1, flag),
            Flag::Two(flag) => (2, flag),
            Flag::Course(flag) => (3, flag),
            Flag::Event(flag) => (4, flag),
        }
    }
}

macro_rules! event_flags {
    (
        $(#[$attr:meta])*
        $($index:literal: $flag:ident,)+
    ) => {
        $(#[$attr])*
        $(pub const $flag: Flag = Flag::Event($index);)+
    }
}

impl Flag {
    event_flags! {
        730: CREDITS,

        920: WV_YOUR_HOUSE,
        921: WV_KAKARIKO_VILLAGE,
        922: WV_EASTERN_PALACE,
        923: WV_HOUSE_OF_GALES,
        924: WV_TOWER_OF_HERA,
        925: WV_WITCHS_HOUSE,
        926: WV_DEATH_MTN_HYRULE,
        927: WV_DESERT_PALACE,
        928: WV_SANCTUARY,

        932: WV_SKULL_WOODS,
        933: WV_TREACHEROUS_TOWER,
        934: WV_ICE_RUINS,
        935: WV_LORULE_CASTLE,
        936: WV_GRAVEYARD,
        937: WV_THIEVES_TOWN,
        938: WV_DARK_PALACE,
        939: WV_BLACKSMITH,
        940: WV_VACANT_HOUSE,
        941: WV_MISERY_MIRE,
        942: WV_SWAMP_PALACE,
        943: WV_TURTLE_ROCK,
        944: WV_DEATH_MTN_LORULE,
    }

    pub fn get_hyrule_weather_vane_flags() -> Vec<Flag> {
        vec![
            Flag::WV_YOUR_HOUSE,
            Flag::WV_KAKARIKO_VILLAGE,
            Flag::WV_EASTERN_PALACE,
            Flag::WV_HOUSE_OF_GALES,
            Flag::WV_TOWER_OF_HERA,
            Flag::WV_WITCHS_HOUSE,
            Flag::WV_DEATH_MTN_HYRULE,
            Flag::WV_DESERT_PALACE,
            Flag::WV_SANCTUARY,
        ]
    }

    pub fn get_lorule_weather_vane_flags() -> Vec<Flag> {
        vec![
            Flag::WV_SKULL_WOODS,
            Flag::WV_TREACHEROUS_TOWER,
            Flag::WV_ICE_RUINS,
            Flag::WV_LORULE_CASTLE,
            Flag::WV_GRAVEYARD,
            Flag::WV_THIEVES_TOWN,
            Flag::WV_DARK_PALACE,
            Flag::WV_BLACKSMITH,
            Flag::WV_VACANT_HOUSE,
            Flag::WV_MISERY_MIRE,
            Flag::WV_SWAMP_PALACE,
            Flag::WV_TURTLE_ROCK,
            Flag::WV_DEATH_MTN_LORULE,
        ]
    }

    pub fn get_all_weather_vane_flags() -> Vec<Flag> {
        let mut flags = Vec::with_capacity(22);
        flags.append(&mut Flag::get_hyrule_weather_vane_flags());
        flags.append(&mut Flag::get_lorule_weather_vane_flags());
        flags
    }
}
