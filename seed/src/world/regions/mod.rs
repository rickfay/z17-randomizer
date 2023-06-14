// use std::{fmt::Debug, hash::Hash};

// pub mod dungeons {
//     pub mod dark;
//     pub mod desert;
//     pub mod eastern;
//     pub mod graveyards;
//     pub mod house;
//     pub mod hyrule;
//     pub mod ice;
//     pub mod lorule;
//     pub mod skull;
//     pub mod swamp;
//     pub mod thieves;
//     pub mod tower;
//     pub mod turtle;
// }
// pub mod hyrule {
//     pub mod death;
//     pub mod desert;
//     pub mod eastern;
//     pub mod field;
//     pub mod irene;
//     pub mod kakariko;
//     pub mod lake;
//     pub mod lost;
//     pub mod southern;
//     pub mod zora;
// }
// pub mod lorule {
//     pub mod chamber;
//     pub mod dark;
//     pub mod death;
//     pub mod field;
//     pub mod lake;
//     pub mod misery;
//     pub mod skull;
// }

// /// Region ID
// #[derive(Clone, Debug, Eq, Hash, PartialEq)]
// pub enum RegionId {
//     CentralHyrule,
//     CentralLorule,
//     DarkPalace,
//     DarkRuins,
//     DeathMountain,
//     DeathMountainLorule,
//     DesertOfMystery,
//     DesertPalace,
//     EasternPalace,
//     EasternRuins,
//     Graveyards,
//     HouseOfGales,
//     IceRuins,
//     InsideHyruleCastle,
//     KakarikoVillage,
//     LakeHylia,
//     LoruleCastle,
//     LoruleLake,
//     LostWoods,
//     MiseryMire,
//     SkullWoods,
//     SkullWoodsOverworld,
//     SouthernRuins,
//     SwampPalace,
//     ThievesHideout,
//     TowerOfHera,
//     TurtleRock,
//     ZorasRiver,
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! region {
//     (
//         course:
//         $course:ident,name:
//         $name:literal,color:
//         $color:ident,locations:
//         [
//             $(
//                 $key:literal :
//                 $item:ident @
//                 $variant:ident
//                 $props:tt
//                 $(: - $condition:tt)?
//                 $(where $settings:ident : $where:expr)?,
//             )*
//         ]
//     ) => {
//         #[allow(unused)]
//         pub(crate) fn start() -> &'static crate::regions::Subregion {
//             $start::SUBREGION
//         }
//
//         pub const NAME: &str = $name;
//         pub const COLOR: crate::hints::hint_color::HintColor =
//             crate::hints::hint_color::HintColor::$color;
//         #[allow(unused)]
//         pub const COURSE: albw::course::Id = albw::course::Id::$course;
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! subregion {
//     () => {
//         pub mod $id {
//
//             pub use super::COURSE;
//             use crate::{patch::Patcher, regions::Subregion};
//
//             pub const SUBREGION: &Subregion = &Subregion {
//                 name: super::NAME,
//                 color: super::COLOR,
//                 world: super::super::WORLD,
//                 id: stringify!($id),
//             };
//         }
//     };
// }

// #[doc(hidden)]
// #[macro_export]
// macro_rules! patch {
//     (Chest($course:ident $stage:literal[$unq:literal])) => {
//         Patch::Chest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
//     };
//     (Chest($stage:literal[$unq:literal])) => {
//         Patch::Chest { course: COURSE, stage: $stage - 1, unq: $unq }
//     };
//     (Chest[$($stage:literal[$unq:literal],)+]) => {
//         Patch::Multi(vec![
//             $(
//                 Patch::Chest { course: COURSE, stage: $stage - 1, unq: $unq },
//             )+
//         ])
//     };
//     (BigChest($course:ident $stage:literal[$unq:literal])) => {
//         Patch::BigChest { course: albw::course::Id::$course, stage: $stage - 1, unq: $unq }
//     };
//     (BigChest($stage:literal[$unq:literal])) => {
//         Patch::BigChest { course: COURSE, stage: $stage - 1, unq: $unq }
//     };
//     (Event($name:ident[$index:literal])) => {
//         Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index }
//     };
//     (Event(Boot/$name:ident[$index:literal])) => {
//         Patch::Event { course: None, name: stringify!($name), index: $index }
//     };
//     (Event($course:ident/$name:ident[$index:literal])) => {
//         Patch::Event { course: Some(albw::course::Id::$course), name: stringify!($name), index: $index }
//     };
//     (Event[$($name:ident[$index:literal],)+]) => {
//         Patch::Multi(vec![
//             $(
//                 Patch::Event { course: Some(COURSE), name: stringify!($name), index: $index },
//             )+
//         ])
//     };
//     (Heart($course:ident $scene:literal[$unq:literal])) => {
//         Patch::Heart { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
//     };
//     (Heart($scene:literal[$unq:literal])) => {
//         Patch::Heart { course: COURSE, scene: $scene - 1, unq: $unq }
//     };
//     (Key($course:ident $scene:literal[$unq:literal])) => {
//         Patch::Key { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
//     };
//     (Key($scene:literal[$unq:literal])) => {
//         Patch::Key { course: COURSE, scene: $scene - 1, unq: $unq }
//     };
//     (Maiamai($course:ident $scene:literal[$unq:literal])) => {
//         Patch::Maiamai { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
//     };
//     (Maiamai($scene:literal[$unq:literal])) => {
//         Patch::Maiamai { course: COURSE, scene: $scene - 1, unq: $unq }
//     };
//     (SilverRupee($course:ident $scene:literal[$unq:literal])) => {
//         Patch::SilverRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
//     };
//     (SilverRupee($scene:literal[$unq:literal])) => {
//         Patch::SilverRupee { course: COURSE, scene: $scene - 1, unq: $unq }
//     };
//     (GoldRupee($course:ident $scene:literal[$unq:literal])) => {
//         Patch::GoldRupee { course: albw::course::Id::$course, scene: $scene - 1, unq: $unq }
//     };
//     (GoldRupee($scene:literal[$unq:literal])) => {
//         Patch::GoldRupee { course: COURSE, scene: $scene - 1, unq: $unq }
//     };
//     (Shop($variant:ident$($args:tt)?)) => {
//         Patch::Shop(crate::patch::Shop::$variant $($args)?)
//     };
//     (None()) => {
//         Patch::None
//     }
// }
