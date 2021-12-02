use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

use albw::{course, Item};

use crate::{Pendant, Portrait, Quest, Settings};

#[derive(Clone, Debug)]
pub struct State<'settings> {
    settings: &'settings Settings,
    player: Player,
}

impl<'settings> State<'settings> {
    pub fn new(settings: &'settings Settings) -> Self {
        let sword = if settings.items.captains_sword.is_skipped() {
            1
        } else {
            0
        };
        let bracelet = if settings.items.first_bracelet.is_skipped() {
            1
        } else {
            0
        };
        Self {
            settings,
            player: Player {
                sword,
                bracelet,
                ..Default::default()
            },
        }
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    pub fn with_all_overworld_items(settings: &'settings Settings) -> Self {
        Self {
            settings,
            player: Player::with_all_overworld_items(),
        }
    }

    pub fn is_different(&self, other: &Self) -> bool {
        self.player != other.player
    }

    pub fn is_barrier_up(&self) -> bool {
        self.settings.behavior.barrier.is_start() || self.did_eastern()
    }

    fn can_use_items(&self) -> bool {
        self.settings.modifications.y_button_enabled || self.player.lamp
    }

    pub fn can_damage(&self) -> bool {
        self.sword()
            || self.can_ice_rod()
            || self.can_bomb()
            || self.can_fire_rod()
            || self.can_hammer()
            || self.can_bow()
    }

    pub fn can_ice_rod(&self) -> bool {
        self.can_use_items() && self.player.ice_rod
    }

    pub fn can_sand_rod(&self) -> bool {
        self.can_use_items() && self.player.sand_rod
    }

    pub fn can_tornado_rod(&self) -> bool {
        self.can_use_items() && self.player.tornado_rod
    }

    pub fn can_bomb(&self) -> bool {
        self.can_use_items() && self.player.bomb
    }

    pub fn can_fire_rod(&self) -> bool {
        self.can_use_items() && self.player.fire_rod
    }

    pub fn can_hookshot(&self) -> bool {
        self.can_use_items() && self.player.hookshot
    }

    pub fn can_boomerang(&self) -> bool {
        self.can_use_items() && self.player.boomerang
    }

    pub fn can_hammer(&self) -> bool {
        self.can_use_items() && self.player.hammer
    }

    pub fn can_bow(&self) -> bool {
        self.can_use_items() && self.player.bow
    }

    pub fn can_use_projectile(&self) -> bool {
        self.can_bomb() || self.can_hookshot() || self.can_boomerang() || self.can_bow()
    }

    pub fn has_ranged_attack(&self) -> bool {
        self.can_use_projectile() || self.has_master_sword()
    }

    pub fn can_lamp(&self) -> bool {
        self.player.lamp
    }

    pub fn can_see_in_dark(&self) -> bool {
        self.settings.logic.dont_require_lamp_for_darkness || self.player.lamp
    }

    pub fn has_bottle(&self) -> bool {
        self.player.bottle
    }

    pub fn has_smooth_gem(&self) -> bool {
        self.player.smooth_gem
    }

    pub fn can_light(&self) -> bool {
        self.can_lamp() || self.can_fire_rod()
    }

    pub fn sword(&self) -> bool {
        self.player.sword > 1
    }

    pub fn has_master_sword(&self) -> bool {
        self.player.sword > 2
    }

    pub fn can_swim(&self) -> bool {
        self.player.flippers
    }

    pub fn can_merge(&self) -> bool {
        self.player.bracelet > 1
    }

    pub fn can_lift(&self) -> bool {
        self.player.glove > 0
    }

    pub fn can_insect_net(&self) -> bool {
        self.can_use_items() && self.player.insect_net
    }

    pub fn can_lift_big(&self) -> bool {
        self.player.glove > 1
    }

    pub fn ore(&self) -> u8 {
        self.player.ore
    }

    pub fn has_boots(&self) -> bool {
        self.player.boots
    }

    pub fn has_message(&self) -> bool {
        self.player.message
    }

    pub fn has_premium_milk(&self) -> bool {
        self.player.premium_milk
    }

    pub fn has_stamina_scroll(&self) -> bool {
        self.player.scroll
    }

    pub fn did_sanctuary(&self) -> bool {
        self.settings.behavior.open || self.player.sanctuary
    }

    pub fn did_eastern(&self) -> bool {
        self.player.courage
    }

    pub fn has_three_pendants(&self) -> bool {
        self.player.courage && self.player.wisdom && self.player.power
    }

    pub fn small_keys(&self, dungeon: course::Id) -> u8 {
        self.player
            .small_keys
            .get(&dungeon)
            .copied()
            .unwrap_or_default()
    }

    pub fn has_boss_key(&self, dungeon: course::Id) -> bool {
        self.player.boss_keys.contains(&dungeon)
    }

    pub fn lorule(&self) -> bool {
        self.player.lorule
    }

    pub fn has_seven_portraits(&self) -> bool {
        self.player.portraits == 7
    }

    pub fn osfala(&self) -> bool {
        self.player.osfala
    }

    pub fn add_item(&mut self, item: Item) {
        self.player.add_item(item)
    }

    pub fn add_item_with_location(&mut self, item: Item, dungeon: course::Id) {
        match item {
            Item::KeySmall => self
                .player
                .small_keys
                .entry(dungeon)
                .or_default()
                .add_assign(1),
            Item::KeyBoss => {
                self.player.boss_keys.insert(dungeon);
            }
            _ => {
                self.add_item(item);
            }
        }
    }

    pub(crate) fn add_quest_item(&mut self, quest: Quest) {
        if quest == Quest::Portrait(Portrait::Osfala) {
            self.player.osfala = true;
        }
        match quest {
            Quest::Sanctuary => self.player.sanctuary = true,
            Quest::Pendant(Pendant::Courage) => self.player.courage = true,
            Quest::Pendant(Pendant::Wisdom) => self.player.wisdom = true,
            Quest::Pendant(Pendant::Power) => self.player.power = true,
            Quest::Lorule => self.player.lorule = true,
            Quest::Portrait(_) => self.player.portraits += 1,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Player {
    ice_rod: bool,
    sand_rod: bool,
    tornado_rod: bool,
    bomb: bool,
    fire_rod: bool,
    hookshot: bool,
    boomerang: bool,
    hammer: bool,
    bow: bool,
    bottle: bool,
    smooth_gem: bool,
    lamp: bool,
    sword: u8,
    flippers: bool,
    bracelet: u8,
    glove: u8,
    insect_net: bool,
    boots: bool,
    ore: u8,
    message: bool,
    premium_milk: bool,
    scroll: bool,
    sanctuary: bool,
    courage: bool,
    wisdom: bool,
    power: bool,
    lorule: bool,
    portraits: u8,
    osfala: bool,
    small_keys: HashMap<course::Id, u8>,
    boss_keys: HashSet<course::Id>,
}

impl Player {
    fn with_all_overworld_items() -> Self {
        Self {
            ice_rod: true,
            sand_rod: true,
            tornado_rod: true,
            bomb: true,
            fire_rod: true,
            hookshot: true,
            boomerang: true,
            hammer: true,
            bow: true,
            bottle: true,
            lamp: true,
            sword: 5,
            flippers: true,
            bracelet: 2,
            glove: 2,
            scroll: true,
            ..Default::default()
        }
    }

    fn add_item(&mut self, item: Item) {
        match item {
            Item::ItemRentalIceRod => self.ice_rod = true,
            Item::ItemRentalSandRod => self.sand_rod = true,
            Item::ItemRentalTornadeRod => self.tornado_rod = true,
            Item::ItemRentalBomb => self.bomb = true,
            Item::ItemRentalFireRod => self.fire_rod = true,
            Item::ItemRentalHookShot => self.hookshot = true,
            Item::ItemRentalBoomerang => self.boomerang = true,
            Item::ItemRentalHammer => self.hammer = true,
            Item::ItemRentalBow => self.bow = true,
            Item::ItemBottle => self.bottle = true,
            Item::ItemStoneBeauty => self.smooth_gem = true,
            Item::ItemKandelaar => self.lamp = true,
            Item::ItemSwordLv1 | Item::ItemSwordLv2 | Item::PackageSword => self.sword += 1,
            Item::ItemMizukaki => self.flippers = true,
            Item::RingRental | Item::RingHekiga => self.bracelet += 1,
            Item::PowerGlove | Item::PowerfulGlove => self.glove += 1,
            Item::ItemInsectNet => self.insect_net = true,
            Item::OreYellow | Item::OreGreen | Item::OreBlue | Item::OreRed => self.ore += 1,
            Item::DashBoots => self.boots = true,
            Item::MessageBottle => self.message = true,
            Item::MilkMatured => self.premium_milk = true,
            Item::GanbariPowerUp => self.scroll = true,
            _ => {}
        }
    }
}
