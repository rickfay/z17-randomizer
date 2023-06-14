use {
    crate::{
        csmc::CsmcChest::{Large, Small},
        settings::Settings,
    },
    jack::item::{Item, Item::*},
};

///
pub enum CsmcChest {
    Large,
    Small,
}

///
pub fn get_chest_size(item: Item, _settings: &Settings) -> CsmcChest {
    match item {
        // Empty |
        KeySmall | KeyBoss |
        // Compass |
        // HeartContainer | HeartPiece |
        // RupeeR | RupeeG | RupeeB | RupeeGold | RupeeSilver | RupeePurple |
        ItemIceRod | ItemRentalIceRod | ItemIceRodLv2 |
        ItemSandRod | ItemRentalSandRod | ItemSandRodLv2 | ItemRentalSandRodFirst |
        ItemTornadeRod | ItemRentalTornadeRod | ItemTornadeRodLv2 |
        ItemBomb | ItemRentalBomb | ItemBombLv2 |
        ItemFireRod | ItemRentalFireRod | ItemFireRodLv2 |
        ItemHookShot | ItemRentalHookShot | ItemHookShotLv2 |
        ItemBoomerang | ItemRentalBoomerang | ItemBoomerangLv2 |
        ItemHammer | ItemRentalHammer | ItemHammerLv2 |
        ItemBow | ItemRentalBow | ItemBowLv2 |
        ItemShield | ItemRentalShield | HyruleShield |
        ItemBottle |
        // ItemPotShopRed | ItemPotShopBlue | ItemPotShopPurple | ItemPotShopYellow | Milk |
        ItemStoneBeauty |
        PendantPower | PendantWisdom | PendantCourage |
        ZeldaAmulet |
        ItemKandelaar | ItemKandelaarLv2 |
        ItemSwordLv1 | ItemSwordLv2 | ItemSwordLv3 | ItemSwordLv4 | PackageSword |
        ItemMizukaki |
        RingRental | RingHekiga |
        ItemBell |
        PowerGlove | PowerfulGlove |
        ItemInsectNet | ItemInsectNetLv2 |
        // Kinsta |
        BadgeBee |
        GoldenBee |
        // Bee | Fairy | GoldenBeeForSale |
        HintGlasses |
        EscapeFruit |
        StopFruit |
        // LiverBlue | LiverPurple | LiverYellow |
        ClothesBlue | ClothesRed |
        OreYellow | OreGreen | OreBlue | OreRed |
        GanbariPowerUp |
        // GanbariTubo |
        Pouch |
        DashBoots |
        MessageBottle | MilkMatured |
        SpecialMove |
        ItemBowLight |
        // TriforceCourage |
        // Heart |
        SageGulley | SageOren | SageSeres | SageOsfala | SageImpa | SageIrene | SageRosso => Large,
        _ => Small,
    }
}
