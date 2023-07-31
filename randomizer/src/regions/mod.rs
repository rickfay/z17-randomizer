use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
};

use rom::course::Id;

use crate::hints::hint_color::HintColor;

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

macro_rules! regions {
    ($($group:ident($variant:ident) {
        $($region:ident;)+
    })+) => {
        $(pub mod $group {
            use crate::LocationKey;
            use super::Location;

            $(pub mod $region;)+

            pub fn regions() -> impl Iterator<Item = Box<dyn Iterator<Item = (LocationKey, Location)>>> {
                [
                    $(Box::new($region::locations()) as Box<_>,)+
                ].into_iter()
            }

            pub const GROUP: super::Group = super::Group::$variant;
        })+
    };
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
        pub fn locations() -> impl Iterator<Item=($crate::LocationKey, $crate::regions::Location)> {
            $start::locations()
                $(.chain($id::locations()))*
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
        pub const COURSE: ::rom::course::Id = ::rom::course::Id::$course;
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
            use $crate::{regions::{Area, AreaInfo, Location}, LocationKey};

            pub use super::COURSE;

            pub const AREA: Area = &AreaInfo {
                name: super::NAME,
                color: super::COLOR,
                group: super::super::GROUP,
                id: stringify!($id),
            };

            #[allow(unused)]
            #[inline]
            pub fn locations() -> impl Iterator<Item = (LocationKey, Location)> {
                [
                    $($((
                        LocationKey {
                            area: AREA,
                            name: $key,
                        },
                        $crate::location!($variant $props),
                    ),)*)?
                ].into_iter()
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! location {
    (Chest($course:ident $stage:literal[$unq:literal])) => {
        Location::Chest { course: ::rom::course::Id::$course, stage: $stage - 1, unq: $unq }
    };
    (Chest($stage:literal[$unq:literal])) => {
        Location::Chest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Chest[$($stage:literal[$unq:literal],)+]) => {
        Location::Multi(vec![
            $(
                Location::Chest { course: COURSE, stage: $stage - 1, unq: $unq },
            )+
        ])
    };
    (BigChest($course:ident $stage:literal[$unq:literal])) => {
        Location::BigChest { course: ::rom::course::Id::$course, stage: $stage - 1, unq: $unq }
    };
    (BigChest($stage:literal[$unq:literal])) => {
        Location::BigChest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Event($name:ident[$index:literal])) => {
        Location::Event { course: Some(COURSE), name: stringify!($name), index: $index }
    };
    (Event(Boot/$name:ident[$index:literal])) => {
        Location::Event { course: None, name: stringify!($name), index: $index }
    };
    (Event($course:ident/$name:ident[$index:literal])) => {
        Location::Event { course: Some(::rom::course::Id::$course), name: stringify!($name), index: $index }
    };
    (Event[$($name:ident[$index:literal],)+]) => {
        Location::Multi(vec![
            $(
                Location::Event { course: Some(COURSE), name: stringify!($name), index: $index },
            )+
        ])
    };
    (Heart($course:ident $scene:literal[$unq:literal])) => {
        Location::Heart { course: ::rom::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Heart($scene:literal[$unq:literal])) => {
        Location::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Key($course:ident $scene:literal[$unq:literal])) => {
        Location::Key { course: ::rom::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Key($scene:literal[$unq:literal])) => {
        Location::Key { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($course:ident $scene:literal[$unq:literal])) => {
        Location::Maiamai { course: ::rom::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($scene:literal[$unq:literal])) => {
        Location::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
        Location::SilverRupee { course: ::rom::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($scene:literal[$unq:literal])) => {
        Location::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
        Location::GoldRupee { course: ::rom::course::Id::$course, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($scene:literal[$unq:literal])) => {
        Location::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Shop($variant:ident$($args:tt)?)) => {
        Location::Shop($crate::regions::Shop::$variant $($args)?)
    };
    (None()) => {
        Location::None
    }
}

regions! {
    dungeons(Dungeons) {
        dark;
        desert;
        eastern;
        graveyards;
        house;
        hyrule;
        ice;
        lorule;
        skull;
        swamp;
        thieves;
        tower;
        turtle;
    }
    hyrule(Hyrule) {
        death;
        desert;
        eastern;
        field;
        irene;
        kakariko;
        lake;
        lost;
        southern;
        zora;
    }
    lorule(Lorule) {
        chamber;
        dark;
        death;
        field;
        lake;
        misery;
        skull;
    }
}

#[derive(Clone, Debug)]
pub enum Location {
    Chest { course: Id, stage: u16, unq: u16 },
    BigChest { course: Id, stage: u16, unq: u16 },
    Event { course: Option<Id>, name: &'static str, index: u16 },
    Heart { course: Id, scene: u16, unq: u16 },
    Key { course: Id, scene: u16, unq: u16 },
    Maiamai { course: Id, scene: u16, unq: u16 },
    SilverRupee { course: Id, scene: u16, unq: u16 },
    GoldRupee { course: Id, scene: u16, unq: u16 },
    Shop(Shop),
    Multi(Vec<Location>),
    None, // Workaround until everything is shufflable
}

#[derive(Clone, Debug)]
pub enum Shop {
    Ravio(u8),
    Merchant(u8),
}
