//use rand::Rng;

use crate::roll::*;
use crate::equipment::*;
use crate::creature::*;

pub struct Character {
    //Condition
    pub condition: Condition,

    //Equipped items
    pub e_weapon: Weapon,
    pub e_armor: Armor,
    pub e_shield: bool,

    //Inventory
    pub i_weapons: Vec<Weapon>,
    pub i_armor: Vec<Armor>,
    pub i_spells: Vec<Spell>,
}

impl Character {
    pub fn new_character() -> Self {

        let stat1 = roll("3d6").total;
        let stat2 = roll("3d6").total;
        let stat3 = roll("3d6").total;
        let hit_p = roll("2d6").total;
        
        Self {
            condition: Condition::new(hit_p, stat1, stat2, stat3),

            e_weapon: Weapon::new("Unarmed", "punch", "1d4", false, StatTypes::Power),
            e_armor: Armor::new("Unarmored", 0, false),
            e_shield: false,

            i_weapons: Vec::new(),
            i_armor: Vec::new(),
            i_spells: Vec::new(),
        }
    }

    pub fn get_wep_mod(&self) -> i8 {
        match self.e_weapon.stat {
            StatTypes::Power => self.condition.mod_power,
            StatTypes::Finesse => self.condition.mod_finesse,
            StatTypes::Mind => self.condition.mod_mind,
            StatTypes::Health => 0,
            StatTypes::None => 0,
        }
    }

    pub fn print_character(&self){
        let c: Condition = self.condition;
        println!("Your character has: \n\t{}/{} Hit Protection\n\t{}/{} power ({}) \n\t{}/{} finesse ({}) \n\t{}/{} mind ({}).", 
            c.c_hp, c.tot_hp,
            c.c_power, c.power, c.mod_power,
            c.c_finesse, c.finesse, c.mod_finesse,
            c.c_mind, c.mind, c.mod_mind);

        if self.e_shield
        {
            println!("You have a shield equipped (+1)");
        }
        else {
            println!("You have no shield equipped (+0).");
        }
    }

    pub fn swap_weapon(&mut self, w: Weapon){
        if w.bulky && self.e_shield
        {
            println!("You must unequip your shield to hold a bulky weapon.");
            return;
        }

        //cache old weapon
        let old: Weapon = self.e_weapon.clone();
        
        //Set new weapon
        self.e_weapon = w.clone();

        //Add old to inventory if not unarmed
        if old.name != "Unarmed"
        {
            self.i_weapons.push(old.clone());
        }

        println!("Swapped {} for {} ({}).", old.print(), w.print(), w.roll);
    }

    pub fn acquire_weapon(&mut self, w: Weapon){
        self.i_weapons.push(w.clone());
        println!("Stowed {} in inventory.", w.name);
    }

    pub fn learn_spell(&mut self, s: Spell){
        self.i_spells.push(s.clone());
        println!("Engraved {}({}) to spell memory.", s.name, s.key);
    }

    pub fn equip_shield(&mut self)
    {
        //already have shield
        if self.e_shield
        {
            println!("Shield already equipped!");
        }

        //set true and armor up
        self.e_shield = true;
        self.condition.armor += 1;
        println!("Shield equipped. Your armor is now {}.", self.condition.armor);
    }

    pub fn print_inventory(&self)
    {
        println!("You have are weilding {} ({})", self.e_weapon.name, self.e_weapon.roll);
        println!("You have {} armor from wearing {} (+{}). ", self.condition.armor, self.e_armor.name, self.e_armor.bonus);
        println!("[Weapons]");
        for wep in &self.i_weapons
        {
            println!("*\t{}", wep.name);
        }
        println!("[Armor]");
        for arm in &self.i_armor
        {
            println!("*\t{}", arm.name);
        }
        println!("[Spells]");
        for spel in &self.i_spells
        {
            println!("*\t{}({})", spel.name, spel.key);
        }
    }
    pub fn print_spells(&self) {
        for spel in &self.i_spells
        {
            println!("*\t{}({})", spel.name, spel.key);
        }
    }
}

