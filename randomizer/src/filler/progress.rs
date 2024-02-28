use crate::filler::filler_item::Item::Quake;
use crate::filler::filler_item::Vane;
use crate::filler::filler_item::{Goal, Item, Randomizable};
use crate::filler::item_pools;
use crate::{DashSet, SeedInfo};
use modinfo::settings::keysy::Keysy;
use modinfo::settings::portals::Portals;
use modinfo::settings::ravios_shop::RaviosShop;
use modinfo::settings::trials_door::TrialsDoor;
use modinfo::settings::weather_vanes::WeatherVanes;
use modinfo::settings::{pedestal::PedestalSetting, Settings};

#[derive(Clone, Debug)]
pub struct Progress<'s> {
    items: DashSet<Randomizable>,
    seed_info: &'s SeedInfo,
}

impl<'s> Progress<'s> {
    pub fn new(seed_info: &'s SeedInfo) -> Progress<'s> {
        Self { items: Default::default(), seed_info }
    }

    pub fn get_items(&self) -> &DashSet<Randomizable> {
        &self.items
    }

    pub fn get_settings(&self) -> &Settings {
        &self.seed_info.settings
    }

    pub fn add_item(&mut self, item: impl Into<Randomizable>) {
        self.items.insert(item.into());
    }

    pub fn difference(&self, other: &Progress) -> DashSet<Randomizable> {
        let mut new_items: DashSet<Randomizable> = Default::default();

        for item in &self.items {
            if !other.items.contains(item) {
                new_items.insert(*item);
            }
        }

        new_items
    }

    pub fn has(&self, item: impl Into<Randomizable>) -> bool {
        self.items.contains(&item.into())
    }

    fn has_either(&self, item1: impl Into<Randomizable>, item2: impl Into<Randomizable>) -> bool {
        self.items.contains(&item1.into()) || self.items.contains(&item2.into())
    }

    fn has_both(&self, item1: impl Into<Randomizable>, item2: impl Into<Randomizable>) -> bool {
        self.items.contains(&item1.into()) && self.items.contains(&item2.into())
    }

    fn has_any<T>(&self, items: impl IntoIterator<Item = T>) -> bool
    where
        T: Into<Randomizable>,
    {
        for item in items {
            if self.has(item.into()) {
                return true;
            }
        }
        false
    }

    fn has_amount<T>(&self, amount: u8, items: impl IntoIterator<Item = T>) -> bool
    where
        T: Into<Randomizable>,
    {
        self.count(items) >= amount
    }

    fn count<T>(&self, items: impl IntoIterator<Item = T>) -> u8
    where
        T: Into<Randomizable>,
    {
        let mut sum: u8 = 0;
        for item in items {
            // fixme expensive clone
            if self.clone().has(item.into()) {
                sum += 1;
            }
        }
        sum
    }

    pub fn hearts(&self, amount: f32) -> bool {
        let heart_containers = self.count(item_pools::get_heart_containers()) as f32;
        let heart_pieces = self.count(item_pools::get_heart_pieces()) as f32;
        3.0 + heart_containers + (0.25 * heart_pieces) >= amount
    }

    pub fn has_rupees(&self, amount: u16) -> bool {
        let purples = self.count(item_pools::get_purple_rupee_pool());
        let silvers = self.count(item_pools::get_silver_rupee_pool());
        let golds = self.count(item_pools::get_gold_rupee_pool());

        amount <= (purples as u16 * 50) + (silvers as u16 * 100) + (golds as u16 * 300)
    }

    pub fn has_lamp(&self) -> bool {
        self.has_either(Item::Lamp01, Item::Lamp02)
    }

    #[allow(unused)]
    pub fn has_super_lamp(&self) -> bool {
        self.has_both(Item::Lamp01, Item::Lamp02)
    }

    pub fn has_bow(&self) -> bool {
        self.has_any([Item::Bow01, Item::Bow02, Item::Bow03])
    }

    pub fn has_nice_bow(&self) -> bool {
        self.has_amount(2, [Item::Bow01, Item::Bow02, Item::Bow03])
    }

    pub fn has_boomerang(&self) -> bool {
        self.has_either(Item::Boomerang01, Item::Boomerang02)
    }

    #[allow(unused)]
    pub fn has_nice_boomerang(&self) -> bool {
        self.has_both(Item::Boomerang01, Item::Boomerang02)
    }

    pub fn has_hookshot(&self) -> bool {
        self.has_either(Item::Hookshot01, Item::Hookshot02)
    }

    pub fn has_nice_hookshot(&self) -> bool {
        self.has_both(Item::Hookshot01, Item::Hookshot02)
    }

    pub fn has_hammer(&self) -> bool {
        self.has_either(Item::Hammer01, Item::Hammer02)
    }

    #[allow(unused)]
    pub fn has_nice_hammer(&self) -> bool {
        self.has_both(Item::Hammer01, Item::Hammer02)
    }

    pub fn has_bombs(&self) -> bool {
        self.has_either(Item::Bombs01, Item::Bombs02)
    }

    pub fn has_nice_bombs(&self) -> bool {
        self.has_both(Item::Bombs01, Item::Bombs02)
    }

    pub fn has_fire_rod(&self) -> bool {
        self.has_either(Item::FireRod01, Item::FireRod02)
    }

    #[allow(unused)]
    pub fn has_nice_fire_rod(&self) -> bool {
        self.has_both(Item::FireRod01, Item::FireRod02)
    }

    pub fn has_ice_rod(&self) -> bool {
        self.has_either(Item::IceRod01, Item::IceRod02)
    }

    pub fn has_nice_ice_rod(&self) -> bool {
        self.has_both(Item::IceRod01, Item::IceRod02)
    }

    pub fn has_tornado_rod(&self) -> bool {
        self.has_either(Item::TornadoRod01, Item::TornadoRod02)
    }

    pub fn has_nice_tornado_rod(&self) -> bool {
        self.has_both(Item::TornadoRod01, Item::TornadoRod02)
    }

    pub fn has_sand_rod(&self) -> bool {
        self.has_either(Item::SandRod01, Item::SandRod02)
    }

    #[allow(unused)]
    pub fn has_nice_sand_rod(&self) -> bool {
        self.has_both(Item::SandRod01, Item::SandRod02)
    }

    pub fn has_net(&self) -> bool {
        self.has_either(Item::Net01, Item::Net02)
    }

    pub fn can_use_shield(&self) -> bool {
        self.has_sword()
            && self.has_any([Item::Shield01, Item::Shield02, Item::Shield03, Item::Shield04, Item::HylianShield])
    }

    pub fn has_scoot_fruit(&self) -> bool {
        self.has_either(Item::ScootFruit01, Item::ScootFruit02)
    }

    #[allow(unused)]
    pub fn has_foul_fruit(&self) -> bool {
        self.has_either(Item::FoulFruit01, Item::FoulFruit02)
    }

    pub fn has_fire_source(&self) -> bool {
        self.has_any([Item::Lamp01, Item::Lamp02, Item::FireRod01, Item::FireRod02])
    }

    pub fn can_destroy_curtain(&self) -> bool {
        use Item::*;
        self.has_any([
            Sword01, Sword02, Sword03, Sword04, Lamp01, Lamp02, FireRod01, FireRod02, Bombs01, Bombs02, PegasusBoots,
        ])
    }

    pub fn can_extinguish_torches(&self) -> bool {
        use Item::*;
        self.has_any([
            Sword01, Sword02, Sword03, Sword04, Bombs01, Bombs02, IceRod01, IceRod02, TornadoRod01, TornadoRod02,
            Net01, Net02,
        ])
    }

    pub fn has_bell(&self) -> bool {
        self.has(Item::Bell)
    }

    pub fn are_hyrule_vanes_active(&self) -> bool {
        match self.seed_info.settings.weather_vanes {
            WeatherVanes::Hyrule | WeatherVanes::All => true,
            _ => false,
        }
    }

    pub fn are_lorule_vanes_active(&self) -> bool {
        match self.seed_info.settings.weather_vanes {
            WeatherVanes::Lorule | WeatherVanes::All => true,
            _ => false,
        }
    }

    pub fn can_escape(&self) -> bool {
        self.has_bell() || self.has_fire_rod() || self.has_bombs()
    }

    pub fn can_escape_dungeon(&self) -> bool {
        self.has_fire_rod() || self.has_bombs() || self.has_scoot_fruit()
    }

    pub fn has_stamina_scroll(&self) -> bool {
        self.has(Item::StaminaScroll)
    }

    pub fn has_bottle(&self) -> bool {
        self.has_any([Item::Bottle01, Item::Bottle02, Item::Bottle03, Item::Bottle04, Item::Bottle05])
    }

    pub fn has_boots(&self) -> bool {
        self.has(Item::PegasusBoots)
    }

    pub fn has_power_glove(&self) -> bool {
        self.has_either(Item::Glove01, Item::Glove02)
    }

    pub fn has_titans_mitt(&self) -> bool {
        self.has_both(Item::Glove01, Item::Glove02)
    }

    pub fn has_flippers(&self) -> bool {
        self.has(Item::Flippers)
    }

    pub fn are_portals_open(&self) -> bool {
        self.seed_info.settings.portals == Portals::Open || self.has(Quake)
    }

    pub fn can_merge(&self) -> bool {
        self.seed_info.settings.start_with_merge || self.has_both(Item::RaviosBracelet01, Item::RaviosBracelet02)
    }

    pub fn has_mail(&self) -> bool {
        self.has_either(Item::Mail01, Item::Mail02)
    }

    pub fn has_master_ore(&self, amount: u8) -> bool {
        self.has_amount(amount, [Item::OreRed, Item::OreGreen, Item::OreBlue, Item::OreYellow])
    }

    pub fn has_90_maiamai(&self) -> bool {
        self.has_maiamai(90)
    }

    pub fn has_100_maiamai(&self) -> bool {
        self.has_maiamai(100)
    }

    pub fn has_maiamai(&self, amount: u8) -> bool {
        self.has_amount(amount, item_pools::get_maiamai_pool())
    }

    pub fn has_smooth_gem(&self) -> bool {
        self.has(Item::SmoothGem)
    }

    pub fn has_letter_in_a_bottle(&self) -> bool {
        self.has(Item::LetterInABottle)
    }

    pub fn has_premium_milk(&self) -> bool {
        self.has(Item::PremiumMilk)
    }

    pub fn has_gold_bee(&self) -> bool {
        self.has(Item::GoldBee01) // Do not consider buying for 9999 in logic
    }

    pub fn has_sword(&self) -> bool {
        self.has_any([Item::Sword01, Item::Sword02, Item::Sword03, Item::Sword04])
    }

    pub fn has_master_sword(&self) -> bool {
        self.has_amount(2, [Item::Sword01, Item::Sword02, Item::Sword03, Item::Sword04])
    }

    pub fn swordless_mode(&self) -> bool {
        self.seed_info.settings.swordless_mode
    }

    pub fn nice_mode(&self) -> bool {
        self.seed_info.settings.nice_mode
    }

    pub fn progression_enemies(&self) -> bool {
        !self.seed_info.settings.no_progression_enemies
    }

    pub fn break_floor_tiles(&self) -> bool {
        self.has_bombs() || self.has_hammer()
    }

    pub fn not_nice_mode(&self) -> bool {
        !self.nice_mode()
    }

    pub fn lampless(&self) -> bool {
        self.seed_info.settings.dark_rooms_lampless
    }

    pub fn can_great_spin(&self) -> bool {
        self.has_sword() && self.has(Item::GreatSpin)
    }

    pub fn can_destroy_skull(&self) -> bool {
        use Item::*;
        self.has_any([
            Sword01, Sword02, Sword03, Sword04, Bow01, Bow02, Bow03, Boomerang01, Boomerang02, Hookshot01, Hookshot02,
            Bombs01, Bombs02, FireRod01, FireRod02, IceRod01, IceRod02, SandRod01, SandRod02, Hammer01, Hammer02,
            PegasusBoots, Glove01, Glove02,
        ])
    }

    pub fn can_cut_grass(&self) -> bool {
        use Item::*;
        self.has_any([
            Sword01, Sword02, Sword03, Sword04, Boomerang01, Boomerang02, Bombs01, Bombs02, FireRod01, FireRod02,
            IceRod01, IceRod02, Lamp01, Lamp02, PegasusBoots,
        ])
    }

    pub fn can_attack(&self) -> bool {
        self.has_sword()
            || self.has_bow()
            || self.has_bombs()
            || self.has_fire_rod()
            || self.has_ice_rod()
            || self.has_hammer()
            || self.has_boots()
            || self.has_nice_tornado_rod()
            || self.has_nice_hookshot()
    }

    pub fn can_attack_fireproof(&self) -> bool {
        self.has_sword()
            || self.has_bow()
            || self.has_bombs()
            || self.has_ice_rod()
            || self.has_hammer()
            || self.has_boots()
            || self.has_nice_tornado_rod()
            || self.has_nice_hookshot()
    }

    pub fn has_lamp_or_net(&self) -> bool {
        self.has_any([Item::Lamp01, Item::Lamp02, Item::Net01, Item::Net02])
    }

    pub fn can_hit_switch(&self) -> bool {
        self.has_sword()
            || self.has_bow()
            || self.has_boomerang()
            || self.has_hookshot()
            || self.has_bombs()
            || self.has_ice_rod()
            || self.has_hammer()
            || self.has_boots()
    }

    pub fn can_hit_switch_bootless(&self) -> bool {
        self.has_sword()
            || self.has_bow()
            || self.has_boomerang()
            || self.has_hookshot()
            || self.has_bombs()
            || self.has_ice_rod()
            || self.has_hammer()
    }

    pub fn can_hit_far_switch(&self) -> bool {
        use Item::*;
        self.has_any([Bow01, Bow02, Bow03, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02])
    }

    pub fn can_hit_shielded_switch(&self) -> bool {
        use Item::*;
        self.has_any([
            Sword01, Sword02, Sword03, Sword04, Bow01, Bow02, Bow03, Boomerang01, Boomerang02, Hookshot01, Hookshot02,
            Bombs01, Bombs02, Hammer01, Hammer02,
        ])
    }

    pub fn can_hit_hog_1f_switch(&self) -> bool {
        self.can_hit_far_switch()
            || self.has_ice_rod()
            || self.can_great_spin()
            || (self.can_merge() && (self.has_sword() || self.has_hammer()))
    }

    // BOSSES ----------------------------------------------------------------------------------------------------------

    pub fn can_defeat_margomill(&self) -> bool {
        self.has_tornado_rod()
            && (self.has_sword() || self.has_bow() || self.has_bombs() || self.has_fire_rod() || self.has_hammer())
    }

    pub fn can_defeat_moldorm(&self) -> bool {
        self.has_hammer()
    }

    pub fn can_defeat_yuga2(&self) -> bool {
        self.has_sword() || self.has_bombs() || self.has_fire_rod() || self.has_ice_rod() || self.has_hammer()
    }

    pub fn can_defeat_gemesaur(&self) -> bool {
        self.has_bombs() && (self.has_lamp() || (self.has_fire_rod() && self.lampless()))
    }

    pub fn can_defeat_arrgus(&self) -> bool {
        self.has_hookshot() && self.can_attack()
    }

    pub fn can_defeat_knucklemaster_swordless(&self) -> bool {
        // Bow does not work
        self.swordless_mode()
            && self.can_merge()
            && (self.has_bombs() || self.has_fire_rod() || self.has_hammer() || self.has_ice_rod())
    }

    pub fn can_technically_defeat_knucklemaster(&self) -> bool {
        self.can_merge()
            && (self.has_sword()
            // Bow does not work
            || self.has_bombs()
            || self.has_fire_rod()
            || self.has_ice_rod()
            || self.has_hammer()
            || self.has_lamp_or_net())
    }

    pub fn can_defeat_dharkstare(&self) -> bool {
        self.has_fire_rod()
    }

    pub fn can_defeat_grinexx(&self) -> bool {
        self.has_ice_rod()
    }

    // KEYS ------------------------------------------------------------------------------------------------------------

    fn is_small_keysy(&self) -> bool {
        match self.seed_info.settings.keysy {
            Keysy::SmallKeysy | Keysy::AllKeysy => true,
            _ => false,
        }
    }

    fn is_big_keysy(&self) -> bool {
        match self.seed_info.settings.keysy {
            Keysy::BigKeysy | Keysy::AllKeysy => true,
            _ => false,
        }
    }

    pub fn has_sanctuary_key(&self) -> bool {
        self.is_small_keysy() || self.has(Item::HyruleSanctuaryKey)
    }

    pub fn has_lorule_sanctuary_key(&self) -> bool {
        self.is_small_keysy() || self.has(Item::LoruleSanctuaryKey)
    }

    pub fn has_eastern_compass(&self) -> bool {
        self.has(Item::EasternCompass)
    }

    pub fn has_eastern_keys(&self, amount: u8) -> bool {
        self.is_small_keysy() || self.has_amount(amount, [Item::EasternKeySmall01, Item::EasternKeySmall02])
    }

    pub fn has_eastern_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::EasternKeyBig)
    }

    pub fn has_gales_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(
                amount,
                [Item::GalesKeySmall01, Item::GalesKeySmall02, Item::GalesKeySmall03, Item::GalesKeySmall04],
            )
    }

    pub fn has_gales_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::GalesKeyBig)
    }

    pub fn has_hera_keys(&self, amount: u8) -> bool {
        self.is_small_keysy() || self.has_amount(amount, [Item::HeraKeySmall01, Item::HeraKeySmall02])
    }

    pub fn has_hera_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::HeraKeyBig)
    }

    pub fn has_dark_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(
                amount,
                [Item::DarkKeySmall01, Item::DarkKeySmall02, Item::DarkKeySmall03, Item::DarkKeySmall04],
            )
    }

    pub fn has_dark_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::DarkKeyBig)
    }

    pub fn has_swamp_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(
                amount,
                [Item::SwampKeySmall01, Item::SwampKeySmall02, Item::SwampKeySmall03, Item::SwampKeySmall04],
            )
    }

    pub fn has_swamp_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::SwampKeyBig)
    }

    pub fn has_skull_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(amount, [Item::SkullKeySmall01, Item::SkullKeySmall02, Item::SkullKeySmall03])
    }

    pub fn has_skull_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::SkullKeyBig)
    }

    pub fn has_thieves_key(&self) -> bool {
        self.is_small_keysy() || self.has(Item::ThievesKeySmall)
    }

    pub fn has_thieves_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::ThievesKeyBig)
    }

    pub fn has_ice_compass(&self) -> bool {
        self.has(Item::IceCompass)
    }

    pub fn has_ice_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(amount, [Item::IceKeySmall01, Item::IceKeySmall02, Item::IceKeySmall03])
    }

    pub fn has_ice_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::IceKeyBig)
    }

    pub fn has_desert_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(
                amount,
                [
                    Item::DesertKeySmall01,
                    Item::DesertKeySmall02,
                    Item::DesertKeySmall03,
                    Item::DesertKeySmall04,
                    Item::DesertKeySmall05,
                ],
            )
    }

    pub fn has_desert_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::DesertKeyBig)
    }

    pub fn has_turtle_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(amount, [Item::TurtleKeySmall01, Item::TurtleKeySmall02, Item::TurtleKeySmall03])
    }

    pub fn has_turtle_big_key(&self) -> bool {
        self.is_big_keysy() || self.has(Item::TurtleKeyBig)
    }

    pub fn has_lorule_keys(&self, amount: u8) -> bool {
        self.is_small_keysy()
            || self.has_amount(
                amount,
                [
                    Item::LoruleCastleKeySmall01,
                    Item::LoruleCastleKeySmall02,
                    Item::LoruleCastleKeySmall03,
                    Item::LoruleCastleKeySmall04,
                    Item::LoruleCastleKeySmall05,
                ],
            )
    }

    pub fn has_completed_trials(&self) -> bool {
        self.seed_info.settings.trials_door == TrialsDoor::Open
            || ((self.has(Goal::LcBombTrial) || !self.seed_info.trials_config.bomb_trial)
                && (self.has(Goal::LcTileTrial) || !self.seed_info.trials_config.tile_trial)
                && (self.has(Goal::LcLampTrial) || !self.seed_info.trials_config.lamp_trial)
                && (self.has(Goal::LcHookTrial) || !self.seed_info.trials_config.hook_trial))
    }

    pub fn has_bow_of_light(&self) -> bool {
        if self.seed_info.settings.progressive_bow_of_light {
            self.has(Item::Bow01) && self.has(Item::Bow02) && self.has(Item::Bow03)
        } else {
            self.has(Item::BowOfLight)
        }
    }

    // Events ------------------------------------------------

    // pub fn has_defeated_yuga(&self) -> bool {
    //     self.has(Yuga)
    // }
    // pub fn has_defeated_margomill(&self) -> bool {
    //     self.has(Margomill)
    // }
    // pub fn has_defeated_moldorm(&self) -> bool {
    //     self.has(Moldorm)
    // }
    // pub fn has_defeated_gemesaur_king(&self) -> bool {
    //     self.has(GemesaurKing)
    // }
    // pub fn has_defeated_arrghus(&self) -> bool {
    //     self.has(Arrghus)
    // }
    // pub fn has_defeated_knucklemaster(&self) -> bool {
    //     self.has(Knucklemaster)
    // }
    // pub fn has_defeated_stalblind(&self) -> bool {
    //     self.has(Stalblind)
    // }
    // pub fn has_defeated_grinexx(&self) -> bool {
    //     self.has(Grinexx)
    // }
    // pub fn has_defeated_zaganaga(&self) -> bool {
    //     self.has(Zaganaga)
    // }
    // pub fn has_defeated_dharkstare(&self) -> bool {
    //     self.has(Dharkstare)
    // }
    // pub fn has_opened_stylish_womans_house(&self) -> bool {
    //     self.has(StylishWomansHouseOpen)
    // }

    pub fn has_skull_eye_right(&self) -> bool {
        self.has(Goal::SkullEyeRight)
    }

    pub fn has_skull_eyes(&self) -> bool {
        self.has_both(Goal::SkullEyeLeft, Goal::SkullEyeRight)
    }

    pub fn thieves_b1_door_open(&self) -> bool {
        self.has(Goal::ThievesB1DoorOpen)
    }

    pub fn thieves_b2_door_open(&self) -> bool {
        self.has(Goal::ThievesB2DoorOpen)
    }

    pub fn thieves_b3_water_drained(&self) -> bool {
        self.has(Goal::ThievesB3WaterDrained)
    }

    pub fn thieves_b1b2_doors_open(&self) -> bool {
        self.thieves_b1_door_open() && self.thieves_b2_door_open()
    }

    pub fn thieves_escape_equipment(&self) -> bool {
        self.thieves_b1b2_doors_open() && self.has_thieves_key() && self.can_merge() && self.thieves_b3_water_drained()
    }

    pub fn adv_thieves_statue_clip(&self) -> bool {
        self.can_merge() && (self.has_bow() || self.has_boomerang() || self.has_ice_rod() || self.has_bombs())
    }

    pub fn hell_thieves_statue_clip(&self) -> bool {
        self.has_bombs()
            || (self.has_master_sword()
                && (self.can_merge() || self.has_boomerang() || self.has_ice_rod() || self.can_great_spin()))
            || self.adv_thieves_statue_clip()
    }

    pub fn can_rescue_turtles(&self) -> bool {
        self.has(Goal::TurtleFlipped) && self.has(Goal::TurtleAttacked) && self.has(Goal::TurtleWall)
    }

    pub fn has_seen_ravio_signs(&self) -> bool {
        self.has(Goal::RavioSigns)
    }

    pub fn is_ravio_shop_open(&self) -> bool {
        self.has(Goal::RavioShopOpen) || self.seed_info.settings.ravios_shop == RaviosShop::Open
    }

    pub fn has_bomb_flower(&self) -> bool {
        self.has(Goal::BigBombFlower)
    }

    pub fn has_shady_guy_trigger(&self) -> bool {
        self.has(Goal::ShadyGuyTrigger)
    }

    pub fn has_pendant_of_courage(&self) -> bool {
        self.has(Item::PendantOfCourage)
    }

    pub fn has_required_pendants(&self) -> bool {
        self.has(Item::PendantOfWisdom)
            && self.has(Item::PendantOfPower)
            && match self.seed_info.settings.ped_requirement {
                PedestalSetting::Vanilla => true,
                PedestalSetting::Standard => self.has_pendant_of_courage(),
            }
    }

    pub fn has_sage_gulley(&self) -> bool {
        self.has(Item::SageGulley)
    }

    pub fn has_sage_oren(&self) -> bool {
        self.has(Item::SageOren)
    }

    pub fn has_sage_seres(&self) -> bool {
        self.has(Item::SageSeres)
    }

    pub fn has_sage_osfala(&self) -> bool {
        self.has(Item::SageOsfala)
    }

    pub fn has_sage_impa(&self) -> bool {
        self.has(Item::SageImpa)
    }

    pub fn has_sage_irene(&self) -> bool {
        self.has(Item::SageIrene)
    }

    pub fn has_sage_rosso(&self) -> bool {
        self.has(Item::SageRosso)
    }

    pub fn has_lc_requirement(&self) -> bool {
        use Item::*;
        self.has_amount(
            self.seed_info.settings.lc_requirement,
            [SageGulley, SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso],
        )
    }

    pub fn has_yuganon_requirement(&self) -> bool {
        use Item::*;
        self.has_amount(
            self.seed_info.settings.yuganon_requirement,
            [SageGulley, SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso],
        )
    }

    pub fn has_saved_thief_girl(&self) -> bool {
        self.has(Goal::Stalblind)
    }

    pub fn has_opened_stylish_womans_house(&self) -> bool {
        self.has(Goal::StylishWomansHouseOpen)
    }

    pub fn has_woman_roof_maiamai(&self) -> bool {
        self.has(Goal::WomanRoofMaiamai)
    }

    pub fn has_opened_sanctuary_doors(&self) -> bool {
        self.has(Goal::OpenSanctuaryDoors)
    }

    pub fn can_access_milk_bar(&self) -> bool {
        self.has(Goal::AccessMilkBar)
    }

    pub fn can_get_potion(&self) -> bool {
        self.has_bottle() && self.has_either(Goal::AccessPotionShop, Goal::AccessMilkBar)
    }

    pub fn can_access_hyrule_blacksmith(&self) -> bool {
        self.has(Goal::AccessHyruleBlacksmith)
    }

    pub fn can_access_lorule_castle_field(&self) -> bool {
        self.has(Goal::AccessLoruleCastleField)
    }

    pub fn has_weather_vane(&self, vane: Vane) -> bool {
        self.has(vane)
    }
}
