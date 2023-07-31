use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use log::info;

use crate::{hints::hint_color::HintColor, patch::Patcher, Settings};

pub struct AreaInfo {
    name: &'static str,
    color: HintColor,
    group: Group,
    id: &'static str,
}

impl AreaInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn name_colorized(&self) -> String {
        self.color.format(self.name)
    }

    pub fn world(&self) -> Group {
        self.group
    }
}

impl Debug for AreaInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("AreaInfo")
            .field("name", &self.name)
            .field("world", &self.group)
            .field("id", &self.id)
            .finish()
    }
}

impl Eq for AreaInfo {}

impl PartialEq for AreaInfo {
    fn eq(&self, other: &Self) -> bool {
        self.group == other.group && self.name == other.name && self.id == other.id
    }
}

impl Hash for AreaInfo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.group.hash(state);
        self.name.hash(state);
        self.id.hash(state);
    }
}

pub type Area = &'static AreaInfo;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Group {
    Hyrule,
    Lorule,
    Dungeons,
}

// macro_rules! regions {
//     ($($world:ident($variant:ident) {
//         $($region:ident;)+
//     })+) => {
//         use crate::patch::Patcher;
//
//         $(pub(crate) mod $world {
//             pub const WORLD: super::World = super::World::$variant;
//             $(pub(crate) mod $region;)+
//         })+
//
//         pub(crate) fn patch(patcher: &mut Patcher, layout: &crate::Layout, settings: &$crate::Settings) -> crate::Result<()> {
//             $($($world::$region::patch(patcher, layout, settings)?;)+)+
//             Ok(())
//         }
//     };
// }

pub mod dungeons {
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

    pub const GROUP: super::Group = super::Group::Dungeons;
}

pub mod hyrule {
    pub mod death;
    pub mod desert;
    pub mod eastern;
    pub mod field;
    pub mod irene;
    pub mod kakariko;
    pub mod lake;
    pub mod lost;
    pub mod southern;
    pub mod zora;

    pub const GROUP: super::Group = super::Group::Hyrule;
}

pub mod lorule {
    pub mod chamber;
    pub mod dark;
    pub mod death;
    pub mod field;
    pub mod lake;
    pub mod misery;
    pub mod skull;

    pub const GROUP: super::Group = super::Group::Lorule;
}

pub(crate) fn patch(
    patcher: &mut Patcher, layout: &crate::Layout, settings: &Settings,
) -> crate::Result<()> {
    info!("Patching Randomized Checks...");

    // todo unravel this
    dungeons::dark::patch(patcher, layout, settings)?;
    dungeons::desert::patch(patcher, layout, settings)?;
    dungeons::eastern::patch(patcher, layout, settings)?;
    dungeons::graveyards::patch(patcher, layout, settings)?;
    dungeons::house::patch(patcher, layout, settings)?;
    dungeons::hyrule::patch(patcher, layout, settings)?;
    dungeons::ice::patch(patcher, layout, settings)?;
    dungeons::lorule::patch(patcher, layout, settings)?;
    dungeons::skull::patch(patcher, layout, settings)?;
    dungeons::swamp::patch(patcher, layout, settings)?;
    dungeons::thieves::patch(patcher, layout, settings)?;
    dungeons::tower::patch(patcher, layout, settings)?;
    dungeons::turtle::patch(patcher, layout, settings)?;
    hyrule::death::patch(patcher, layout, settings)?;
    hyrule::desert::patch(patcher, layout, settings)?;
    hyrule::eastern::patch(patcher, layout, settings)?;
    hyrule::field::patch(patcher, layout, settings)?;
    hyrule::irene::patch(patcher, layout, settings)?;
    hyrule::kakariko::patch(patcher, layout, settings)?;
    hyrule::lake::patch(patcher, layout, settings)?;
    hyrule::lost::patch(patcher, layout, settings)?;
    hyrule::southern::patch(patcher, layout, settings)?;
    hyrule::zora::patch(patcher, layout, settings)?;
    lorule::chamber::patch(patcher, layout, settings)?;
    lorule::dark::patch(patcher, layout, settings)?;
    lorule::death::patch(patcher, layout, settings)?;
    lorule::field::patch(patcher, layout, settings)?;
    lorule::lake::patch(patcher, layout, settings)?;
    lorule::misery::patch(patcher, layout, settings)?;
    lorule::skull::patch(patcher, layout, settings)?;
    Ok(())
}

// regions! {
//     dungeons(Dungeons) {
//         dark;
//         desert;
//         eastern;
//         graveyards;
//         house;
//         hyrule;
//         ice;
//         lorule;
//         skull;
//         swamp;
//         thieves;
//         tower;
//         turtle;
//     }
//     hyrule(Hyrule) {
//         death;
//         desert;
//         eastern;
//         field;
//         irene;
//         kakariko;
//         lake;
//         lost;
//         southern;
//         zora;
//     }
//     lorule(Lorule) {
//         chamber;
//         dark;
//         death;
//         field;
//         lake;
//         misery;
//         skull;
//     }
// }

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
        pub fn patch(patcher: &mut $crate::patch::Patcher, layout: &$crate::Layout, settings: &$crate::Settings) -> $crate::Result<()> {
            $start::patch(patcher, layout, settings)?;
            $($id::patch(patcher, layout, settings)?;)*
            Ok(())
        }

        $crate::area!($start $start_props);
        $($crate::area!($id $props);)*

        #[allow(unused)]
        pub(crate) fn start() -> $crate::regions::Area {
            $start::AREA
        }

        pub const NAME: &str = $name;
        pub const COLOR: $crate::hints::hint_color::HintColor = $crate::hints::hint_color::HintColor::$color;
        #[allow(unused)]
        pub const COURSE: albw::course::Id = albw::course::Id::$course;
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! area {
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
            use $crate::{patch::Patcher, regions::{Area, AreaInfo}};

            pub use super::COURSE;

            pub const AREA: Area = &AreaInfo {
                name: super::NAME,
                color: super::COLOR,
                group: super::super::GROUP,
                id: stringify!($id),
            };

            #[allow(unused)]
            #[inline]
            pub fn patch(patcher: &mut Patcher, layout: &$crate::Layout, settings: &$crate::Settings) -> $crate::Result<()> {
                $(use $crate::patch::Patch;
                $($crate::patch!($variant $props).apply(
                    patcher,
                    layout
                        .get(&$crate::LocationInfo::new(AREA, $key))
                        .unwrap_or_else(|| unreachable!(stringify!($key))),
                    settings,
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
        Patch::Chest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
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
        Patch::BigChest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
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
        Patch::Event { course: Some(albw::course::Id::$course), name: stringify!($name), index: $index }
    };
    (Event[$($name:ident[$index:literal],)+]) => {
        Patch::Multi(vec![
            $(
                Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index },
            )+
        ])
    };
    (Heart($course:ident $scene:literal[$unq:literal])) => {
        Patch::Heart { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Heart($scene:literal[$unq:literal])) => {
        Patch::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Key($course:ident $scene:literal[$unq:literal])) => {
        Patch::Key { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Key($scene:literal[$unq:literal])) => {
        Patch::Key { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($course:ident $scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($scene:literal[$unq:literal])) => {
        Patch::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($scene:literal[$unq:literal])) => {
        Patch::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($scene:literal[$unq:literal])) => {
        Patch::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Shop($variant:ident$($args:tt)?)) => {
        Patch::Shop($crate::patch::Shop::$variant $($args)?)
    };
    (None()) => {
        Patch::None
    }
}
