use crate::{hints::hint_color::HintColor, patch::Patcher, SeedInfo};
use log::info;
use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

pub struct Subregion {
    name: &'static str,
    color: HintColor,
    world: World,
    id: &'static str,
}

impl Subregion {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn name_colorized(&self) -> String {
        self.color.format(self.name)
    }

    pub fn world(&self) -> World {
        self.world
    }
}

impl Debug for Subregion {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("Subregion").field("name", &self.name).field("world", &self.world).field("id", &self.id).finish()
    }
}

impl Eq for Subregion {}

impl PartialEq for Subregion {
    fn eq(&self, other: &Self) -> bool {
        self.world == other.world && self.name == other.name && self.id == other.id
    }
}

impl Hash for Subregion {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.world.hash(state);
        self.name.hash(state);
        self.id.hash(state);
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum World {
    Hyrule,
    Lorule,
    Dungeons,
}

pub mod dungeons {
    pub const WORLD: super::World = super::World::Dungeons;
    pub mod dark;
    pub mod desert;
    pub mod eastern;
    pub mod graveyards;
    pub mod house;
    pub mod hyrule;
    pub mod ice;
    pub mod lorule;
    pub mod skull;
    pub mod swamp;
    pub mod thieves;
    pub mod tower;
    pub mod turtle;
}
pub mod hyrule {
    pub const WORLD: super::World = super::World::Hyrule;
    pub mod death;
    pub mod desert;
    pub mod eastern;
    pub mod field;
    pub mod irene;
    pub mod kakariko;
    pub mod lake;
    pub mod lost;
    pub mod ravio;
    pub mod southern;
    pub mod zora;
}
pub mod lorule {
    pub const WORLD: super::World = super::World::Lorule;
    pub mod chamber;
    pub mod dark;
    pub mod death;
    pub mod field;
    pub mod lake;
    pub mod misery;
    pub mod skull;
}

pub(crate) fn patch(patcher: &mut Patcher, seed_info: &SeedInfo) -> crate::Result<()> {
    info!("Patching Randomized Checks...");

    // todo unravel this
    dungeons::dark::patch(patcher, seed_info)?;
    dungeons::desert::patch(patcher, seed_info)?;
    dungeons::eastern::patch(patcher, seed_info)?;
    dungeons::graveyards::patch(patcher, seed_info)?;
    dungeons::house::patch(patcher, seed_info)?;
    dungeons::hyrule::patch(patcher, seed_info)?;
    dungeons::ice::patch(patcher, seed_info)?;
    dungeons::lorule::patch(patcher, seed_info)?;
    dungeons::skull::patch(patcher, seed_info)?;
    dungeons::swamp::patch(patcher, seed_info)?;
    dungeons::thieves::patch(patcher, seed_info)?;
    dungeons::tower::patch(patcher, seed_info)?;
    dungeons::turtle::patch(patcher, seed_info)?;

    hyrule::death::patch(patcher, seed_info)?;
    hyrule::desert::patch(patcher, seed_info)?;
    hyrule::eastern::patch(patcher, seed_info)?;
    hyrule::field::patch(patcher, seed_info)?;
    hyrule::irene::patch(patcher, seed_info)?;
    hyrule::kakariko::patch(patcher, seed_info)?;
    hyrule::lake::patch(patcher, seed_info)?;
    hyrule::lost::patch(patcher, seed_info)?;
    hyrule::ravio::patch(patcher, seed_info)?;
    hyrule::southern::patch(patcher, seed_info)?;
    hyrule::zora::patch(patcher, seed_info)?;

    lorule::chamber::patch(patcher, seed_info)?;
    lorule::dark::patch(patcher, seed_info)?;
    lorule::death::patch(patcher, seed_info)?;
    lorule::field::patch(patcher, seed_info)?;
    lorule::lake::patch(patcher, seed_info)?;
    lorule::misery::patch(patcher, seed_info)?;
    lorule::skull::patch(patcher, seed_info)?;
    Ok(())
}

#[doc(hidden)]
#[macro_export]
macro_rules! region {
    (
        course: $course:ident,
        name: $name:literal,
        color: $color:ident,
        $start:ident $start_props:tt,
        $($id:ident $props:tt,)*
    ) => {

        #[inline]
        pub fn patch(patcher: &mut $crate::patch::Patcher, seed_info: &$crate::SeedInfo) -> $crate::Result<()> {
            $start::patch(patcher, seed_info)?;
            $($id::patch(patcher, seed_info)?;)*
            Ok(())
        }

        $crate::subregion!($start $start_props);
        $($crate::subregion!($id $props);)*

        #[allow(unused)]
        pub(crate) fn start() -> &'static $crate::regions::Subregion {
            $start::SUBREGION
        }

        pub const NAME: &str = $name;
        pub const COLOR: $crate::hints::hint_color::HintColor = $crate::hints::hint_color::HintColor::$color;
        #[allow(unused)]
        pub const COURSE: ::game::Course = ::game::Course::$course;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! subregion {
    ($id:ident {
        $(locations: [
            $($key:literal: $item:ident @$variant:ident $props:tt $(:- $condition:tt)?
                $(where $settings:ident: $where:expr)?,)*
        ],)?
        $(paths: [
            $($path:ident$(::$path_rest:ident)* $(:- $pcondition:tt)?,)*
        ],)?
        $(quest: $kind:ident$(::$qvariant:ident)?,)?
    }) => {
        pub mod $id {

            use $crate::{patch::Patcher, regions::Subregion};

            pub use super::COURSE;

            pub const SUBREGION: &Subregion = &Subregion {
                name: super::NAME,
                color: super::COLOR,
                world: super::super::WORLD,
                id: stringify!($id),
            };

            #[allow(unused)]
            #[inline]
            pub fn patch(patcher: &mut Patcher, seed_info: &$crate::SeedInfo) -> $crate::Result<()> {
                $(use $crate::patch::Patch;
                $($crate::patch!($variant $props).apply(
                    patcher,
                    seed_info,
                    seed_info.layout
                        .get(&$crate::LocationInfo::new($key, SUBREGION))
                )?;)*)?
                Ok(())
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! patch {
    (Chest($course:ident $stage:literal[$unq:literal])) => {
        Patch::Chest { course: ::game::Course::$course, stage: $stage - 1, unq: $unq }
    };
    (Chest($stage:literal[$unq:literal])) => {
        Patch::Chest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Chest[$($stage:literal[$unq:literal],)+]) => {
        Patch::Multi(vec![
            $(
                Patch::Chest { course: COURSE, stage: $stage - 1, unq: $unq },
            )+
        ])
    };
    (BigChest($course:ident $stage:literal[$unq:literal])) => {
        Patch::BigChest { course: ::game::Course::$course, stage: $stage - 1, unq: $unq }
    };
    (BigChest($stage:literal[$unq:literal])) => {
        Patch::BigChest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Event($name:ident[$index:literal])) => {
        Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index }
    };
    (Event(Boot/$name:ident[$index:literal])) => {
        Patch::Event { course: None, name: stringify!($name), index: $index }
    };
    (Event($course:ident/$name:ident[$index:literal])) => {
        Patch::Event { course: Some(::game::Course::$course), name: stringify!($name), index: $index }
    };
    (Event[$($name:ident[$index:literal],)+]) => {
        Patch::Multi(vec![
            $(
                Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index },
            )+
        ])
    };
    (Heart($course:ident $scene:literal[$unq:literal])) => {
        Patch::Heart { course: ::game::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Heart($scene:literal[$unq:literal])) => {
        Patch::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Key($course:ident $scene:literal[$unq:literal])) => {
        Patch::Key { course: ::game::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Key($scene:literal[$unq:literal])) => {
        Patch::Key { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($course:ident $scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: ::game::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: ::game::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: ::game::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Portal($course:ident $scene:literal[$unq:literal] $portal:ident)) => {
        Patch::Portal { course: ::game::Course::$course, scene: $scene - 1, unq: $unq, portal: crate::Portal::$portal }
    };
    (Portal($scene:literal[$unq:literal] $portal:ident)) => {
        Patch::Portal { course: COURSE, scene: $scene - 1, unq: $unq, portal: crate::Portal::$portal }
    };
    (WeatherVane($course:ident $scene:literal[$unq:literal] $vane:ident)) => {
        Patch::WeatherVane { course: ::game::Course::$course, scene: $scene - 1, unq: $unq, vane: crate::filler::filler_item::Vane::$vane }
    };
    (WeatherVane($scene:literal[$unq:literal] $vane:ident)) => {
        Patch::WeatherVane { course: COURSE, scene: $scene - 1, unq: $unq, vane: crate::filler::filler_item::Vane::$vane }
    };
    (Shop($variant:ident$($args:tt)?)) => {
        Patch::Shop($crate::patch::Shop::$variant $($args)?)
    };
    (None()) => {
        Patch::None
    }
}
