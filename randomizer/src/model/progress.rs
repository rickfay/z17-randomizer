use {
    crate::{
        item_pools::{
            get_gold_rupee_pool, get_maiamai_pool, get_purple_rupee_pool, get_silver_rupee_pool,
        },
        model::filler_item::{FillerItem, FillerItem::*},
        settings::pedestal_setting::PedestalSetting,
        Settings,
    },
    std::collections::HashSet,
};

#[derive(Clone)]
pub struct Progress {
    items: HashSet<FillerItem>,
    settings: Settings,
}

impl Progress {
    pub fn new(settings: Settings) -> Self {
        Self { items: HashSet::new(), settings }
    }

    pub fn get_settings(&self) -> &Settings {
        &self.settings
    }

    pub fn add_item(&mut self, item: FillerItem) {
        self.items.insert(item);
    }

    pub fn difference(&self, other: &Progress) -> HashSet<FillerItem> {
        let mut new_items: HashSet<FillerItem> = HashSet::new();

        for item in &self.items {
            if !other.items.contains(&item) {
                new_items.insert(*item);
            }
        }

        new_items
    }

    pub fn has(&self, item: FillerItem) -> bool {
        self.items.contains(&item)
    }

    fn has_either(&self, item1: FillerItem, item2: FillerItem) -> bool {
        self.items.contains(&item1) || self.items.contains(&item2)
    }

    fn has_both(&self, item1: FillerItem, item2: FillerItem) -> bool {
        self.items.contains(&item1) && self.items.contains(&item2)
    }

    fn has_any(&self, items: &[FillerItem]) -> bool {
        for item in items {
            if self.has(*item) {
                return true;
            }
        }

        false
    }

    fn has_amount(&self, amount: u8, items: &[FillerItem]) -> bool {
        self.count(items) >= amount
    }

    fn count(&self, items: &[FillerItem]) -> u8 {
        let mut sum: u8 = 0;
        for item in items {
            // fixme expensive clone
            if self.clone().has(*item) {
                sum += 1;
            }
        }

        sum
    }

    pub fn has_rupees(&self, amount: u16) -> bool {
        let purples = self.count(get_purple_rupee_pool().as_slice());
        let silvers = self.count(get_silver_rupee_pool().as_slice());
        let golds = self.count(get_gold_rupee_pool().as_slice());

        amount <= (purples as u16 * 50) + (silvers as u16 * 100) + (golds as u16 * 300)
    }

    pub fn has_lamp(&self) -> bool {
        self.has_either(Lamp01, Lamp02)
    }

    #[allow(unused)]
    pub fn has_super_lamp(&self) -> bool {
        self.has_both(Lamp01, Lamp02)
    }

    pub fn has_bow(&self) -> bool {
        self.has_either(Bow01, Bow02)
    }

    #[allow(unused)]
    pub fn has_nice_bow(&self) -> bool {
        self.has_both(Bow01, Bow02)
    }

    pub fn has_boomerang(&self) -> bool {
        self.has_either(Boomerang01, Boomerang02)
    }

    #[allow(unused)]
    pub fn has_nice_boomerang(&self) -> bool {
        self.has_both(Boomerang01, Boomerang02)
    }

    pub fn has_hookshot(&self) -> bool {
        self.has_either(Hookshot01, Hookshot02)
    }

    pub fn has_nice_hookshot(&self) -> bool {
        self.has_both(Hookshot01, Hookshot02)
    }

    pub fn has_hammer(&self) -> bool {
        self.has_either(Hammer01, Hammer02)
    }

    #[allow(unused)]
    pub fn has_nice_hammer(&self) -> bool {
        self.has_both(Hammer01, Hammer02)
    }

    pub fn has_bombs(&self) -> bool {
        self.has_either(Bombs01, Bombs02)
    }

    pub fn has_nice_bombs(&self) -> bool {
        if self.settings.logic.nice_mode {
            self.has_both(Bombs01, Bombs02)
        } else {
            self.has_either(Bombs01, Bombs02) && self.has_maiamai(10)
        }
    }

    pub fn has_fire_rod(&self) -> bool {
        self.has_either(FireRod01, FireRod02)
    }

    #[allow(unused)]
    pub fn has_nice_fire_rod(&self) -> bool {
        self.has_both(FireRod01, FireRod02)
    }

    pub fn has_ice_rod(&self) -> bool {
        self.has_either(IceRod01, IceRod02)
    }

    pub fn has_nice_ice_rod(&self) -> bool {
        self.has_both(IceRod01, IceRod02)
    }

    pub fn has_tornado_rod(&self) -> bool {
        self.has_either(TornadoRod01, TornadoRod02)
    }

    pub fn has_nice_tornado_rod(&self) -> bool {
        self.has_both(TornadoRod01, TornadoRod02)
    }

    pub fn has_sand_rod(&self) -> bool {
        self.has_either(SandRod01, SandRod02)
    }

    #[allow(unused)]
    pub fn has_nice_sand_rod(&self) -> bool {
        self.has_both(SandRod01, SandRod02)
    }

    pub fn has_net(&self) -> bool {
        self.has_either(Net01, Net02)
    }

    pub fn can_use_shield(&self) -> bool {
        self.has_sword() && self.has_any(&[Shield01, Shield02, Shield03, Shield04, HylianShield])
    }

    pub fn has_scoot_fruit(&self) -> bool {
        self.has_either(ScootFruit01, ScootFruit02)
    }

    #[allow(unused)]
    pub fn has_foul_fruit(&self) -> bool {
        self.has_either(FoulFruit01, FoulFruit02)
    }

    pub fn has_fire_source(&self) -> bool {
        self.has_any(&[Lamp01, Lamp02, FireRod01, FireRod02])
    }

    pub fn can_destroy_curtain(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04, Lamp01, Lamp02, FireRod01, FireRod02, Bombs01,
            Bombs02, PegasusBoots,
        ])
    }

    pub fn can_extinguish_torches(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04, Bombs01, Bombs02, IceRod01, IceRod02, TornadoRod01,
            TornadoRod02, Net01, Net02,
        ])
    }

    pub fn has_bell(&self) -> bool {
        self.has(Bell)
    }

    pub fn are_vanes_activated(&self) -> bool {
        self.settings.logic.weather_vanes_activated
    }

    pub fn can_escape(&self) -> bool {
        self.has_bell() || self.has_fire_rod() || self.has_bombs()
    }

    pub fn can_escape_dungeon(&self) -> bool {
        self.has_fire_rod() || self.has_bombs() || self.has_scoot_fruit()
    }

    pub fn has_stamina_scroll(&self) -> bool {
        self.has(StaminaScroll)
    }

    pub fn has_bottle(&self) -> bool {
        self.has_any(&[Bottle01, Bottle02, Bottle03, Bottle04, Bottle05])
    }

    pub fn has_boots(&self) -> bool {
        self.has(PegasusBoots)
    }

    pub fn has_power_glove(&self) -> bool {
        self.has_either(Glove01, Glove02)
    }

    pub fn has_titans_mitt(&self) -> bool {
        self.has_both(Glove01, Glove02)
    }

    pub fn has_flippers(&self) -> bool {
        self.has(Flippers)
    }

    pub fn can_merge(&self) -> bool {
        self.settings.logic.start_with_merge || self.has_both(RaviosBracelet01, RaviosBracelet02)
    }

    pub fn has_mail(&self) -> bool {
        self.has_either(Mail01, Mail02)
    }

    pub fn has_master_ore(&self, amount: u8) -> bool {
        self.has_amount(amount, &[OreRed, OreGreen, OreBlue, OreYellow])
    }

    pub fn has_maiamai(&self, amount: u8) -> bool {
        self.has_amount(amount, get_maiamai_pool().as_slice())
    }

    pub fn has_smooth_gem(&self) -> bool {
        self.has(SmoothGem)
    }

    pub fn has_letter_in_a_bottle(&self) -> bool {
        self.has(LetterInABottle)
    }

    pub fn has_premium_milk(&self) -> bool {
        self.has(PremiumMilk)
    }

    pub fn has_gold_bee(&self) -> bool {
        self.has(GoldBee01) // Do not consider buying for 9999 in logic
    }

    pub fn has_sword(&self) -> bool {
        self.has_any(&[Sword01, Sword02, Sword03, Sword04])
    }

    pub fn has_master_sword(&self) -> bool {
        self.has_amount(2, &[Sword01, Sword02, Sword03, Sword04])
    }

    pub fn swordless_mode(&self) -> bool {
        self.settings.logic.swordless_mode
    }

    pub fn nice_mode(&self) -> bool {
        self.settings.logic.nice_mode
    }

    pub fn progression_enemies(&self) -> bool {
        !self.settings.logic.no_progression_enemies
    }

    pub fn break_floor_tiles(&self) -> bool {
        self.has_bombs() || self.has_hammer()
    }

    pub fn not_nice_mode(&self) -> bool {
        !self.nice_mode()
    }

    pub fn lampless(&self) -> bool {
        self.settings.logic.dark_rooms_lampless
    }

    pub fn can_great_spin(&self) -> bool {
        self.has_sword() && self.has(GreatSpin)
    }

    pub fn can_destroy_skull(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04, Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01,
            Hookshot02, Bombs01, Bombs02, FireRod01, FireRod02, IceRod01, IceRod02, SandRod01,
            SandRod02, Hammer01, Hammer02, PegasusBoots, Glove01, Glove02,
        ])
    }

    pub fn can_cut_grass(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04, Boomerang01, Boomerang02, Bombs01, Bombs02,
            FireRod01, FireRod02, IceRod01, IceRod02, Lamp01, Lamp02, PegasusBoots,
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
        self.has_any(&[Lamp01, Lamp02, Net01, Net02])
    }

    pub fn can_hit_switch(&self) -> bool {
        self.has_sword()
            || self.has_bow()
            || self.has_boomerang()
            || self.has_hookshot()
            || self.has_bombs()
            || self.has_ice_rod()
            || self.has_hammer()
    }

    pub fn can_hit_far_switch(&self) -> bool {
        self.has_any(&[
            Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01, Hookshot02, Bombs01, Bombs02,
        ])
    }

    pub fn can_hit_shielded_switch(&self) -> bool {
        self.has_any(&[
            Sword01, Sword02, Sword03, Sword04, Bow01, Bow02, Boomerang01, Boomerang02, Hookshot01,
            Hookshot02, Bombs01, Bombs02, Hammer01, Hammer02,
        ])
    }

    pub fn can_hit_hog_1f_switch(&self) -> bool {
        self.can_hit_far_switch()
            || self.has_ice_rod()
            || self.can_great_spin()
            || (self.can_merge() && (self.has_sword() || self.has_hammer()))
    }

    pub fn has_sanctuary_key(&self) -> bool {
        self.has(HyruleSanctuaryKey)
    }

    pub fn has_lorule_sanctuary_key(&self) -> bool {
        self.has(LoruleSanctuaryKey)
    }

    pub fn has_eastern_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[EasternKeySmall01, EasternKeySmall02])
    }

    pub fn has_eastern_big_key(&self) -> bool {
        self.has(EasternKeyBig)
    }

    pub fn has_gales_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[
            GalesKeySmall01, GalesKeySmall02, GalesKeySmall03, GalesKeySmall04,
        ])
    }

    pub fn has_gales_big_key(&self) -> bool {
        self.has(GalesKeyBig)
    }

    pub fn can_defeat_margomill(&self) -> bool {
        self.has_tornado_rod()
            && (self.has_sword()
                || self.has_bow()
                || self.has_bombs()
                || self.has_fire_rod()
                || self.has_hammer())
    }

    pub fn has_hera_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[HeraKeySmall01, HeraKeySmall02])
    }

    pub fn has_hera_big_key(&self) -> bool {
        self.has(HeraKeyBig)
    }

    pub fn can_defeat_moldorm(&self) -> bool {
        self.has_hammer()
    }

    pub fn can_defeat_yuga2(&self) -> bool {
        self.has_sword()
            || self.has_bombs()
            || self.has_fire_rod()
            || self.has_ice_rod()
            || self.has_hammer()
    }

    pub fn has_dark_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[DarkKeySmall01, DarkKeySmall02, DarkKeySmall03, DarkKeySmall04])
    }

    pub fn has_dark_big_key(&self) -> bool {
        self.has(DarkKeyBig)
    }

    pub fn can_defeat_gemesaur(&self) -> bool {
        self.has_bombs() && (self.has_lamp() || (self.has_fire_rod() && self.lampless()))
    }

    pub fn has_swamp_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[
            SwampKeySmall01, SwampKeySmall02, SwampKeySmall03, SwampKeySmall04,
        ])
    }

    pub fn has_swamp_big_key(&self) -> bool {
        self.has(SwampKeyBig)
    }

    pub fn can_defeat_arrgus(&self) -> bool {
        self.has_hookshot() && self.can_attack()
    }

    pub fn has_skull_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[SkullKeySmall01, SkullKeySmall02, SkullKeySmall03])
    }

    pub fn has_skull_big_key(&self) -> bool {
        self.has(SkullKeyBig)
    }

    pub fn can_defeat_knucklemaster(&self) -> bool {
        self.can_merge()
            && (self.has_sword()
            // Bow does not work
            || self.has_bombs()
            || self.has_fire_rod()
            || self.has_ice_rod()
            || self.has_hammer())
    }

    pub fn has_thieves_key(&self) -> bool {
        self.has(ThievesKeySmall)
    }

    pub fn has_thieves_big_key(&self) -> bool {
        self.has(ThievesKeyBig)
    }

    pub fn has_ice_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[IceKeySmall01, IceKeySmall02, IceKeySmall03])
    }

    pub fn has_ice_big_key(&self) -> bool {
        self.has(IceKeyBig)
    }

    pub fn can_defeat_dharkstare(&self) -> bool {
        self.has_fire_rod()
    }

    pub fn has_desert_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[
            DesertKeySmall01, DesertKeySmall02, DesertKeySmall03, DesertKeySmall04,
            DesertKeySmall05,
        ])
    }

    pub fn has_desert_big_key(&self) -> bool {
        self.has(DesertKeyBig)
    }

    pub fn can_defeat_zaganaga(&self) -> bool {
        self.has_sand_rod() && self.can_attack()
    }

    pub fn has_turtle_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[TurtleKeySmall01, TurtleKeySmall02, TurtleKeySmall03])
    }

    pub fn has_turtle_big_key(&self) -> bool {
        self.has(TurtleKeyBig)
    }

    pub fn can_defeat_grinexx(&self) -> bool {
        self.has_ice_rod()
    }

    pub fn has_lorule_keys(&self, amount: u8) -> bool {
        self.has_amount(amount, &[
            LoruleCastleKeySmall01, LoruleCastleKeySmall02, LoruleCastleKeySmall03,
            LoruleCastleKeySmall04, LoruleCastleKeySmall05,
        ])
    }

    pub fn has_completed_trials(&self) -> bool {
        self.settings.logic.skip_trials
            || (self.has(LcBombTrial)
                && self.has(LcBallTrial)
                && self.has(LcLampTrial)
                && self.has(LcHookTrial))
    }

    pub fn has_bow_of_light(&self) -> bool {
        self.has(BowOfLight)
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
        self.has(SkullEyeRight)
    }

    pub fn has_skull_eyes(&self) -> bool {
        self.has_both(SkullEyeLeft, SkullEyeRight)
    }

    pub fn thieves_b1_door_open(&self) -> bool {
        self.has(ThievesB1DoorOpen)
    }

    pub fn thieves_b2_door_open(&self) -> bool {
        self.has(ThievesB2DoorOpen)
    }

    pub fn thieves_b3_water_drained(&self) -> bool {
        self.has(ThievesB3WaterDrained)
    }

    pub fn thieves_b1b2_doors_open(&self) -> bool {
        self.thieves_b1_door_open() && self.thieves_b2_door_open()
    }

    pub fn thieves_escape_equipment(&self) -> bool {
        self.thieves_b1b2_doors_open()
            && self.has_thieves_key()
            && self.can_merge()
            && self.thieves_b3_water_drained()
    }

    pub fn adv_thieves_statue_clip(&self) -> bool {
        self.can_merge()
            && (self.has_bow() || self.has_boomerang() || self.has_ice_rod() || self.has_bombs())
    }

    pub fn hell_thieves_statue_clip(&self) -> bool {
        self.has_bombs()
            || (self.has_master_sword()
                && (self.can_merge()
                    || self.has_boomerang()
                    || self.has_ice_rod()
                    || self.can_great_spin()))
            || self.adv_thieves_statue_clip()
    }

    pub fn can_rescue_turtles(&self) -> bool {
        self.has(TurtleFlipped) && self.has(TurtleAttacked) && self.has(TurtleWall)
    }

    pub fn has_bomb_flower(&self) -> bool {
        self.has(BigBombFlower)
    }

    pub fn has_shady_guy_trigger(&self) -> bool {
        self.has(ShadyGuyTrigger)
    }

    fn has_charm(&self) -> bool {
        self.has_either(PendantOfCourage01, PendantOfCourage02)
    }

    pub fn has_pendant_of_courage(&self) -> bool {
        self.has_both(PendantOfCourage01, PendantOfCourage02)
    }

    pub fn has_required_pendants(&self) -> bool {
        self.has(PendantOfWisdom)
            && self.has(PendantOfPower)
            && match self.settings.logic.ped_requirement {
                PedestalSetting::Vanilla => true,
                PedestalSetting::Charmed => self.has_charm(),
                PedestalSetting::Standard => self.has_pendant_of_courage(),
            }
    }

    /// Reverse Sage Events
    pub fn is_rse(&self) -> bool {
        self.settings.logic.reverse_sage_events
    }

    pub fn has_sage_gulley(&self) -> bool {
        self.has(SageGulley)
    }

    pub fn has_sage_oren(&self) -> bool {
        self.has(SageOren)
    }

    pub fn has_sage_seres(&self) -> bool {
        self.has(SageSeres)
    }

    pub fn has_sage_osfala(&self) -> bool {
        self.has(SageOsfala)
    }

    pub fn has_sage_impa(&self) -> bool {
        self.has(SageImpa)
    }

    pub fn has_sage_irene(&self) -> bool {
        self.has(SageIrene)
    }

    pub fn has_sage_rosso(&self) -> bool {
        self.has(SageRosso)
    }

    pub fn has_lc_requirement(&self) -> bool {
        self.has_amount(self.settings.logic.lc_requirement, &[
            SageGulley, SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso,
        ])
    }

    pub fn has_yuganon_requirement(&self) -> bool {
        self.has_amount(self.settings.logic.yuganon_requirement, &[
            SageGulley, SageOren, SageSeres, SageOsfala, SageImpa, SageIrene, SageRosso,
        ])
    }

    pub fn has_opened_stylish_womans_house(&self) -> bool {
        self.has(StylishWomansHouseOpen)
    }

    pub fn has_woman_roof_maiamai(&self) -> bool {
        self.has(WomanRoofMaiamai)
    }

    pub fn has_opened_sanctuary_doors(&self) -> bool {
        self.has(OpenSanctuaryDoors)
    }

    pub fn can_get_potion(&self) -> bool {
        self.has_bottle() && self.has_either(AccessPotionShop, AccessMilkBar)
    }

    pub fn can_access_hyrule_blacksmith(&self) -> bool {
        self.has(AccessHyruleBlacksmith)
    }

    pub fn can_access_lorule_castle_field(&self) -> bool {
        self.has(AccessLoruleCastleField)
    }
}
