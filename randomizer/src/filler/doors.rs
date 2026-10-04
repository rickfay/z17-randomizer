use log::info;
use modinfo::Settings;
use modinfo::settings::{DoorShuffle, LogicMode, WeatherVanes};
use rand::Rng;
use rand::prelude::StdRng;
use serde::{Serialize, Serializer};
use std::cmp::Ordering;
use std::fmt::{Display, Formatter};

use crate::filler::filler_item::Randomizable;
use crate::filler::item_pools;
use crate::filler::location::Location;
use crate::{DashMap, DoorMap, filler};
use rom::scene::SpawnPoint;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Door {
    // Dungeons ------------------------------------------------------------------------------------
    EasternPalaceEntrance,
    EasternPalaceExit,
    HouseOfGalesEntrance,
    HouseOfGalesExit,
    TowerOfHeraEntrance,
    TowerOfHeraExit,

    InsideHyruleCastleEntrance,
    InsideHyruleCastleExit,

    DarkPalaceEntrance,
    DarkPalaceExit,
    SwampPalaceEntrance,
    SwampPalaceExit,
    SkullWoodsEntrance,
    SkullWoodsExit,
    ThievesHideoutEntrance,
    ThievesHideoutExit,
    TurtleRockEntrance,
    TurtleRockExit,
    DesertPalaceEntrance,
    DesertPalaceExit,
    IceRuinsEntrance,
    IceRuinsExit,

    LoruleCastleEntrance,
    LoruleCastleExit,

    // Back Doors
    CuccoHouseBackEntrance,
    CuccoHouseBackExit,
    WitchCaveBackEntrance,
    WitchCaveBackExit,
    VacantHouseBackEntrance,
    VacantHouseBackExit,
    IceCaveBackEntrance,
    IceCaveBackExit,

    // Hyrule --------------------------------------------------------------------------------------

    // Hyrule Castle Area
    YourHouseEntrance,
    YourHouseExit,
    BlacksmithEntrance,
    BlacksmithExit,
    BlacksmithCaveEntrance,
    BlacksmithCaveExit,
    CuccoMiniDungeonEntrance,
    CuccoMiniDungeonExit,
    HyruleCastleUpperLeftEntrance,
    HyruleCastleUpperLeftExit,
    HyruleCastleUpperRightEntrance,
    HyruleCastleUpperRightExit,
    HyruleCastleLowerLeftEntrance,
    HyruleCastleLowerLeftExit,
    HyruleCastleLowerRightEntrance,
    HyruleCastleLowerRightExit,
    HyruleCastleMainEntrance,
    HyruleCastleMainExit,

    // Kakariko Village
    MilkBarEntrance,
    MilkBarExit,
    CuccoHouseFrontEntrance,
    CuccoHouseFrontExit,
    StylishWomansHouseEntrance,
    StylishWomansHouseExit,
    BeeGuyHouseEntrance,
    BeeGuyHouseExit,
    HyruleFortunesChoiceEntrance,
    HyruleFortunesChoiceExit,
    WomanHouseEntrance,
    WomanHouseExit,
    KakarikoItemShopEntrance,
    KakarikoItemShopExit,
    JailEntrance,
    JailExit,
    SahasrahlaLeftEntrance,
    SahasrahlaLeftExit,
    SahasrahlaRightEntrance,
    SahasrahlaRightExit,
    KakarikoCaveEntrance,
    KakarikoCaveExit,

    // Southern Ruins
    SouthernRuinsFairyCaveEntrance,
    SouthernRuinsFairyCaveExit,
    RunawayItemSellerCaveEntrance,
    RunawayItemSellerCaveExit,
    SouthernRuinsMiniDungeonEntrance,
    SouthernRuinsMiniDungeonExit,
    SouthernRuinsBombCaveEntrance,
    SouthernRuinsBombCaveExit,
    SouthernRuinsPillarCaveEntrance,
    SouthernRuinsPillarCaveExit,

    // Desert
    DesertBigFairyCaveEntrance,
    DesertBigFairyCaveExit,
    DesertFairyCaveEntrance,
    DesertFairyCaveExit,

    // River
    HyruleSewersEntrance,
    HyruleSewersExit,
    GraveyardLedgeCaveEntrance,
    GraveyardLedgeCaveExit,
    ZorasDomainEntrance,
    ZorasDomainExit,
    WaterfallCaveEntrance,
    WaterfallCaveExit,
    WitchHouseEntrance,
    WitchHouseExit,
    RiverMiniDungeonEntrance,
    RiverMiniDungeonExit,

    // Eastern Ruins
    MergeMiniDungeonEntrance,
    MergeMiniDungeonExit,
    EasternRuinsFairyCaveEntrance,
    EasternRuinsFairyCaveExit,
    EasternRuinsBigFairyCaveEntrance,
    EasternRuinsBigFairyCaveExit,
    EasternRuinsCaveTopEntrance,
    EasternRuinsCaveTopExit,
    EasternRuinsCaveBottomEntrance,
    EasternRuinsCaveBottomExit,
    WitchCaveFrontEntrance,
    WitchCaveFrontExit,

    // Lake Hylia
    LakeHyliaDarkCaveEntrance,
    LakeHyliaDarkCaveExit,
    LakesideItemShopEntrance,
    LakesideItemShopExit,
    MotherMaiamaiCaveEntrance,
    MotherMaiamaiCaveExit,
    IceRodCaveLeftEntrance,
    IceRodCaveLeftExit,
    IceRodCaveRightEntrance,
    IceRodCaveRightExit,

    // Lost Woods Area
    FortuneTellerTentEntrance,
    FortuneTellerTentExit,
    FortuneTellerCaveEntrance,
    FortuneTellerCaveExit,
    RumorGuyCaveEntrance,
    RumorGuyCaveExit,
    RossoHouseEntrance,
    RossoHouseExit,
    RossoCaveEntrance,
    RossoCaveExit,
    MoldormCaveLowerEntrance,
    MoldormCaveLowerExit,
    MoldormCaveLedgeEntrance,
    MoldormCaveLedgeExit,

    // Death Mountain
    MoldormCaveTopEntrance,
    MoldormCaveTopExit,
    DeathMountainBombCaveEntrance,
    DeathMountainBombCaveExit,
    DeathMountainWeatherVaneLeftCaveEntrance,
    DeathMountainWeatherVaneLeftCaveExit,
    DeathMountainWestFairyCaveEntrance,
    DeathMountainWestFairyCaveExit,
    DonkeyCaveLowerEntrance,
    DonkeyCaveLowerExit,
    DonkeyCaveMiddleEntrance,
    DonkeyCaveMiddleExit,
    DonkeyCaveUpperEntrance,
    DonkeyCaveUpperExit,
    BigRollingRocksCaveLowerEntrance,
    BigRollingRocksCaveLowerExit,
    BigRollingRocksCaveMiddleRightEntrance,
    BigRollingRocksCaveMiddleRightExit,
    BigRollingRocksCaveMiddleLeftEntrance,
    BigRollingRocksCaveMiddleLeftExit,
    BigRollingRocksCaveUpperEntrance,
    BigRollingRocksCaveUpperExit,
    SpectacleRockCaveLeftEntrance,
    SpectacleRockCaveLeftExit,
    SpectacleRockCaveRightEntrance,
    SpectacleRockCaveRightExit,
    HookshotMiniDungeonEntrance,
    HookshotMiniDungeonExit,
    FireCaveTopEntrance,
    FireCaveTopExit,
    FireCaveMiddleLeftEntrance,
    FireCaveMiddleLeftExit,
    FireCaveMiddleRightEntrance,
    FireCaveMiddleRightExit,
    FireCaveBottomEntrance,
    FireCaveBottomExit,
    DeathMountainBigFairyCaveEntrance,
    DeathMountainBigFairyCaveExit,

    // Lorule --------------------------------------------------------------------------------------

    // Skull Woods Area
    MysteriousManCaveEntrance,
    MysteriousManCaveExit,

    // Lorule Death Mountain
    LoruleDeathMountainBigFairyCaveEntrance,
    LoruleDeathMountainBigFairyCaveExit,
    IceCaveLowerEntrance,
    IceCaveLowerExit,
    IceCaveMiddleLeftEntrance,
    IceCaveMiddleLeftExit,
    IceCaveMiddleRightEntrance,
    IceCaveMiddleRightExit,
    IceCaveUpperEntrance,
    IceCaveUpperExit,

    // Graveyard
    PhilosophersCaveEntrance,
    PhilosophersCaveExit,
    LoruleSewersEntrance,
    LoruleSewersExit,

    // Dark Ruins
    HinoxCaveEntrance,
    HinoxCaveExit,
    DarkRuinsFairyCaveEntrance,
    DarkRuinsFairyCaveExit,
    DarkRuinsBigFairyCaveEntrance,
    DarkRuinsBigFairyCaveExit,

    // Turtle Rock Area
    TurtleRockFairyCaveEntrance,
    TurtleRockFairyCaveExit,
    LoruleLakeItemShopEntrance,
    LoruleLakeItemShopExit,

    // Lorule Castle Area
    VacantHouseFrontEntrance,
    VacantHouseFrontExit,
    ThiefGirlCaveEntrance,
    ThiefGirlCaveExit,
    SwampCaveEntrance,
    SwampCaveExit,
    BombFlowerCaveEntrance,
    BombFlowerCaveExit,
    GreatRupeeFairyCaveEntrance,
    GreatRupeeFairyCaveExit,
    BombFlowerShopEntrance,
    BombFlowerShopExit,
    LoruleBlacksmithEntrance,
    LoruleBlacksmithExit,
    LoruleFortuneTellerEntrance,
    LoruleFortuneTellerExit,
    LoruleMilkBarEntrance,
    LoruleMilkBarExit,
    VeteransHouseEntrance,
    VeteransHouseExit,
    LoruleFortunesChoiceEntrance,
    LoruleFortunesChoiceExit,
    ThievesTownItemShopEntrance,
    ThievesTownItemShopExit,

    // Misery Mire
    SandRodMiniDungeonEntrance,
    SandRodMiniDungeonExit,
}

impl Door {
    ///
    pub fn get_vanilla_destination(&self) -> Self {
        use self::Door::*;
        match self {
            // Dungeons ----------------------------------------------------------------------------
            EasternPalaceEntrance => EasternPalaceExit,
            EasternPalaceExit => EasternPalaceEntrance,
            HouseOfGalesEntrance => HouseOfGalesExit,
            HouseOfGalesExit => HouseOfGalesEntrance,
            TowerOfHeraEntrance => TowerOfHeraExit,
            TowerOfHeraExit => TowerOfHeraEntrance,
            InsideHyruleCastleEntrance => InsideHyruleCastleExit,
            InsideHyruleCastleExit => InsideHyruleCastleEntrance,
            DarkPalaceEntrance => DarkPalaceExit,
            DarkPalaceExit => DarkPalaceEntrance,
            SwampPalaceEntrance => SwampPalaceExit,
            SwampPalaceExit => SwampPalaceEntrance,
            SkullWoodsEntrance => SkullWoodsExit,
            SkullWoodsExit => SkullWoodsEntrance,
            ThievesHideoutEntrance => ThievesHideoutExit,
            ThievesHideoutExit => ThievesHideoutEntrance,
            TurtleRockEntrance => TurtleRockExit,
            TurtleRockExit => TurtleRockEntrance,
            DesertPalaceEntrance => DesertPalaceExit,
            DesertPalaceExit => DesertPalaceEntrance,
            IceRuinsEntrance => IceRuinsExit,
            IceRuinsExit => IceRuinsEntrance,
            LoruleCastleEntrance => LoruleCastleExit,
            LoruleCastleExit => LoruleCastleEntrance,

            // Back Entrances
            CuccoHouseBackEntrance => CuccoHouseBackExit,
            CuccoHouseBackExit => CuccoHouseBackEntrance,
            VacantHouseBackEntrance => VacantHouseBackExit,
            VacantHouseBackExit => VacantHouseBackEntrance,
            WitchCaveBackEntrance => WitchCaveBackExit,
            WitchCaveBackExit => WitchCaveBackEntrance,
            IceCaveBackEntrance => IceCaveBackExit,
            IceCaveBackExit => IceCaveBackEntrance,

            // Hyrule ------------------------------------------------------------------------------

            // Hyrule Castle Area
            YourHouseEntrance => YourHouseExit,
            YourHouseExit => YourHouseEntrance,
            BlacksmithEntrance => BlacksmithExit,
            BlacksmithExit => BlacksmithEntrance,
            BlacksmithCaveEntrance => BlacksmithCaveExit,
            BlacksmithCaveExit => BlacksmithCaveEntrance,
            CuccoMiniDungeonEntrance => CuccoMiniDungeonExit,
            CuccoMiniDungeonExit => CuccoMiniDungeonEntrance,
            HyruleCastleUpperLeftEntrance => HyruleCastleUpperLeftExit,
            HyruleCastleUpperLeftExit => HyruleCastleUpperLeftEntrance,
            HyruleCastleUpperRightEntrance => HyruleCastleUpperRightExit,
            HyruleCastleUpperRightExit => HyruleCastleUpperRightEntrance,
            HyruleCastleLowerLeftEntrance => HyruleCastleLowerLeftExit,
            HyruleCastleLowerLeftExit => HyruleCastleLowerLeftEntrance,
            HyruleCastleLowerRightEntrance => HyruleCastleLowerRightExit,
            HyruleCastleLowerRightExit => HyruleCastleLowerRightEntrance,
            HyruleCastleMainEntrance => HyruleCastleMainExit,
            HyruleCastleMainExit => HyruleCastleMainEntrance,

            // Kakariko Village
            MilkBarEntrance => MilkBarExit,
            MilkBarExit => MilkBarEntrance,
            CuccoHouseFrontEntrance => CuccoHouseFrontExit,
            CuccoHouseFrontExit => CuccoHouseFrontEntrance,
            StylishWomansHouseEntrance => StylishWomansHouseExit,
            StylishWomansHouseExit => StylishWomansHouseEntrance,
            BeeGuyHouseEntrance => BeeGuyHouseExit,
            BeeGuyHouseExit => BeeGuyHouseEntrance,
            HyruleFortunesChoiceEntrance => HyruleFortunesChoiceExit,
            HyruleFortunesChoiceExit => HyruleFortunesChoiceEntrance,
            WomanHouseEntrance => WomanHouseExit,
            WomanHouseExit => WomanHouseEntrance,
            KakarikoItemShopEntrance => KakarikoItemShopExit,
            KakarikoItemShopExit => KakarikoItemShopEntrance,
            JailEntrance => JailExit,
            JailExit => JailEntrance,
            SahasrahlaLeftEntrance => SahasrahlaLeftExit,
            SahasrahlaLeftExit => SahasrahlaLeftEntrance,
            SahasrahlaRightEntrance => SahasrahlaRightExit,
            SahasrahlaRightExit => SahasrahlaRightEntrance,
            KakarikoCaveEntrance => KakarikoCaveExit,
            KakarikoCaveExit => KakarikoCaveEntrance,

            // Southern Ruins
            SouthernRuinsFairyCaveEntrance => SouthernRuinsFairyCaveExit,
            SouthernRuinsFairyCaveExit => SouthernRuinsFairyCaveEntrance,
            RunawayItemSellerCaveEntrance => RunawayItemSellerCaveExit,
            RunawayItemSellerCaveExit => RunawayItemSellerCaveEntrance,
            SouthernRuinsMiniDungeonEntrance => SouthernRuinsMiniDungeonExit,
            SouthernRuinsMiniDungeonExit => SouthernRuinsMiniDungeonEntrance,
            SouthernRuinsBombCaveEntrance => SouthernRuinsBombCaveExit,
            SouthernRuinsBombCaveExit => SouthernRuinsBombCaveEntrance,
            SouthernRuinsPillarCaveEntrance => SouthernRuinsPillarCaveExit,
            SouthernRuinsPillarCaveExit => SouthernRuinsPillarCaveEntrance,

            // Desert
            DesertBigFairyCaveEntrance => DesertBigFairyCaveExit,
            DesertBigFairyCaveExit => DesertBigFairyCaveEntrance,
            DesertFairyCaveEntrance => DesertFairyCaveExit,
            DesertFairyCaveExit => DesertFairyCaveEntrance,

            // River Area
            HyruleSewersEntrance => HyruleSewersExit,
            HyruleSewersExit => HyruleSewersEntrance,
            GraveyardLedgeCaveEntrance => GraveyardLedgeCaveExit,
            GraveyardLedgeCaveExit => GraveyardLedgeCaveEntrance,
            ZorasDomainEntrance => ZorasDomainExit,
            ZorasDomainExit => ZorasDomainEntrance,
            WaterfallCaveEntrance => WaterfallCaveExit,
            WaterfallCaveExit => WaterfallCaveEntrance,
            WitchHouseEntrance => WitchHouseExit,
            WitchHouseExit => WitchHouseEntrance,
            RiverMiniDungeonEntrance => RiverMiniDungeonExit,
            RiverMiniDungeonExit => RiverMiniDungeonEntrance,

            // Eastern Ruins
            MergeMiniDungeonEntrance => MergeMiniDungeonExit,
            MergeMiniDungeonExit => MergeMiniDungeonEntrance,
            EasternRuinsFairyCaveEntrance => EasternRuinsFairyCaveExit,
            EasternRuinsFairyCaveExit => EasternRuinsFairyCaveEntrance,
            EasternRuinsBigFairyCaveEntrance => EasternRuinsBigFairyCaveExit,
            EasternRuinsBigFairyCaveExit => EasternRuinsBigFairyCaveEntrance,
            EasternRuinsCaveTopEntrance => EasternRuinsCaveTopExit,
            EasternRuinsCaveTopExit => EasternRuinsCaveTopEntrance,
            EasternRuinsCaveBottomEntrance => EasternRuinsCaveBottomExit,
            EasternRuinsCaveBottomExit => EasternRuinsCaveBottomEntrance,
            WitchCaveFrontEntrance => WitchCaveFrontExit,
            WitchCaveFrontExit => WitchCaveFrontEntrance,

            // Lake Hylia
            LakeHyliaDarkCaveEntrance => LakeHyliaDarkCaveExit,
            LakeHyliaDarkCaveExit => LakeHyliaDarkCaveEntrance,
            LakesideItemShopEntrance => LakesideItemShopExit,
            LakesideItemShopExit => LakesideItemShopEntrance,
            MotherMaiamaiCaveEntrance => MotherMaiamaiCaveExit,
            MotherMaiamaiCaveExit => MotherMaiamaiCaveEntrance,
            IceRodCaveLeftEntrance => IceRodCaveLeftExit,
            IceRodCaveLeftExit => IceRodCaveLeftEntrance,
            IceRodCaveRightEntrance => IceRodCaveRightExit,
            IceRodCaveRightExit => IceRodCaveRightEntrance,

            // Lost Woods Area
            FortuneTellerTentEntrance => FortuneTellerTentExit,
            FortuneTellerTentExit => FortuneTellerTentEntrance,
            FortuneTellerCaveEntrance => FortuneTellerCaveExit,
            FortuneTellerCaveExit => FortuneTellerCaveEntrance,
            RumorGuyCaveEntrance => RumorGuyCaveExit,
            RumorGuyCaveExit => RumorGuyCaveEntrance,
            RossoHouseEntrance => RossoHouseExit,
            RossoHouseExit => RossoHouseEntrance,
            RossoCaveEntrance => RossoCaveExit,
            RossoCaveExit => RossoCaveEntrance,
            MoldormCaveLowerEntrance => MoldormCaveLowerExit,
            MoldormCaveLowerExit => MoldormCaveLowerEntrance,
            MoldormCaveLedgeEntrance => MoldormCaveLedgeExit,
            MoldormCaveLedgeExit => MoldormCaveLedgeEntrance,

            // Death Mountain
            MoldormCaveTopEntrance => MoldormCaveTopExit,
            MoldormCaveTopExit => MoldormCaveTopEntrance,
            DeathMountainBombCaveEntrance => DeathMountainBombCaveExit,
            DeathMountainBombCaveExit => DeathMountainBombCaveEntrance,
            DeathMountainWeatherVaneLeftCaveEntrance => DeathMountainWeatherVaneLeftCaveExit,
            DeathMountainWeatherVaneLeftCaveExit => DeathMountainWeatherVaneLeftCaveEntrance,
            DeathMountainWestFairyCaveEntrance => DeathMountainWestFairyCaveExit,
            DeathMountainWestFairyCaveExit => DeathMountainWestFairyCaveEntrance,
            DonkeyCaveLowerEntrance => DonkeyCaveLowerExit,
            DonkeyCaveLowerExit => DonkeyCaveLowerEntrance,
            DonkeyCaveMiddleEntrance => DonkeyCaveMiddleExit,
            DonkeyCaveMiddleExit => DonkeyCaveMiddleEntrance,
            DonkeyCaveUpperEntrance => DonkeyCaveUpperExit,
            DonkeyCaveUpperExit => DonkeyCaveUpperEntrance,
            BigRollingRocksCaveLowerEntrance => BigRollingRocksCaveLowerExit,
            BigRollingRocksCaveLowerExit => BigRollingRocksCaveLowerEntrance,
            BigRollingRocksCaveMiddleRightEntrance => BigRollingRocksCaveMiddleRightExit,
            BigRollingRocksCaveMiddleRightExit => BigRollingRocksCaveMiddleRightEntrance,
            BigRollingRocksCaveMiddleLeftEntrance => BigRollingRocksCaveMiddleLeftExit,
            BigRollingRocksCaveMiddleLeftExit => BigRollingRocksCaveMiddleLeftEntrance,
            BigRollingRocksCaveUpperEntrance => BigRollingRocksCaveUpperExit,
            BigRollingRocksCaveUpperExit => BigRollingRocksCaveUpperEntrance,
            SpectacleRockCaveLeftEntrance => SpectacleRockCaveLeftExit,
            SpectacleRockCaveLeftExit => SpectacleRockCaveLeftEntrance,
            SpectacleRockCaveRightEntrance => SpectacleRockCaveRightExit,
            SpectacleRockCaveRightExit => SpectacleRockCaveRightEntrance,
            HookshotMiniDungeonEntrance => HookshotMiniDungeonExit,
            HookshotMiniDungeonExit => HookshotMiniDungeonEntrance,
            FireCaveTopEntrance => FireCaveTopExit,
            FireCaveTopExit => FireCaveTopEntrance,
            FireCaveMiddleLeftEntrance => FireCaveMiddleLeftExit,
            FireCaveMiddleLeftExit => FireCaveMiddleLeftEntrance,
            FireCaveMiddleRightEntrance => FireCaveMiddleRightExit,
            FireCaveMiddleRightExit => FireCaveMiddleRightEntrance,
            FireCaveBottomEntrance => FireCaveBottomExit,
            FireCaveBottomExit => FireCaveBottomEntrance,
            DeathMountainBigFairyCaveEntrance => DeathMountainBigFairyCaveExit,
            DeathMountainBigFairyCaveExit => DeathMountainBigFairyCaveEntrance,

            // Lorule ------------------------------------------------------------------------------

            // Skull Woods
            MysteriousManCaveEntrance => MysteriousManCaveExit,
            MysteriousManCaveExit => MysteriousManCaveEntrance,

            // Lorule Death Mountain
            LoruleDeathMountainBigFairyCaveEntrance => LoruleDeathMountainBigFairyCaveExit,
            LoruleDeathMountainBigFairyCaveExit => LoruleDeathMountainBigFairyCaveEntrance,
            IceCaveLowerEntrance => IceCaveLowerExit,
            IceCaveLowerExit => IceCaveLowerEntrance,
            IceCaveMiddleLeftEntrance => IceCaveMiddleLeftExit,
            IceCaveMiddleLeftExit => IceCaveMiddleLeftEntrance,
            IceCaveMiddleRightEntrance => IceCaveMiddleRightExit,
            IceCaveMiddleRightExit => IceCaveMiddleRightEntrance,
            IceCaveUpperEntrance => IceCaveUpperExit,
            IceCaveUpperExit => IceCaveUpperEntrance,

            // Graveyard
            PhilosophersCaveEntrance => PhilosophersCaveExit,
            PhilosophersCaveExit => PhilosophersCaveEntrance,
            LoruleSewersEntrance => LoruleSewersExit,
            LoruleSewersExit => LoruleSewersEntrance,

            // Dark Ruins
            HinoxCaveEntrance => HinoxCaveExit,
            HinoxCaveExit => HinoxCaveEntrance,
            DarkRuinsFairyCaveEntrance => DarkRuinsFairyCaveExit,
            DarkRuinsFairyCaveExit => DarkRuinsFairyCaveEntrance,
            DarkRuinsBigFairyCaveEntrance => DarkRuinsBigFairyCaveExit,
            DarkRuinsBigFairyCaveExit => DarkRuinsBigFairyCaveEntrance,

            // Turtle Rock Area
            TurtleRockFairyCaveEntrance => TurtleRockFairyCaveExit,
            TurtleRockFairyCaveExit => TurtleRockFairyCaveEntrance,
            LoruleLakeItemShopEntrance => LoruleLakeItemShopExit,
            LoruleLakeItemShopExit => LoruleLakeItemShopEntrance,

            // Lorule Castle Area
            VacantHouseFrontEntrance => VacantHouseFrontExit,
            VacantHouseFrontExit => VacantHouseFrontEntrance,
            ThiefGirlCaveEntrance => ThiefGirlCaveExit,
            ThiefGirlCaveExit => ThiefGirlCaveEntrance,
            SwampCaveEntrance => SwampCaveExit,
            SwampCaveExit => SwampCaveEntrance,
            BombFlowerCaveEntrance => BombFlowerCaveExit,
            BombFlowerCaveExit => BombFlowerCaveEntrance,
            GreatRupeeFairyCaveEntrance => GreatRupeeFairyCaveExit,
            GreatRupeeFairyCaveExit => GreatRupeeFairyCaveEntrance,
            BombFlowerShopEntrance => BombFlowerShopExit,
            BombFlowerShopExit => BombFlowerShopEntrance,
            LoruleBlacksmithEntrance => LoruleBlacksmithExit,
            LoruleBlacksmithExit => LoruleBlacksmithEntrance,
            LoruleFortuneTellerEntrance => LoruleFortuneTellerExit,
            LoruleFortuneTellerExit => LoruleFortuneTellerEntrance,
            LoruleMilkBarEntrance => LoruleMilkBarExit,
            LoruleMilkBarExit => LoruleMilkBarEntrance,
            VeteransHouseEntrance => VeteransHouseExit,
            VeteransHouseExit => VeteransHouseEntrance,
            LoruleFortunesChoiceEntrance => LoruleFortunesChoiceExit,
            LoruleFortunesChoiceExit => LoruleFortunesChoiceEntrance,
            ThievesTownItemShopEntrance => ThievesTownItemShopExit,
            ThievesTownItemShopExit => ThievesTownItemShopEntrance,

            // Misery Mire
            SandRodMiniDungeonEntrance => SandRodMiniDungeonExit,
            SandRodMiniDungeonExit => SandRodMiniDungeonEntrance,
        }
    }

    ///
    pub fn get_location(&self) -> Location {
        use crate::filler::location::Location::*;
        match self {
            // Dungeons ----------------------------------------------------------------------------
            Door::EasternPalaceEntrance => EasternRuinsUpper,
            Door::EasternPalaceExit => EasternPalaceFoyer,
            Door::HouseOfGalesEntrance => HouseOfGalesIsland,
            Door::HouseOfGalesExit => HouseOfGalesFoyer,
            Door::TowerOfHeraEntrance => TowerOfHeraEntrancePegs,
            Door::TowerOfHeraExit => TowerOfHeraFoyer,
            Door::InsideHyruleCastleEntrance => HyruleCastleRoof,
            Door::InsideHyruleCastleExit => HyruleCastleDungeonFoyer,
            Door::DarkPalaceEntrance => DarkPalaceWeatherVane,
            Door::DarkPalaceExit => DarkPalaceFoyer,
            Door::SwampPalaceEntrance => SwampPalaceOutside,
            Door::SwampPalaceExit => SwampPalaceAntechamber,
            Door::SkullWoodsEntrance => SkullWoodsOverworld,
            Door::SkullWoodsExit => SkullWoodsFoyer,
            Door::ThievesHideoutEntrance => LoruleCastleArea,
            Door::ThievesHideoutExit => ThievesHideoutFoyer,
            Door::TurtleRockEntrance => TurtleRockFrontDoor,
            Door::TurtleRockExit => TurtleRockFoyer,
            Door::DesertPalaceEntrance => DesertPalaceWeatherVane,
            Door::DesertPalaceExit => DesertPalaceFoyer,
            Door::IceRuinsEntrance => LoruleDeathEastTop,
            Door::IceRuinsExit => IceRuinsFoyer,
            Door::LoruleCastleEntrance => LoruleCastleArea,
            Door::LoruleCastleExit => LoruleCastleFoyer,

            // Back Entrances
            Door::CuccoHouseBackEntrance => CuccoHouseRear,
            Door::CuccoHouseBackExit => CuccoHouse,
            Door::VacantHouseBackEntrance => LoruleCastleArea,
            Door::VacantHouseBackExit => VacantHouseTop,
            Door::WitchCaveBackEntrance => HyruleField,
            Door::WitchCaveBackExit => WitchCave,
            Door::IceCaveBackEntrance => FloatingIslandLorule,
            Door::IceCaveBackExit => IceCaveNorthWest,

            // Hyrule ------------------------------------------------------------------------------

            // Hyrule Castle Area
            Door::YourHouseEntrance => HyruleField,
            Door::YourHouseExit => RavioShop,
            Door::BlacksmithEntrance => HyruleField,
            Door::BlacksmithExit => BlacksmithHouse,
            Door::BlacksmithCaveEntrance => HyruleField,
            Door::BlacksmithCaveExit => BlacksmithCave,
            Door::CuccoMiniDungeonEntrance => CuccoDungeonLedge,
            Door::CuccoMiniDungeonExit => CuccoDungeon,
            Door::HyruleCastleUpperLeftEntrance => HyruleCastleRoof,
            Door::HyruleCastleUpperLeftExit => HyruleCastleInterior,
            Door::HyruleCastleUpperRightEntrance => HyruleCastleRoof,
            Door::HyruleCastleUpperRightExit => HyruleCastleInterior,
            Door::HyruleCastleLowerLeftEntrance => HyruleCastleCourtyard,
            Door::HyruleCastleLowerLeftExit => HyruleCastleLeftRoom,
            Door::HyruleCastleLowerRightEntrance => HyruleCastleCourtyard,
            Door::HyruleCastleLowerRightExit => HyruleCastleRightRoom,
            Door::HyruleCastleMainEntrance => HyruleCastleCourtyard,
            Door::HyruleCastleMainExit => HyruleCastleInterior,

            // Kakariko Village
            Door::MilkBarEntrance => HyruleField,
            Door::MilkBarExit => MilkBar,
            Door::CuccoHouseFrontEntrance => HyruleField,
            Door::CuccoHouseFrontExit => CuccoHouse,
            Door::StylishWomansHouseEntrance => HyruleField,
            Door::StylishWomansHouseExit => StylishWomanHouse,
            Door::BeeGuyHouseEntrance => HyruleField,
            Door::BeeGuyHouseExit => BeeGuyHouse,
            Door::HyruleFortunesChoiceEntrance => HyruleField,
            Door::HyruleFortunesChoiceExit => FortunesChoiceHyrule,
            Door::WomanHouseEntrance => HyruleField,
            Door::WomanHouseExit => WomanHouse,
            Door::KakarikoItemShopEntrance => HyruleField,
            Door::KakarikoItemShopExit => KakarikoItemShop,
            Door::JailEntrance => HyruleField,
            Door::JailExit => KakarikoJailCell,
            Door::SahasrahlaLeftEntrance => HyruleField,
            Door::SahasrahlaLeftExit => SahasrahlasHouse,
            Door::SahasrahlaRightEntrance => HyruleField,
            Door::SahasrahlaRightExit => SahasrahlasHouse,
            Door::KakarikoCaveEntrance => HyruleField,
            Door::KakarikoCaveExit => WellLower,

            // Southern Ruins
            Door::SouthernRuinsFairyCaveEntrance => HyruleField,
            Door::SouthernRuinsFairyCaveExit => SouthernRuinsFairyCave,
            Door::RunawayItemSellerCaveEntrance => HyruleField,
            Door::RunawayItemSellerCaveExit => ItemSellerCave,
            Door::SouthernRuinsMiniDungeonEntrance => OutsideFlippersDungeon,
            Door::SouthernRuinsMiniDungeonExit => FlippersDungeon,
            Door::SouthernRuinsBombCaveEntrance => HyruleField,
            Door::SouthernRuinsBombCaveExit => SouthernRuinsBombCave,
            Door::SouthernRuinsPillarCaveEntrance => SouthernRuinsPillars,
            Door::SouthernRuinsPillarCaveExit => SouthernRuinsBombCave,

            // Desert
            Door::DesertBigFairyCaveEntrance => HyruleField,
            Door::DesertBigFairyCaveExit => SouthernRuinsBigFairyCave,
            Door::DesertFairyCaveEntrance => DesertFairyLedge,
            Door::DesertFairyCaveExit => DesertFairyCave,

            // River Area
            Door::HyruleSewersEntrance => SewersEntrance,
            Door::HyruleSewersExit => Sewers,
            Door::GraveyardLedgeCaveEntrance => GraveyardLedgeHyrule,
            Door::GraveyardLedgeCaveExit => GraveyardLedgeCave,
            Door::ZorasDomainEntrance => ZoraDomainArea,
            Door::ZorasDomainExit => ZoraDomain,
            Door::WaterfallCaveEntrance => WaterfallCaveShallowWater,
            Door::WaterfallCaveExit => WaterfallCave,
            Door::WitchHouseEntrance => HyruleField,
            Door::WitchHouseExit => WitchHouse,
            Door::RiverMiniDungeonEntrance => HyruleField,
            Door::RiverMiniDungeonExit => TornadoRodDungeon,

            // Eastern Ruins
            Door::MergeMiniDungeonEntrance => EasternRuinsUpper,
            Door::MergeMiniDungeonExit => MergeDungeon,
            Door::EasternRuinsFairyCaveEntrance => HyruleField,
            Door::EasternRuinsFairyCaveExit => EasternFairyCave,
            Door::EasternRuinsBigFairyCaveEntrance => HyruleField,
            Door::EasternRuinsBigFairyCaveExit => EasternBigFairyCave,
            Door::EasternRuinsCaveTopEntrance => EasternRuinsEastLedge,
            Door::EasternRuinsCaveTopExit => EastRuinsBombCaveUpper,
            Door::EasternRuinsCaveBottomEntrance => HyruleField,
            Door::EasternRuinsCaveBottomExit => EastRuinsBombCaveLower,
            Door::WitchCaveFrontEntrance => EasternRuinsUpper,
            Door::WitchCaveFrontExit => WitchCave,

            // Lake Hylia
            Door::LakeHyliaDarkCaveEntrance => HyruleField,
            Door::LakeHyliaDarkCaveExit => LakeDarkCave,
            Door::LakesideItemShopEntrance => HyruleField,
            Door::LakesideItemShopExit => LakesideItemShop,
            Door::MotherMaiamaiCaveEntrance => HyruleField,
            Door::MotherMaiamaiCaveExit => MaiamaiCave,
            Door::IceRodCaveLeftEntrance => HyruleField,
            Door::IceRodCaveLeftExit => IceRodCaveLeft,
            Door::IceRodCaveRightEntrance => HyruleField,
            Door::IceRodCaveRightExit => IceRodCaveRight,

            // Lost Woods Area
            Door::FortuneTellerTentEntrance => HyruleField,
            Door::FortuneTellerTentExit => FortuneTeller,
            Door::FortuneTellerCaveEntrance => HyruleField,
            Door::FortuneTellerCaveExit => FortuneTellerCave,
            Door::RumorGuyCaveEntrance => RumorGuyCaveEntrance,
            Door::RumorGuyCaveExit => RumorGuyCave,
            Door::RossoHouseEntrance => HyruleField,
            Door::RossoHouseExit => RossosHouse,
            Door::RossoCaveEntrance => HyruleField,
            Door::RossoCaveExit => RossoCave,
            Door::MoldormCaveLowerEntrance => HyruleField,
            Door::MoldormCaveLowerExit => MoldormCave,
            Door::MoldormCaveLedgeEntrance => MoldormLedge,
            Door::MoldormCaveLedgeExit => MoldormCaveTop,

            // Death Mountain
            Door::MoldormCaveTopEntrance => DeathMountainBase,
            Door::MoldormCaveTopExit => MoldormCave,
            Door::DeathMountainBombCaveEntrance => DeathBombCaveLedge,
            Door::DeathMountainBombCaveExit => DeathBombCave,
            Door::DeathMountainWeatherVaneLeftCaveEntrance => DeathMountainBase,
            Door::DeathMountainWeatherVaneLeftCaveExit => DeathWeatherVaneCaveLeft,
            Door::DeathMountainWestFairyCaveEntrance => DeathFairyCaveLedge,
            Door::DeathMountainWestFairyCaveExit => DeathFairyCave,
            Door::DonkeyCaveLowerEntrance => DeathMountainBase,
            Door::DonkeyCaveLowerExit => DonkeyCaveLower,
            Door::DonkeyCaveMiddleEntrance => DeathSecondFloor,
            Door::DonkeyCaveMiddleExit => DonkeyCaveUpper,
            Door::DonkeyCaveUpperEntrance => DeathWestLedge,
            Door::DonkeyCaveUpperExit => DonkeyCaveUpper,
            Door::BigRollingRocksCaveLowerEntrance => DeathSecondFloor,
            Door::BigRollingRocksCaveLowerExit => BigRollingRocksCaveLower,
            Door::BigRollingRocksCaveMiddleRightEntrance => DeathThirdFloor,
            Door::BigRollingRocksCaveMiddleRightExit => BigRollingRocksCaveLower,
            Door::BigRollingRocksCaveMiddleLeftEntrance => DeathThirdFloor,
            Door::BigRollingRocksCaveMiddleLeftExit => BigRollingRocksCaveUpper,
            Door::BigRollingRocksCaveUpperEntrance => DeathTopLeftLedge,
            Door::BigRollingRocksCaveUpperExit => BigRollingRocksCaveUpper,
            Door::SpectacleRockCaveLeftEntrance => SpectacleRock,
            Door::SpectacleRockCaveLeftExit => SpectacleRockCaveLeft,
            Door::SpectacleRockCaveRightEntrance => DeathMountainWestTop,
            Door::SpectacleRockCaveRightExit => SpectacleRockCaveRight,
            Door::HookshotMiniDungeonEntrance => DeathMountainEastTop,
            Door::HookshotMiniDungeonExit => HookshotDungeon,
            Door::FireCaveTopEntrance => DeathMountainEastTop,
            Door::FireCaveTopExit => FireCaveTop,
            Door::FireCaveMiddleLeftEntrance => BoulderingLedgeLeft,
            Door::FireCaveMiddleLeftExit => FireCaveMiddle,
            Door::FireCaveMiddleRightEntrance => BoulderingLedgeBottom,
            Door::FireCaveMiddleRightExit => FireCaveMiddle,
            Door::FireCaveBottomEntrance => RossosOreMine,
            Door::FireCaveBottomExit => FireCaveBottom,
            Door::DeathMountainBigFairyCaveEntrance => RossosOreMine,
            Door::DeathMountainBigFairyCaveExit => RossosOreMineBigFairyCave,

            // Lorule ------------------------------------------------------------------------------

            // Skull Woods
            Door::MysteriousManCaveEntrance => SkullWoodsOverworld,
            Door::MysteriousManCaveExit => MysteriousManCave,

            // Lorule Death Mountain
            Door::LoruleDeathMountainBigFairyCaveEntrance => LoruleDeathWest,
            Door::LoruleDeathMountainBigFairyCaveExit => LoruleDeathWestBigFairyCave,

            Door::IceCaveLowerEntrance => RossosOreMineLorule,
            Door::IceCaveLowerExit => IceCaveEast,
            Door::IceCaveMiddleLeftEntrance => LoruleDeathEastLedgeUpper,
            Door::IceCaveMiddleLeftExit => IceCaveSouthWest,
            Door::IceCaveMiddleRightEntrance => LoruleDeathEastLedgeLower,
            Door::IceCaveMiddleRightExit => IceCaveSouth,
            Door::IceCaveUpperEntrance => LoruleDeathEastTop,
            Door::IceCaveUpperExit => IceCaveCenter,

            // Graveyard
            Door::PhilosophersCaveEntrance => LoruleGraveyard,
            Door::PhilosophersCaveExit => PhilosophersCaveLower,
            Door::LoruleSewersEntrance => LoruleGraveyard,
            Door::LoruleSewersExit => LoruleSanctuary,

            // Dark Ruins
            Door::HinoxCaveEntrance => HinoxCaveShallowWater,
            Door::HinoxCaveExit => HinoxCave,
            Door::DarkRuinsFairyCaveEntrance => DarkRuins,
            Door::DarkRuinsFairyCaveExit => DarkRuinsFairyCave,
            Door::DarkRuinsBigFairyCaveEntrance => DarkRuinsBigFairyLedge,
            Door::DarkRuinsBigFairyCaveExit => DarkRuinsBigFairyCave,

            // Turtle Rock Area
            Door::TurtleRockFairyCaveEntrance => LoruleLakeEast,
            Door::TurtleRockFairyCaveExit => LoruleLakeFairyFountain,
            Door::LoruleLakeItemShopEntrance => LoruleLakeNorthWest,
            Door::LoruleLakeItemShopExit => LoruleLakesideItemShop,

            // Lorule Castle Area
            Door::VacantHouseFrontEntrance => LoruleCastleArea,
            Door::VacantHouseFrontExit => VacantHouseBottom,
            Door::ThiefGirlCaveEntrance => LoruleCastleArea,
            Door::ThiefGirlCaveExit => ThiefGirlCave,
            Door::SwampCaveEntrance => LoruleCastleArea,
            Door::SwampCaveExit => SwampCave,
            Door::BombFlowerCaveEntrance => LoruleCastleArea,
            Door::BombFlowerCaveExit => BigBombCave,
            Door::GreatRupeeFairyCaveEntrance => LoruleCastleArea,
            Door::GreatRupeeFairyCaveExit => GreatRupeeFairyCave,
            Door::BombFlowerShopEntrance => LoruleCastleArea,
            Door::BombFlowerShopExit => BigBombFlowerShop,
            Door::LoruleBlacksmithEntrance => LoruleCastleArea,
            Door::LoruleBlacksmithExit => LoruleBlacksmith,
            Door::LoruleFortuneTellerEntrance => LoruleCastleArea,
            Door::LoruleFortuneTellerExit => FortuneTellerLorule,
            Door::LoruleMilkBarEntrance => LoruleCastleArea,
            Door::LoruleMilkBarExit => MilkBarLorule,
            Door::VeteransHouseEntrance => LoruleCastleArea,
            Door::VeteransHouseExit => VeteranThiefsHouse,
            Door::LoruleFortunesChoiceEntrance => LoruleCastleArea,
            Door::LoruleFortunesChoiceExit => FortunesChoiceLorule,
            Door::ThievesTownItemShopEntrance => LoruleCastleArea,
            Door::ThievesTownItemShopExit => ThievesTownItemShop,

            // Misery Mire
            Door::SandRodMiniDungeonEntrance => MiseryMire,
            Door::SandRodMiniDungeonExit => SandRodDungeon,
        }
    }

    pub fn get_spawn_point(&self) -> SpawnPoint {
        use game::Course::*;
        let (course, scene, spawn) = match self {
            // Dungeons
            Door::EasternPalaceEntrance => (FieldLight, 20, 0),
            Door::EasternPalaceExit => (DungeonEast, 1, 0),
            Door::HouseOfGalesEntrance => (FieldLight, 35, 0),
            Door::HouseOfGalesExit => (DungeonWind, 1, 0),
            Door::TowerOfHeraEntrance => (FieldLight, 3, 3),
            Door::TowerOfHeraExit => (DungeonHera, 1, 0),
            Door::InsideHyruleCastleEntrance => (FieldLight, 18, 0),
            Door::InsideHyruleCastleExit => (DungeonCastle, 1, 0),
            Door::DarkPalaceEntrance => (FieldDark, 20, 5),
            Door::DarkPalaceExit => (DungeonDark, 2, 0),
            Door::SwampPalaceEntrance => (FieldDark, 33, 0),
            Door::SwampPalaceExit => (CaveDark, 1, 1),
            Door::SkullWoodsEntrance => (FieldDark, 1, 10),
            Door::SkullWoodsExit => (DungeonDokuro, 1, 0),
            Door::ThievesHideoutEntrance => (FieldDark, 16, 5),
            Door::ThievesHideoutExit => (DungeonHagure, 1, 0),
            Door::TurtleRockEntrance => (FieldDark, 35, 6),
            Door::TurtleRockExit => (DungeonKame, 1, 0),
            Door::DesertPalaceEntrance => (FieldLight, 31, 2),
            Door::DesertPalaceExit => (DungeonSand, 1, 0),
            Door::IceRuinsEntrance => (FieldDark, 5, 0),
            Door::IceRuinsExit => (DungeonIce, 1, 0),
            Door::LoruleCastleEntrance => (FieldDark, 18, 0),
            Door::LoruleCastleExit => (DungeonGanon, 1, 0),

            // Back Entrances
            Door::CuccoHouseBackEntrance => (FieldLight, 16, 19),
            Door::CuccoHouseBackExit => (IndoorLight, 9, 1),
            Door::VacantHouseBackEntrance => (FieldDark, 27, 7),
            Door::VacantHouseBackExit => (IndoorDark, 11, 1),
            Door::WitchCaveBackEntrance => (FieldLight, 14, 4),
            Door::WitchCaveBackExit => (CaveLight, 30, 1),
            Door::IceCaveBackEntrance => (FieldDark, 4, 4),
            Door::IceCaveBackExit => (CaveDark, 9, 4),

            // Hyrule ------------------------------------------------------------------------------

            // Hyrule Castle Area
            Door::YourHouseEntrance => (FieldLight, 27, 5),
            Door::YourHouseExit => (IndoorLight, 1, 1),
            Door::BlacksmithEntrance => (FieldLight, 21, 4),
            Door::BlacksmithExit => (IndoorLight, 19, 0),
            Door::BlacksmithCaveEntrance => (FieldLight, 21, 6),
            Door::BlacksmithCaveExit => (CaveLight, 16, 0),
            Door::CuccoMiniDungeonEntrance => (FieldLight, 32, 3),
            Door::CuccoMiniDungeonExit => (AttractionLight, 3, 0),
            Door::HyruleCastleUpperLeftEntrance => (FieldLight, 18, 12),
            Door::HyruleCastleUpperLeftExit => (IndoorLight, 12, 7),
            Door::HyruleCastleUpperRightEntrance => (FieldLight, 18, 11),
            Door::HyruleCastleUpperRightExit => (IndoorLight, 12, 5),
            Door::HyruleCastleLowerLeftEntrance => (FieldLight, 18, 16),
            Door::HyruleCastleLowerLeftExit => (IndoorLight, 12, 16),
            Door::HyruleCastleLowerRightEntrance => (FieldLight, 18, 15),
            Door::HyruleCastleLowerRightExit => (IndoorLight, 12, 15),
            Door::HyruleCastleMainEntrance => (FieldLight, 18, 10),
            Door::HyruleCastleMainExit => (IndoorLight, 12, 0),

            // Kakariko Village
            Door::MilkBarEntrance => (FieldLight, 16, 12),
            Door::MilkBarExit => (IndoorLight, 15, 0),
            Door::CuccoHouseFrontEntrance => (FieldLight, 16, 18),
            Door::CuccoHouseFrontExit => (IndoorLight, 9, 0),
            Door::StylishWomansHouseEntrance => (FieldLight, 16, 10),
            Door::StylishWomansHouseExit => (IndoorLight, 14, 0),
            Door::BeeGuyHouseEntrance => (FieldLight, 16, 11),
            Door::BeeGuyHouseExit => (IndoorLight, 17, 0),
            Door::HyruleFortunesChoiceEntrance => (FieldLight, 16, 14),
            Door::HyruleFortunesChoiceExit => (IndoorLight, 20, 0),
            Door::WomanHouseEntrance => (FieldLight, 16, 16),
            Door::WomanHouseExit => (IndoorLight, 21, 0),
            Door::KakarikoItemShopEntrance => (FieldLight, 16, 9),
            Door::KakarikoItemShopExit => (IndoorLight, 8, 0),
            Door::JailEntrance => (FieldLight, 16, 17),
            Door::JailExit => (IndoorLight, 3, 0),
            Door::SahasrahlaLeftEntrance => (FieldLight, 16, 8),
            Door::SahasrahlaLeftExit => (IndoorLight, 16, 1),
            Door::SahasrahlaRightEntrance => (FieldLight, 16, 7),
            Door::SahasrahlaRightExit => (IndoorLight, 16, 0),
            Door::KakarikoCaveEntrance => (FieldLight, 16, 15),
            Door::KakarikoCaveExit => (CaveLight, 4, 0),

            // Southern Ruins
            Door::SouthernRuinsFairyCaveEntrance => (FieldLight, 33, 6),
            Door::SouthernRuinsFairyCaveExit => (CaveLight, 26, 0),
            Door::RunawayItemSellerCaveEntrance => (FieldLight, 33, 7),
            Door::RunawayItemSellerCaveExit => (CaveLight, 27, 0),
            Door::SouthernRuinsMiniDungeonEntrance => (FieldLight, 33, 0),
            Door::SouthernRuinsMiniDungeonExit => (AttractionLight, 2, 0),
            Door::SouthernRuinsBombCaveEntrance => (FieldLight, 33, 8),
            Door::SouthernRuinsBombCaveExit => (CaveLight, 28, 0),
            Door::SouthernRuinsPillarCaveEntrance => (FieldLight, 33, 9),
            Door::SouthernRuinsPillarCaveExit => (CaveLight, 28, 1),

            // Desert
            Door::DesertBigFairyCaveEntrance => (FieldLight, 37, 4),
            Door::DesertBigFairyCaveExit => (CaveLight, 20, 0),
            Door::DesertFairyCaveEntrance => (FieldLight, 31, 17),
            Door::DesertFairyCaveExit => (CaveLight, 8, 0),

            // River Area
            Door::HyruleSewersEntrance => (FieldLight, 12, 3),
            Door::HyruleSewersExit => (CaveLight, 18, 0),
            Door::GraveyardLedgeCaveEntrance => (FieldLight, 12, 4),
            Door::GraveyardLedgeCaveExit => (CaveLight, 5, 0),
            Door::ZorasDomainEntrance => (FieldLight, 7, 0),
            Door::ZorasDomainExit => (CaveLight, 7, 0),
            Door::WaterfallCaveEntrance => (FieldLight, 15, 3),
            Door::WaterfallCaveExit => (CaveLight, 13, 0),
            Door::WitchHouseEntrance => (FieldLight, 14, 3),
            Door::WitchHouseExit => (IndoorLight, 2, 0),
            Door::RiverMiniDungeonEntrance => (FieldLight, 13, 0),
            Door::RiverMiniDungeonExit => (AttractionLight, 5, 0),

            // Eastern Ruins
            Door::MergeMiniDungeonEntrance => (FieldLight, 20, 2),
            Door::MergeMiniDungeonExit => (AttractionLight, 1, 0),
            Door::EasternRuinsFairyCaveEntrance => (FieldLight, 30, 0),
            Door::EasternRuinsFairyCaveExit => (CaveLight, 10, 0),
            Door::EasternRuinsBigFairyCaveEntrance => (FieldLight, 29, 4),
            Door::EasternRuinsBigFairyCaveExit => (CaveLight, 12, 0),
            Door::EasternRuinsCaveTopEntrance => (FieldLight, 20, 6),
            Door::EasternRuinsCaveTopExit => (CaveLight, 29, 1),
            Door::EasternRuinsCaveBottomEntrance => (FieldLight, 20, 7),
            Door::EasternRuinsCaveBottomExit => (CaveLight, 29, 0),
            Door::WitchCaveFrontEntrance => (FieldLight, 20, 8),
            Door::WitchCaveFrontExit => (CaveLight, 30, 0),

            // Lake Hylia
            Door::LakeHyliaDarkCaveEntrance => (FieldLight, 35, 7),
            Door::LakeHyliaDarkCaveExit => (CaveLight, 11, 0),
            Door::LakesideItemShopEntrance => (FieldLight, 35, 6),
            Door::LakesideItemShopExit => (IndoorLight, 6, 0),
            Door::MotherMaiamaiCaveEntrance => (FieldLight, 35, 8),
            Door::MotherMaiamaiCaveExit => (CaveLight, 15, 0),
            Door::IceRodCaveLeftEntrance => (FieldLight, 36, 16),
            Door::IceRodCaveLeftExit => (CaveLight, 9, 1),
            Door::IceRodCaveRightEntrance => (FieldLight, 36, 15),
            Door::IceRodCaveRightExit => (CaveLight, 9, 0),

            // Lost Woods Area
            Door::FortuneTellerTentEntrance => (FieldLight, 9, 4),
            Door::FortuneTellerTentExit => (IndoorLight, 18, 0),
            Door::FortuneTellerCaveEntrance => (FieldLight, 9, 5),
            Door::FortuneTellerCaveExit => (CaveLight, 21, 0),
            Door::RumorGuyCaveEntrance => (FieldLight, 1, 5),
            Door::RumorGuyCaveExit => (CaveLight, 17, 0),
            Door::RossoHouseEntrance => (FieldLight, 2, 0),
            Door::RossoHouseExit => (IndoorLight, 10, 0),
            Door::RossoCaveEntrance => (FieldLight, 2, 3),
            Door::RossoCaveExit => (CaveLight, 6, 0),
            Door::MoldormCaveLowerEntrance => (FieldLight, 6, 3),
            Door::MoldormCaveLowerExit => (CaveLight, 19, 0),
            Door::MoldormCaveLedgeEntrance => (FieldLight, 6, 4),
            Door::MoldormCaveLedgeExit => (CaveLight, 19, 2),

            // Death Mountain
            Door::MoldormCaveTopEntrance => (FieldLight, 3, 0),
            Door::MoldormCaveTopExit => (CaveLight, 19, 1),
            Door::DeathMountainBombCaveEntrance => (FieldLight, 3, 12),
            Door::DeathMountainBombCaveExit => (CaveLight, 3, 2),
            Door::DeathMountainWeatherVaneLeftCaveEntrance => (FieldLight, 3, 13),
            Door::DeathMountainWeatherVaneLeftCaveExit => (CaveLight, 3, 3),
            Door::DeathMountainWestFairyCaveEntrance => (FieldLight, 3, 14),
            Door::DeathMountainWestFairyCaveExit => (CaveLight, 3, 4),
            Door::DonkeyCaveLowerEntrance => (FieldLight, 3, 5),
            Door::DonkeyCaveLowerExit => (CaveLight, 1, 0),
            Door::DonkeyCaveMiddleEntrance => (FieldLight, 3, 7),
            Door::DonkeyCaveMiddleExit => (CaveLight, 1, 2),
            Door::DonkeyCaveUpperEntrance => (FieldLight, 3, 6),
            Door::DonkeyCaveUpperExit => (CaveLight, 1, 1),
            Door::BigRollingRocksCaveLowerEntrance => (FieldLight, 3, 8),
            Door::BigRollingRocksCaveLowerExit => (CaveLight, 2, 0),
            Door::BigRollingRocksCaveMiddleRightEntrance => (FieldLight, 3, 22),
            Door::BigRollingRocksCaveMiddleRightExit => (CaveLight, 2, 22),
            Door::BigRollingRocksCaveMiddleLeftEntrance => (FieldLight, 3, 21),
            Door::BigRollingRocksCaveMiddleLeftExit => (CaveLight, 2, 21),
            Door::BigRollingRocksCaveUpperEntrance => (FieldLight, 3, 23),
            Door::BigRollingRocksCaveUpperExit => (CaveLight, 2, 23),
            Door::SpectacleRockCaveLeftEntrance => (FieldLight, 3, 26),
            Door::SpectacleRockCaveLeftExit => (CaveLight, 3, 6),
            Door::SpectacleRockCaveRightEntrance => (FieldLight, 3, 25),
            Door::SpectacleRockCaveRightExit => (CaveLight, 3, 5),
            Door::HookshotMiniDungeonEntrance => (FieldLight, 5, 0),
            Door::HookshotMiniDungeonExit => (AttractionLight, 4, 0),
            Door::FireCaveTopEntrance => (FieldLight, 4, 2),
            Door::FireCaveTopExit => (CaveLight, 25, 0),
            Door::FireCaveMiddleLeftEntrance => (FieldLight, 4, 3),
            Door::FireCaveMiddleLeftExit => (CaveLight, 25, 4),
            Door::FireCaveMiddleRightEntrance => (FieldLight, 4, 4),
            Door::FireCaveMiddleRightExit => (CaveLight, 25, 5),
            Door::FireCaveBottomEntrance => (FieldLight, 4, 5),
            Door::FireCaveBottomExit => (CaveLight, 25, 6),
            Door::DeathMountainBigFairyCaveEntrance => (FieldLight, 4, 6),
            Door::DeathMountainBigFairyCaveExit => (CaveLight, 24, 0),

            // Lorule ------------------------------------------------------------------------------

            // Skull Woods
            Door::MysteriousManCaveEntrance => (FieldDark, 1, 26),
            Door::MysteriousManCaveExit => (CaveDark, 8, 0),

            // Lorule Death Mountain
            Door::LoruleDeathMountainBigFairyCaveEntrance => (FieldDark, 3, 0),
            Door::LoruleDeathMountainBigFairyCaveExit => (CaveDark, 7, 0),
            Door::IceCaveLowerEntrance => (FieldDark, 4, 6),
            Door::IceCaveLowerExit => (CaveDark, 9, 0),
            Door::IceCaveMiddleLeftEntrance => (FieldDark, 4, 0),
            Door::IceCaveMiddleLeftExit => (CaveDark, 9, 2),
            Door::IceCaveMiddleRightEntrance => (FieldDark, 4, 1),
            Door::IceCaveMiddleRightExit => (CaveDark, 9, 1),
            Door::IceCaveUpperEntrance => (FieldDark, 4, 5),
            Door::IceCaveUpperExit => (CaveDark, 9, 5),

            // Graveyard
            Door::PhilosophersCaveEntrance => (FieldDark, 11, 3),
            Door::PhilosophersCaveExit => (CaveDark, 5, 0),
            Door::LoruleSewersEntrance => (FieldDark, 12, 10),
            Door::LoruleSewersExit => (AttractionDark, 2, 0),

            // Dark Ruins
            Door::HinoxCaveEntrance => (FieldDark, 15, 3),
            Door::HinoxCaveExit => (CaveDark, 6, 0),
            Door::DarkRuinsFairyCaveEntrance => (FieldDark, 29, 4),
            Door::DarkRuinsFairyCaveExit => (CaveDark, 12, 0),
            Door::DarkRuinsBigFairyCaveEntrance => (FieldDark, 30, 2),
            Door::DarkRuinsBigFairyCaveExit => (CaveDark, 13, 0),

            // Turtle Rock Area
            Door::TurtleRockFairyCaveEntrance => (FieldDark, 36, 3),
            Door::TurtleRockFairyCaveExit => (CaveDark, 11, 0),
            Door::LoruleLakeItemShopEntrance => (FieldDark, 35, 5),
            Door::LoruleLakeItemShopExit => (IndoorDark, 9, 0),

            // Lorule Castle Area
            Door::VacantHouseFrontEntrance => (FieldDark, 27, 0),
            Door::VacantHouseFrontExit => (IndoorDark, 11, 0),
            Door::ThiefGirlCaveEntrance => (FieldDark, 33, 10),
            Door::ThiefGirlCaveExit => (CaveDark, 15, 0),
            Door::SwampCaveEntrance => (FieldDark, 33, 6),
            Door::SwampCaveExit => (CaveDark, 3, 0),
            Door::BombFlowerCaveEntrance => (FieldDark, 32, 3),
            Door::BombFlowerCaveExit => (CaveDark, 2, 0),
            Door::GreatRupeeFairyCaveEntrance => (FieldDark, 24, 6),
            Door::GreatRupeeFairyCaveExit => (CaveDark, 14, 0),
            Door::BombFlowerShopEntrance => (FieldDark, 24, 0),
            Door::BombFlowerShopExit => (IndoorDark, 3, 1),
            Door::LoruleBlacksmithEntrance => (FieldDark, 21, 2),
            Door::LoruleBlacksmithExit => (IndoorDark, 4, 0),
            Door::LoruleFortuneTellerEntrance => (FieldDark, 17, 3),
            Door::LoruleFortuneTellerExit => (IndoorDark, 10, 0),
            Door::LoruleMilkBarEntrance => (FieldDark, 16, 0),
            Door::LoruleMilkBarExit => (IndoorDark, 1, 0),
            Door::VeteransHouseEntrance => (FieldDark, 16, 7),
            Door::VeteransHouseExit => (IndoorDark, 16, 0),
            Door::LoruleFortunesChoiceEntrance => (FieldDark, 16, 6),
            Door::LoruleFortunesChoiceExit => (IndoorDark, 14, 0),
            Door::ThievesTownItemShopEntrance => (FieldDark, 16, 9),
            Door::ThievesTownItemShopExit => (IndoorDark, 13, 0),

            // Misery Mire
            Door::SandRodMiniDungeonEntrance => (FieldDark, 37, 4),
            Door::SandRodMiniDungeonExit => (AttractionDark, 3, 0),
        };

        SpawnPoint { course, scene, spawn }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            // Dungeons ----------------------------------------------------------------------------
            Door::EasternPalaceEntrance => "Eastern Palace Entrance",
            Door::EasternPalaceExit => "Eastern Palace Exit",
            Door::HouseOfGalesEntrance => "House of Gales Entrance",
            Door::HouseOfGalesExit => "House of Gales Exit",
            Door::TowerOfHeraEntrance => "Tower of Hera Entrance",
            Door::TowerOfHeraExit => "Tower of Hera Exit",
            Door::InsideHyruleCastleEntrance => "Inside Hyrule Castle Entrance",
            Door::InsideHyruleCastleExit => "Inside Hyrule Castle Exit",
            Door::DarkPalaceEntrance => "Dark Palace Entrance",
            Door::DarkPalaceExit => "Dark Palace Exit",
            Door::SwampPalaceEntrance => "Swamp Palace Entrance",
            Door::SwampPalaceExit => "Swamp Palace Exit",
            Door::SkullWoodsEntrance => "Skull Woods Entrance",
            Door::SkullWoodsExit => "Skull Woods Exit",
            Door::ThievesHideoutEntrance => "Thieves' Hideout Entrance",
            Door::ThievesHideoutExit => "Thieves' Hideout Exit",
            Door::TurtleRockEntrance => "Turtle Rock Entrance",
            Door::TurtleRockExit => "Turtle Rock Exit",
            Door::DesertPalaceEntrance => "Desert Palace Entrance",
            Door::DesertPalaceExit => "Desert Palace Exit",
            Door::IceRuinsEntrance => "Ice Ruins Entrance",
            Door::IceRuinsExit => "Ice Ruins Exit",
            Door::LoruleCastleEntrance => "Lorule Castle Entrance",
            Door::LoruleCastleExit => "Lorule Castle Exit",

            // Back Entrances
            Door::CuccoHouseBackEntrance => "Cucco House Back Entrance",
            Door::CuccoHouseBackExit => "Cucco House Back Exit",
            Door::VacantHouseBackEntrance => "Vacant House Back Entrance",
            Door::VacantHouseBackExit => "Vacant House Back Exit",
            Door::WitchCaveBackEntrance => "Witch Cave Back Entrance",
            Door::WitchCaveBackExit => "Witch Cave Back Exit",
            Door::IceCaveBackEntrance => "Ice Cave Back Entrance",
            Door::IceCaveBackExit => "Ice Cave Back Exit",

            // Hyrule ------------------------------------------------------------------------------

            // Hyrule Castle Area
            Door::YourHouseEntrance => "Your House Entrance",
            Door::YourHouseExit => "Your House Exit",
            Door::BlacksmithEntrance => "Blacksmith Entrance",
            Door::BlacksmithExit => "Blacksmith Exit",
            Door::BlacksmithCaveEntrance => "Blacksmith Cave Entrance",
            Door::BlacksmithCaveExit => "Blacksmith Cave Exit",
            Door::CuccoMiniDungeonEntrance => "Cucco Mini Dungeon Entrance",
            Door::CuccoMiniDungeonExit => "Cucco Mini Dungeon Exit",
            Door::HyruleCastleUpperLeftEntrance => "Hyrule Castle Upper Left Entrance",
            Door::HyruleCastleUpperLeftExit => "Hyrule Castle Upper Left Exit",
            Door::HyruleCastleUpperRightEntrance => "Hyrule Castle Upper Right Entrance",
            Door::HyruleCastleUpperRightExit => "Hyrule Castle Upper Right Exit",
            Door::HyruleCastleLowerLeftEntrance => "Hyrule Castle Lower Left Entrance",
            Door::HyruleCastleLowerLeftExit => "Hyrule Castle Lower Left Exit",
            Door::HyruleCastleLowerRightEntrance => "Hyrule Castle Lower Right Entrance",
            Door::HyruleCastleLowerRightExit => "Hyrule Castle Lower Right Exit",
            Door::HyruleCastleMainEntrance => "Hyrule Castle Main Entrance",
            Door::HyruleCastleMainExit => "Hyrule Castle Main Exit",

            // Kakariko Village
            Door::MilkBarEntrance => "Milk Bar Entrance",
            Door::MilkBarExit => "Milk Bar Exit",
            Door::CuccoHouseFrontEntrance => "Cucco House Front Entrance",
            Door::CuccoHouseFrontExit => "Cucco House Front Exit",
            Door::StylishWomansHouseEntrance => "Stylish Woman's House Entrance",
            Door::StylishWomansHouseExit => "Stylish Woman's House Exit",
            Door::BeeGuyHouseEntrance => "Bee Guy's House Entrance",
            Door::BeeGuyHouseExit => "Bee Guy's House Exit",
            Door::HyruleFortunesChoiceEntrance => "Hyrule Fortune's Choice Entrance",
            Door::HyruleFortunesChoiceExit => "Hyrule Fortune's Choice Exit",
            Door::WomanHouseEntrance => "Woman's House Entrance",
            Door::WomanHouseExit => "Woman's House Exit",
            Door::KakarikoItemShopEntrance => "Kakariko Item Shop Entrance",
            Door::KakarikoItemShopExit => "Kakariko Item Shop Exit",
            Door::JailEntrance => "Jail Entrance",
            Door::JailExit => "Jail Exit",
            Door::SahasrahlaLeftEntrance => "Sahasrahla's House Left Entrance",
            Door::SahasrahlaLeftExit => "Sahasrahla's House Left Exit",
            Door::SahasrahlaRightEntrance => "Sahasrahla's House Right Entrance",
            Door::SahasrahlaRightExit => "Sahasrahla's House Right Exit",
            Door::KakarikoCaveEntrance => "Kakariko Cave Entrance",
            Door::KakarikoCaveExit => "Kakariko Cave Exit",

            // Southern Ruins
            Door::SouthernRuinsFairyCaveEntrance => "Southern Ruins Fairy Cave Entrance",
            Door::SouthernRuinsFairyCaveExit => "Southern Ruins Fairy Cave Exit",
            Door::RunawayItemSellerCaveEntrance => "Runaway Item Seller Cave Entrance",
            Door::RunawayItemSellerCaveExit => "Runaway Item Seller Cave Exit",
            Door::SouthernRuinsMiniDungeonEntrance => "Southern Ruins Mini Dungeon Entrance",
            Door::SouthernRuinsMiniDungeonExit => "Southern Ruins Mini Dungeon Exit",
            Door::SouthernRuinsBombCaveEntrance => "Southern Ruins Bomb Cave Entrance",
            Door::SouthernRuinsBombCaveExit => "Southern Ruins Bomb Cave Exit",
            Door::SouthernRuinsPillarCaveEntrance => "Southern Ruins Pillar Cave Entrance",
            Door::SouthernRuinsPillarCaveExit => "Southern Ruins Pillar Cave Exit",

            // Desert
            Door::DesertBigFairyCaveEntrance => "Desert Big Fairy Cave Entrance",
            Door::DesertBigFairyCaveExit => "Desert Big Fairy Cave Exit",
            Door::DesertFairyCaveEntrance => "Desert Fairy Cave Entrance",
            Door::DesertFairyCaveExit => "Desert Fairy Cave Exit",

            // River Area
            Door::HyruleSewersEntrance => "Hyrule Sewers Entrance",
            Door::HyruleSewersExit => "Hyrule Sewers Exit",
            Door::GraveyardLedgeCaveEntrance => "Graveyard Ledge Cave Entrance",
            Door::GraveyardLedgeCaveExit => "Graveyard Ledge Cave Exit",
            Door::ZorasDomainEntrance => "Zora's Domain Entrance",
            Door::ZorasDomainExit => "Zoras Domain Exit",
            Door::WaterfallCaveEntrance => "Waterfall Cave Entrance",
            Door::WaterfallCaveExit => "Waterfall Cave Exit",
            Door::WitchHouseEntrance => "Witch's House Entrance",
            Door::WitchHouseExit => "Witch's House Exit",
            Door::RiverMiniDungeonEntrance => "River Mini Dungeon Entrance",
            Door::RiverMiniDungeonExit => "River Mini Dungeon Exit",

            // Eastern Ruins
            Door::MergeMiniDungeonEntrance => "Merge Mini Dungeon Entrance",
            Door::MergeMiniDungeonExit => "Merge Mini Dungeon Exit",
            Door::EasternRuinsFairyCaveEntrance => "Eastern Ruins Fairy Cave Entrance",
            Door::EasternRuinsFairyCaveExit => "Eastern Ruins Fairy Cave Exit",
            Door::EasternRuinsBigFairyCaveEntrance => "Eastern Ruins Big Fairy Cave Entrance",
            Door::EasternRuinsBigFairyCaveExit => "Eastern Ruins Big Fairy Cave Exit",
            Door::EasternRuinsCaveTopEntrance => "Eastern Ruins Cave Top Entrance",
            Door::EasternRuinsCaveTopExit => "Eastern Ruins Cave Top Exit",
            Door::EasternRuinsCaveBottomEntrance => "Eastern Ruins Cave Bottom Entrance",
            Door::EasternRuinsCaveBottomExit => "Eastern Ruins Cave Bottom Exit",
            Door::WitchCaveFrontEntrance => "Witch Cave Front Entrance",
            Door::WitchCaveFrontExit => "Witch Cave Front Exit",

            // Lake Hylia
            Door::LakeHyliaDarkCaveEntrance => "Lake Hylia Dark Cave Entrance",
            Door::LakeHyliaDarkCaveExit => "Lake Hylia Dark Cave Exit",
            Door::LakesideItemShopEntrance => "Lake Hylia Item Shop Entrance",
            Door::LakesideItemShopExit => "Lake Hylia Item Shop Exit",
            Door::MotherMaiamaiCaveEntrance => "Mother Maiamai Cave Entrance",
            Door::MotherMaiamaiCaveExit => "Mother Maiamai Cave Exit",
            Door::IceRodCaveLeftEntrance => "Ice Rod Cave Left Entrance",
            Door::IceRodCaveLeftExit => "Ice Rod Cave Left Exit",
            Door::IceRodCaveRightEntrance => "Ice Rod Cave Right Entrance",
            Door::IceRodCaveRightExit => "Ice Rod Cave Right Exit",

            // Lost Woods Area
            Door::FortuneTellerTentEntrance => "Fortune-Teller Tent Entrance",
            Door::FortuneTellerTentExit => "Fortune-Teller Tent Exit",
            Door::FortuneTellerCaveEntrance => "Fortune-Teller Cave Entrance",
            Door::FortuneTellerCaveExit => "Fortune-Teller Cave Exit",
            Door::RumorGuyCaveEntrance => "Rumor Guy Cave Entrance",
            Door::RumorGuyCaveExit => "Rumor Guy Cave Exit",
            Door::RossoHouseEntrance => "Rosso House Entrance",
            Door::RossoHouseExit => "Rosso House Exit",
            Door::RossoCaveEntrance => "Rosso Cave Entrance",
            Door::RossoCaveExit => "Rosso Cave Exit",
            Door::MoldormCaveLowerEntrance => "Spiral Cave Lower Entrance",
            Door::MoldormCaveLowerExit => "Spiral Cave Lower Exit",
            Door::MoldormCaveLedgeEntrance => "Spiral Cave Middle Entrance",
            Door::MoldormCaveLedgeExit => "Spiral Cave Middle Exit",

            // Death Mountain
            Door::MoldormCaveTopEntrance => "Spiral Cave Top Entrance",
            Door::MoldormCaveTopExit => "Spiral Cave Top Exit",
            Door::DeathMountainBombCaveEntrance => "Death Mountain Bomb Cave Entrance",
            Door::DeathMountainBombCaveExit => "Death Mountain Bomb Cave Exit",
            Door::DeathMountainWeatherVaneLeftCaveEntrance => "Death Mountain Weather Vane Left Cave Entrance",
            Door::DeathMountainWeatherVaneLeftCaveExit => "Death Mountain Weather Vane Left Cave Exit",
            Door::DeathMountainWestFairyCaveEntrance => "Death Mountain West Fairy Cave Entrance",
            Door::DeathMountainWestFairyCaveExit => "Death Mountain West Fairy Cave Exit",
            Door::DonkeyCaveLowerEntrance => "Donkey Cave Lower Entrance",
            Door::DonkeyCaveLowerExit => "Donkey Cave Lower Exit",
            Door::DonkeyCaveMiddleEntrance => "Donkey Cave Middle Entrance",
            Door::DonkeyCaveMiddleExit => "Donkey Cave Middle Exit",
            Door::DonkeyCaveUpperEntrance => "Donkey Cave Upper Entrance",
            Door::DonkeyCaveUpperExit => "Donkey Cave Upper Exit",
            Door::BigRollingRocksCaveLowerEntrance => "Big Rolling Rocks Cave Lower Entrance",
            Door::BigRollingRocksCaveLowerExit => "Big Rolling Rocks Cave Lower Exit",
            Door::BigRollingRocksCaveMiddleRightEntrance => "Big Rolling Rocks Cave Middle Right Entrance",
            Door::BigRollingRocksCaveMiddleRightExit => "Big Rolling Rocks Cave Middle Right Exit",
            Door::BigRollingRocksCaveMiddleLeftEntrance => "Big Rolling Rocks Cave Middle Left Entrance",
            Door::BigRollingRocksCaveMiddleLeftExit => "Big Rolling Rocks Cave Middle Left Exit",
            Door::BigRollingRocksCaveUpperEntrance => "Big Rolling Rocks Cave Upper Entrance",
            Door::BigRollingRocksCaveUpperExit => "Big Rolling Rocks Cave Upper Exit",
            Door::SpectacleRockCaveLeftEntrance => "Spectacle Rock Cave Left Entrance",
            Door::SpectacleRockCaveLeftExit => "Spectacle Rock Cave Left Exit",
            Door::SpectacleRockCaveRightEntrance => "Spectacle Rock Cave Right Entrance",
            Door::SpectacleRockCaveRightExit => "Spectacle Rock Cave Right Exit",
            Door::HookshotMiniDungeonEntrance => "Hookshot Mini Dungeon Entrance",
            Door::HookshotMiniDungeonExit => "Hookshot Mini Dungeon Exit",
            Door::FireCaveTopEntrance => "Fire Cave Top Entrance",
            Door::FireCaveTopExit => "Fire Cave Top Exit",
            Door::FireCaveMiddleLeftEntrance => "Fire Cave Middle Left Entrance",
            Door::FireCaveMiddleLeftExit => "Fire Cave Middle Left Exit",
            Door::FireCaveMiddleRightEntrance => "Fire Cave Middle Right Entrance",
            Door::FireCaveMiddleRightExit => "Fire Cave Middle Right Exit",
            Door::FireCaveBottomEntrance => "Fire Cave Bottom Entrance",
            Door::FireCaveBottomExit => "Fire Cave Bottom Exit",
            Door::DeathMountainBigFairyCaveEntrance => "Death Mountain Big Fairy Cave Entrance",
            Door::DeathMountainBigFairyCaveExit => "Death Mountain Big Fairy Cave Exit",

            // Lorule ------------------------------------------------------------------------------

            // Skull Woods
            Door::MysteriousManCaveEntrance => "Mysterious Man Cave Entrance",
            Door::MysteriousManCaveExit => "Mysterious Man Cave Exit",

            // Lorule Death Mountain
            Door::LoruleDeathMountainBigFairyCaveEntrance => "Lorule Death Mountain Big Fairy Cave Entrance",
            Door::LoruleDeathMountainBigFairyCaveExit => "Lorule Death Mountain Big Fairy Cave Exit",
            Door::IceCaveLowerEntrance => "Ice Cave Lower Entrance",
            Door::IceCaveLowerExit => "Ice Cave Lower Exit",
            Door::IceCaveMiddleLeftEntrance => "Ice Cave Middle Left Entrance",
            Door::IceCaveMiddleLeftExit => "Ice Cave Middle Left Exit",
            Door::IceCaveMiddleRightEntrance => "Ice Cave Middle Right Entrance",
            Door::IceCaveMiddleRightExit => "Ice Cave Middle Right Exit",
            Door::IceCaveUpperEntrance => "Ice Cave Upper Entrance",
            Door::IceCaveUpperExit => "Ice Cave Upper Exit",

            // Graveyard
            Door::PhilosophersCaveEntrance => "Philosopher's Cave Entrance",
            Door::PhilosophersCaveExit => "Philosopher's Cave Exit",
            Door::LoruleSewersEntrance => "Lorule Sewers Entrance",
            Door::LoruleSewersExit => "Lorule Sewers Exit",

            // Dark Ruins
            Door::HinoxCaveEntrance => "Hinox Cave Entrance",
            Door::HinoxCaveExit => "Hinox Cave Exit",
            Door::DarkRuinsFairyCaveEntrance => "Dark Ruins Fairy Cave Entrance",
            Door::DarkRuinsFairyCaveExit => "Dark Ruins Fairy Cave Exit",
            Door::DarkRuinsBigFairyCaveEntrance => "Dark Ruins Big Fairy Cave Entrance",
            Door::DarkRuinsBigFairyCaveExit => "Dark Ruins Big Fairy Cave Exit",

            // Turtle Rock
            Door::TurtleRockFairyCaveEntrance => "Turtle Rock Fairy Cave Entrance",
            Door::TurtleRockFairyCaveExit => "Turtle Rock Fairy Cave Exit",
            Door::LoruleLakeItemShopEntrance => "Lorule Lake Item Shop Entrance",
            Door::LoruleLakeItemShopExit => "Lorule Lake Item Shop Exit",

            // Lorule Castle Area
            Door::VacantHouseFrontEntrance => "Vacant House Front Entrance",
            Door::VacantHouseFrontExit => "Vacant House Front Exit",
            Door::ThiefGirlCaveEntrance => "Thief Girl Cave Entrance",
            Door::ThiefGirlCaveExit => "Thief Girl Cave Exit",
            Door::SwampCaveEntrance => "Swamp Cave Entrance",
            Door::SwampCaveExit => "Swamp Cave Exit",
            Door::BombFlowerCaveEntrance => "Bomb Flower Cave Entrance",
            Door::BombFlowerCaveExit => "Bomb Flower Cave Exit",
            Door::GreatRupeeFairyCaveEntrance => "Great Rupee Fairy Cave Entrance",
            Door::GreatRupeeFairyCaveExit => "Great Rupee Fairy Cave Exit",
            Door::BombFlowerShopEntrance => "Bomb Flower Shop Entrance",
            Door::BombFlowerShopExit => "Bomb Flower Shop Exit",
            Door::LoruleBlacksmithEntrance => "Lorule Blacksmith Entrance",
            Door::LoruleBlacksmithExit => "Lorule Blacksmith Exit",
            Door::LoruleFortuneTellerEntrance => "Lorule Fortune-Teller Entrance",
            Door::LoruleFortuneTellerExit => "Lorule Fortune-Teller Exit",
            Door::LoruleMilkBarEntrance => "Lorule Milk Bar Entrance",
            Door::LoruleMilkBarExit => "Lorule Milk Bar Exit",
            Door::VeteransHouseEntrance => "Veteran's House Entrance",
            Door::VeteransHouseExit => "Veteran's House Exit",
            Door::LoruleFortunesChoiceEntrance => "Lorule Fortune's Choice Entrance",
            Door::LoruleFortunesChoiceExit => "Lorule Fortune's Choice Exit",
            Door::ThievesTownItemShopEntrance => "Thieves' Town Item Shop Entrance",
            Door::ThievesTownItemShopExit => "Thieves' Town Item Shop Exit",

            // Misery Mire
            Door::SandRodMiniDungeonEntrance => "Sand Rod Mini Dungeon Entrance",
            Door::SandRodMiniDungeonExit => "Sand Rod Mini Dungeon Exit",
        }
    }

    pub fn get_world(&self) -> game::World {
        match self {
            Door::EasternPalaceEntrance
            | Door::EasternPalaceExit
            | Door::HouseOfGalesEntrance
            | Door::HouseOfGalesExit
            | Door::TowerOfHeraEntrance
            | Door::TowerOfHeraExit
            | Door::InsideHyruleCastleEntrance
            | Door::InsideHyruleCastleExit
            | Door::CuccoHouseBackEntrance
            | Door::CuccoHouseBackExit
            | Door::WitchCaveBackEntrance
            | Door::WitchCaveBackExit
            | Door::YourHouseEntrance
            | Door::YourHouseExit
            | Door::BlacksmithEntrance
            | Door::BlacksmithExit
            | Door::BlacksmithCaveEntrance
            | Door::BlacksmithCaveExit
            | Door::CuccoMiniDungeonEntrance
            | Door::CuccoMiniDungeonExit
            | Door::HyruleCastleUpperLeftEntrance
            | Door::HyruleCastleUpperLeftExit
            | Door::HyruleCastleUpperRightEntrance
            | Door::HyruleCastleUpperRightExit
            | Door::HyruleCastleLowerLeftEntrance
            | Door::HyruleCastleLowerLeftExit
            | Door::HyruleCastleLowerRightEntrance
            | Door::HyruleCastleLowerRightExit
            | Door::HyruleCastleMainEntrance
            | Door::HyruleCastleMainExit
            | Door::MilkBarEntrance
            | Door::MilkBarExit
            | Door::CuccoHouseFrontEntrance
            | Door::CuccoHouseFrontExit
            | Door::StylishWomansHouseEntrance
            | Door::StylishWomansHouseExit
            | Door::BeeGuyHouseEntrance
            | Door::BeeGuyHouseExit
            | Door::HyruleFortunesChoiceEntrance
            | Door::HyruleFortunesChoiceExit
            | Door::WomanHouseEntrance
            | Door::WomanHouseExit
            | Door::KakarikoItemShopEntrance
            | Door::KakarikoItemShopExit
            | Door::JailEntrance
            | Door::JailExit
            | Door::SahasrahlaLeftEntrance
            | Door::SahasrahlaLeftExit
            | Door::SahasrahlaRightEntrance
            | Door::SahasrahlaRightExit
            | Door::KakarikoCaveEntrance
            | Door::KakarikoCaveExit
            | Door::SouthernRuinsFairyCaveEntrance
            | Door::SouthernRuinsFairyCaveExit
            | Door::RunawayItemSellerCaveEntrance
            | Door::RunawayItemSellerCaveExit
            | Door::SouthernRuinsMiniDungeonEntrance
            | Door::SouthernRuinsMiniDungeonExit
            | Door::SouthernRuinsBombCaveEntrance
            | Door::SouthernRuinsBombCaveExit
            | Door::SouthernRuinsPillarCaveEntrance
            | Door::SouthernRuinsPillarCaveExit
            | Door::DesertBigFairyCaveEntrance
            | Door::DesertBigFairyCaveExit
            | Door::DesertFairyCaveEntrance
            | Door::DesertFairyCaveExit
            | Door::HyruleSewersEntrance
            | Door::HyruleSewersExit
            | Door::GraveyardLedgeCaveEntrance
            | Door::GraveyardLedgeCaveExit
            | Door::ZorasDomainEntrance
            | Door::ZorasDomainExit
            | Door::WaterfallCaveEntrance
            | Door::WaterfallCaveExit
            | Door::WitchHouseEntrance
            | Door::WitchHouseExit
            | Door::RiverMiniDungeonEntrance
            | Door::RiverMiniDungeonExit
            | Door::MergeMiniDungeonEntrance
            | Door::MergeMiniDungeonExit
            | Door::EasternRuinsFairyCaveEntrance
            | Door::EasternRuinsFairyCaveExit
            | Door::EasternRuinsBigFairyCaveEntrance
            | Door::EasternRuinsBigFairyCaveExit
            | Door::EasternRuinsCaveTopEntrance
            | Door::EasternRuinsCaveTopExit
            | Door::EasternRuinsCaveBottomEntrance
            | Door::EasternRuinsCaveBottomExit
            | Door::WitchCaveFrontEntrance
            | Door::WitchCaveFrontExit
            | Door::LakeHyliaDarkCaveEntrance
            | Door::LakeHyliaDarkCaveExit
            | Door::LakesideItemShopEntrance
            | Door::LakesideItemShopExit
            | Door::MotherMaiamaiCaveEntrance
            | Door::MotherMaiamaiCaveExit
            | Door::IceRodCaveLeftEntrance
            | Door::IceRodCaveLeftExit
            | Door::IceRodCaveRightEntrance
            | Door::IceRodCaveRightExit
            | Door::FortuneTellerTentEntrance
            | Door::FortuneTellerTentExit
            | Door::FortuneTellerCaveEntrance
            | Door::FortuneTellerCaveExit
            | Door::RumorGuyCaveEntrance
            | Door::RumorGuyCaveExit
            | Door::RossoHouseEntrance
            | Door::RossoHouseExit
            | Door::RossoCaveEntrance
            | Door::RossoCaveExit
            | Door::MoldormCaveLowerEntrance
            | Door::MoldormCaveLowerExit
            | Door::MoldormCaveLedgeEntrance
            | Door::MoldormCaveLedgeExit
            | Door::MoldormCaveTopEntrance
            | Door::MoldormCaveTopExit
            | Door::DeathMountainBombCaveEntrance
            | Door::DeathMountainBombCaveExit
            | Door::DeathMountainWeatherVaneLeftCaveEntrance
            | Door::DeathMountainWeatherVaneLeftCaveExit
            | Door::DeathMountainWestFairyCaveEntrance
            | Door::DeathMountainWestFairyCaveExit
            | Door::DonkeyCaveLowerEntrance
            | Door::DonkeyCaveLowerExit
            | Door::DonkeyCaveMiddleEntrance
            | Door::DonkeyCaveMiddleExit
            | Door::DonkeyCaveUpperEntrance
            | Door::DonkeyCaveUpperExit
            | Door::BigRollingRocksCaveLowerEntrance
            | Door::BigRollingRocksCaveLowerExit
            | Door::BigRollingRocksCaveMiddleRightEntrance
            | Door::BigRollingRocksCaveMiddleRightExit
            | Door::BigRollingRocksCaveMiddleLeftEntrance
            | Door::BigRollingRocksCaveMiddleLeftExit
            | Door::BigRollingRocksCaveUpperEntrance
            | Door::BigRollingRocksCaveUpperExit
            | Door::SpectacleRockCaveLeftEntrance
            | Door::SpectacleRockCaveLeftExit
            | Door::SpectacleRockCaveRightEntrance
            | Door::SpectacleRockCaveRightExit
            | Door::HookshotMiniDungeonEntrance
            | Door::HookshotMiniDungeonExit
            | Door::FireCaveTopEntrance
            | Door::FireCaveTopExit
            | Door::FireCaveMiddleLeftEntrance
            | Door::FireCaveMiddleLeftExit
            | Door::FireCaveMiddleRightEntrance
            | Door::FireCaveMiddleRightExit
            | Door::FireCaveBottomEntrance
            | Door::FireCaveBottomExit
            | Door::DeathMountainBigFairyCaveEntrance
            | Door::DeathMountainBigFairyCaveExit => game::World::Hyrule,

            // --- //
            Door::DarkPalaceEntrance
            | Door::DarkPalaceExit
            | Door::SwampPalaceEntrance
            | Door::SwampPalaceExit
            | Door::SkullWoodsEntrance
            | Door::SkullWoodsExit
            | Door::ThievesHideoutEntrance
            | Door::ThievesHideoutExit
            | Door::TurtleRockEntrance
            | Door::TurtleRockExit
            | Door::DesertPalaceEntrance
            | Door::DesertPalaceExit
            | Door::IceRuinsEntrance
            | Door::IceRuinsExit
            | Door::LoruleCastleEntrance
            | Door::VacantHouseBackEntrance
            | Door::VacantHouseBackExit
            | Door::IceCaveBackEntrance
            | Door::IceCaveBackExit
            | Door::LoruleCastleExit
            | Door::MysteriousManCaveEntrance
            | Door::MysteriousManCaveExit
            | Door::LoruleDeathMountainBigFairyCaveEntrance
            | Door::LoruleDeathMountainBigFairyCaveExit
            | Door::IceCaveLowerEntrance
            | Door::IceCaveLowerExit
            | Door::IceCaveMiddleLeftEntrance
            | Door::IceCaveMiddleLeftExit
            | Door::IceCaveMiddleRightEntrance
            | Door::IceCaveMiddleRightExit
            | Door::IceCaveUpperEntrance
            | Door::IceCaveUpperExit
            | Door::PhilosophersCaveEntrance
            | Door::PhilosophersCaveExit
            | Door::LoruleSewersEntrance
            | Door::LoruleSewersExit
            | Door::HinoxCaveEntrance
            | Door::HinoxCaveExit
            | Door::DarkRuinsFairyCaveEntrance
            | Door::DarkRuinsFairyCaveExit
            | Door::DarkRuinsBigFairyCaveEntrance
            | Door::DarkRuinsBigFairyCaveExit
            | Door::TurtleRockFairyCaveEntrance
            | Door::TurtleRockFairyCaveExit
            | Door::LoruleLakeItemShopEntrance
            | Door::LoruleLakeItemShopExit
            | Door::VacantHouseFrontEntrance
            | Door::VacantHouseFrontExit
            | Door::ThiefGirlCaveEntrance
            | Door::ThiefGirlCaveExit
            | Door::SwampCaveEntrance
            | Door::SwampCaveExit
            | Door::BombFlowerCaveEntrance
            | Door::BombFlowerCaveExit
            | Door::GreatRupeeFairyCaveEntrance
            | Door::GreatRupeeFairyCaveExit
            | Door::BombFlowerShopEntrance
            | Door::BombFlowerShopExit
            | Door::LoruleBlacksmithEntrance
            | Door::LoruleBlacksmithExit
            | Door::LoruleFortuneTellerEntrance
            | Door::LoruleFortuneTellerExit
            | Door::LoruleMilkBarEntrance
            | Door::LoruleMilkBarExit
            | Door::VeteransHouseEntrance
            | Door::VeteransHouseExit
            | Door::LoruleFortunesChoiceEntrance
            | Door::LoruleFortunesChoiceExit
            | Door::ThievesTownItemShopEntrance
            | Door::ThievesTownItemShopExit
            | Door::SandRodMiniDungeonEntrance
            | Door::SandRodMiniDungeonExit => game::World::Lorule,
        }
    }
}

impl Display for Door {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl Ord for Door {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd<Door> for Door {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<Randomizable> for Door {
    fn from(filler_item: Randomizable) -> Self {
        match filler_item {
            Randomizable::Door(door) => door,
            _ => unreachable!("Not a Door: {:?}", filler_item),
        }
    }
}

impl Serialize for Door {
    fn serialize<S>(&self, serializer: S) -> crate::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

pub(crate) fn build_door_map(settings: &Settings, rng: &mut StdRng) -> crate::Result<DoorMap> {
    info!("Building Door Map...");
    let mut door_map: DashMap<_, _> = Default::default();

    match settings.door_shuffle {
        DoorShuffle::Off => {
            // Set all doors to vanilla
            let entrances = vec![
                item_pools::get_dungeon_door_entrances(),
                item_pools::get_non_dungeon_front_door_entrances(),
                item_pools::get_back_door_entrances(),
                item_pools::get_connector_entrances(),
            ]
            .concat();

            for entrance in entrances {
                let exit = entrance.get_vanilla_destination();

                door_map.insert(entrance, exit);
                door_map.insert(exit, entrance);
            }
        },
        DoorShuffle::DungeonEntrances => {
            // Dungeon Doors
            let dungeon_entrances = item_pools::get_dungeon_door_entrances();
            let dungeon_exits = filler::util::shuffle(rng, item_pools::get_dungeon_door_exits());
            randomly_pair_doors(dungeon_entrances, dungeon_exits, &mut door_map, rng);

            // Other Doors (vanilla)
            let other_doors = vec![
                item_pools::get_non_dungeon_front_door_entrances(),
                item_pools::get_back_door_entrances(),
                item_pools::get_connector_entrances(),
            ]
            .concat();
            set_doors_to_vanilla(other_doors, &mut door_map);
        },
        DoorShuffle::Crossed => {
            let entrances_that_need_connectors = get_entrances_that_need_connectors(settings, rng);

            let connector_entrances: Vec<Door> = item_pools::get_connector_entrances();
            let mut connector_exits = item_pools::get_connector_exits();

            let mut used_exits = vec![];
            for entrance_that_needs_connector in entrances_that_need_connectors.clone() {
                let exit = connector_exits.remove(rng.gen_range(0..connector_exits.len()));
                used_exits.push(exit);
                door_map.insert(entrance_that_needs_connector, exit);
                door_map.insert(exit, entrance_that_needs_connector);
            }

            // Front Doors
            let front_entrances = vec![
                item_pools::get_dungeon_door_entrances(),
                item_pools::get_non_dungeon_front_door_entrances(),
                connector_entrances,
            ]
            .concat();
            let front_entrances = front_entrances
                .iter()
                .filter_map(
                    |entrance| if entrances_that_need_connectors.contains(entrance) { None } else { Some(*entrance) },
                )
                .collect();

            // Front Exits
            let front_exits = vec![
                item_pools::get_dungeon_door_exits(),
                item_pools::get_non_dungeon_front_door_exits(),
                connector_exits,
            ]
            .concat();
            let front_exits = front_exits
                .iter()
                .filter_map(|exit| if used_exits.contains(exit) { None } else { Some(*exit) })
                .collect();

            randomly_pair_doors(front_entrances, front_exits, &mut door_map, rng);

            // Back Doors
            let back_entrances = item_pools::get_back_door_entrances();
            let back_exits = item_pools::get_back_door_exits();
            randomly_pair_doors(back_entrances, back_exits, &mut door_map, rng);
        },
    }

    Ok(door_map.iter().map(|(&a, &b)| (a, b)).collect())
}

/// Some Entrances NEED to be tied to a connector, or they will otherwise be unreachable and seed generation will fail
fn get_entrances_that_need_connectors(settings: &Settings, rng: &mut StdRng) -> Vec<Door> {
    // These always need connectors as there's no alternative way to reach them
    let mut entrances_that_need_connectors =
        vec![Door::IceCaveMiddleLeftEntrance, Door::SouthernRuinsPillarCaveEntrance, Door::MoldormCaveLedgeEntrance];

    // Fire Cave Left Ledge - Only add if glitch paths aren't considered
    match settings.logic_mode {
        LogicMode::Normal | LogicMode::Hard => entrances_that_need_connectors.push(Door::FireCaveMiddleLeftEntrance),
        _ => {},
    };

    // DM Top - Only add if Weather Vanes don't potentially allow access
    match settings.weather_vanes {
        WeatherVanes::Shuffled | WeatherVanes::Hyrule | WeatherVanes::All => {},
        _ => entrances_that_need_connectors.push(
            vec![
                Door::BigRollingRocksCaveUpperEntrance,
                Door::SpectacleRockCaveRightEntrance,
                Door::TowerOfHeraEntrance,
                Door::FireCaveTopEntrance,
                Door::HookshotMiniDungeonEntrance,
            ]
            .remove(rng.gen_range(0..5)),
        ),
    };

    // East LDM Top - Only add if Weather Vanes don't potentially allow access
    match settings.weather_vanes {
        WeatherVanes::Shuffled | WeatherVanes::Lorule | WeatherVanes::All => {},
        _ => entrances_that_need_connectors
            .push(vec![Door::IceCaveUpperEntrance, Door::IceRuinsEntrance].remove(rng.gen_range(0..2))),
    };

    // Hyrule Castle Roof
    entrances_that_need_connectors.extend([vec![
        Door::HyruleCastleUpperLeftEntrance,
        Door::InsideHyruleCastleEntrance,
        Door::HyruleCastleUpperRightEntrance,
    ]
    .remove(rng.gen_range(0..3))]);

    entrances_that_need_connectors
}

fn randomly_pair_doors(entrances: Vec<Door>, exits: Vec<Door>, door_map: &mut DashMap<Door, Door>, rng: &mut StdRng) {
    let exits = filler::util::shuffle(rng, exits);
    for i in 0..entrances.len() {
        let entrance = *entrances.get(i).unwrap();
        let exit = *exits.get(i).unwrap();

        door_map.insert(entrance, exit);
        door_map.insert(exit, entrance);
    }
}

fn set_doors_to_vanilla(entrances: Vec<Door>, door_map: &mut DashMap<Door, Door>) {
    for entrance in entrances {
        let exit = entrance.get_vanilla_destination();

        door_map.insert(entrance, exit);
        door_map.insert(exit, entrance);
    }
}
