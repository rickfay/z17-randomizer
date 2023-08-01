use std::{
    fmt::{self, Debug, Formatter},
    hash::{Hash, Hasher},
    ops::Deref,
};

#[derive(Debug, Default)]
pub struct NamedArea {
    pub name: Option<&'static str>,
    pub areas: &'static [NamedArea],
    locations: &'static [Location],
}

impl NamedArea {
    pub fn locations(&self) -> impl Iterator<Item = LocationNode> {
        self.locations.iter().map(LocationNode)
    }
}

pub struct AreaInfo {
    name: &'static str,
    group: Group,
    id: &'static str,
}

impl AreaInfo {
    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn group(&self) -> Group {
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

pub const NAMED_AREA: NamedArea = NamedArea {
    name: None,
    areas: &[
        hyrule::NAMED_AREA,
        lorule::NAMED_AREA,
        dungeons::graveyards::NAMED_AREA,
        dungeons::eastern::NAMED_AREA,
        dungeons::house::NAMED_AREA,
        dungeons::tower::NAMED_AREA,
        dungeons::hyrule::NAMED_AREA,
        dungeons::dark::NAMED_AREA,
        dungeons::swamp::NAMED_AREA,
        dungeons::skull::NAMED_AREA,
        dungeons::thieves::NAMED_AREA,
        dungeons::ice::NAMED_AREA,
        dungeons::desert::NAMED_AREA,
        dungeons::turtle::NAMED_AREA,
        dungeons::lorule::NAMED_AREA,
    ],
    locations: &[],
};

macro_rules! regions {
    ($($group:ident($variant:ident) {
        $($region:ident;)+
    })+) => {
        $(pub mod $group {
            use super::{LocationNode, NamedArea};

            $(pub mod $region;)+

            pub fn regions() -> impl Iterator<Item = Box<dyn Iterator<Item = LocationNode>>> {
                [
                    $(Box::new($region::locations()) as Box<_>,)+
                ].into_iter()
            }

            pub const NAMED_AREA: NamedArea = NamedArea {
                name: Some(stringify!($variant)),
                areas: &[
                    $($region::NAMED_AREA,)+
                ],
                locations: &[],
            };

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
        $start:ident $start_props:tt,
        $($id:ident $props:tt,)*
    ) => {
        #[inline]
        pub fn locations() -> impl Iterator<Item=$crate::world::LocationNode> {
            $start::locations()
                $(.chain($id::locations()))*
        }

        $crate::area!($start $start_props);
        $($crate::area!($id $props);)*

        #[allow(unused)]
        pub(crate) fn start() -> $crate::world::Area {
            $start::AREA
        }

        pub const NAMED_AREA: $crate::world::NamedArea = $crate::world::NamedArea {
            name: Some($name),
            areas: &[
                $start::NAMED_AREA,
                $($id::NAMED_AREA,)*
            ],
            locations: &[],
        };

        pub const NAME: &str = $name;
        #[allow(unused)]
        pub const COURSE: $crate::Course = $crate::Course::$course;
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
            use $crate::world::{Area, AreaInfo, Location, LocationData, LocationNode, NamedArea};

            pub use super::COURSE;

            pub const AREA: Area = &AreaInfo {
                name: super::NAME,
                group: super::super::GROUP,
                id: stringify!($id),
            };

            #[allow(unused)]
            #[inline]
            pub fn locations() -> impl Iterator<Item = LocationNode> {
                LOCATIONS.iter().map(LocationNode)
            }

            pub fn get(name: &'static str) -> Option<LocationNode> {
                locations().find(|location| {
                    location.name == name
                })
            }

            pub const NAMED_AREA: NamedArea = NamedArea {
                name: None,
                areas: &[],
                locations: LOCATIONS,
            };

            const LOCATIONS: &[Location] = &[
                $($(Location {
                        area: AREA,
                        name: $key,
                        data: $crate::location!($variant $props),
                },)*)?
            ];
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! location {
    (Chest($course:ident $stage:literal[$unq:literal])) => {
        LocationData::Chest { course: $crate::Course::$course, stage: $stage - 1, unq: $unq }
    };
    (Chest($stage:literal[$unq:literal])) => {
        LocationData::Chest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Chest[$($stage:literal[$unq:literal],)+]) => {
        LocationData::Multi(&[
            $(
                LocationData::Chest { course: COURSE, stage: $stage - 1, unq: $unq },
            )+
        ])
    };
    (BigChest($course:ident $stage:literal[$unq:literal])) => {
        LocationData::BigChest { course: $crate::Course::$course, stage: $stage - 1, unq: $unq }
    };
    (BigChest($stage:literal[$unq:literal])) => {
        LocationData::BigChest { course: COURSE, stage: $stage - 1, unq: $unq }
    };
    (Event($name:ident[$index:literal])) => {
        LocationData::Event { course: Some(COURSE), name: stringify!($name), index: $index }
    };
    (Event(Boot/$name:ident[$index:literal])) => {
        LocationData::Event { course: None, name: stringify!($name), index: $index }
    };
    (Event($course:ident/$name:ident[$index:literal])) => {
        LocationData::Event { course: Some($crate::Course::$course), name: stringify!($name), index: $index }
    };
    (Event[$($name:ident[$index:literal],)+]) => {
        LocationData::Multi(&[
            $(
                LocationData::Event { course: Some(COURSE), name: stringify!($name), index: $index },
            )+
        ])
    };
    (Heart($course:ident $scene:literal[$unq:literal])) => {
        LocationData::Heart { course: $crate::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Heart($scene:literal[$unq:literal])) => {
        LocationData::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Key($course:ident $scene:literal[$unq:literal])) => {
        LocationData::Key { course: $crate::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Key($scene:literal[$unq:literal])) => {
        LocationData::Key { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($course:ident $scene:literal[$unq:literal])) => {
        LocationData::Maiamai { course: $crate::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (Maiamai($scene:literal[$unq:literal])) => {
        LocationData::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
        LocationData::SilverRupee { course: $crate::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (SilverRupee($scene:literal[$unq:literal])) => {
        LocationData::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
        LocationData::GoldRupee { course: $crate::Course::$course, scene: $scene - 1, unq: $unq }
    };
    (GoldRupee($scene:literal[$unq:literal])) => {
        LocationData::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
    };
    (Shop($variant:ident$($args:tt)?)) => {
        LocationData::Shop($crate::world::Shop::$variant $($args)?)
    };
    (None()) => {
        LocationData::None
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
pub enum LocationData {
    Chest { course: crate::Course, stage: u16, unq: u16 },
    BigChest { course: crate::Course, stage: u16, unq: u16 },
    Event { course: Option<crate::Course>, name: &'static str, index: u16 },
    Heart { course: crate::Course, scene: u16, unq: u16 },
    Key { course: crate::Course, scene: u16, unq: u16 },
    Maiamai { course: crate::Course, scene: u16, unq: u16 },
    SilverRupee { course: crate::Course, scene: u16, unq: u16 },
    GoldRupee { course: crate::Course, scene: u16, unq: u16 },
    Shop(Shop),
    Multi(&'static [LocationData]),
    None, // Workaround until everything is shufflable
}

#[derive(Clone, Debug)]
pub enum Shop {
    Ravio(u8),
    Merchant(u8),
}

#[derive(Clone, Debug)]
pub struct Location {
    pub area: Area,
    pub name: &'static str,
    pub data: LocationData,
}

impl Location {
    pub fn group(&self) -> Group {
        self.area.group()
    }

    pub fn region(&self) -> &'static str {
        self.area.name()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct LocationNode(&'static Location);

impl Deref for LocationNode {
    type Target = Location;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}
