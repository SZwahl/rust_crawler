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
    pub i_shield: bool,
    pub i_weapons: Vec<Weapon>,
    pub i_armor: Vec<Armor>,
    pub i_spells: Vec<Spell>,
    pub i_potions: u32,
}

impl Character {
    pub fn new_character(adv: u32, dis: u32) -> Self {

        let mut stats: [u32; 3] = [0,0,0];

        for s in 0..stats.len() {
            if adv == 0 {
                stats[s] = roll("3d6").total;
            }
            if adv == s as u32 +1 {
                stats[s] = roll_adv("3d6").total;
            }
            else if dis == s as u32 +1 {
                stats[s] = roll_dis("3d6").total;
            }
            else {
                stats[s] = roll("3d6").total;
            }
        }
        
        let hit_p = 4 + roll("1d3").total;
        
        Self {
            condition: Condition::new(hit_p, stats[0], stats[1], stats[2]),

            e_weapon: Weapon::new("Unarmed", "unarmed", "punch", "1d4", 0, false, StatTypes::Power),
            e_armor: Armor::new("Unarmored", "unarmored", 0, false),
            e_shield: false,

            i_shield: false,
            i_weapons: Vec::new(),
            i_armor: Vec::new(),
            i_spells: Vec::new(),
            i_potions: 0,
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

    pub fn swap_weapon(&mut self, w: &Weapon){
        if w.bulky && self.e_shield
        {
            println!("This weapon is bulky! Shield unequipped.");
            self.e_shield = false;
            self.i_shield = true;
            self.condition.armor -= 1;
            println!("Shield equipped. Your armor is now {}.", self.condition.armor);
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
        self.i_shield = false;
        self.e_shield = true;
        self.condition.armor += 1;
        println!("Shield equipped. Your armor is now {}.", self.condition.armor);
    }

    pub fn print_inventory(&self)
    {
        println!("You have are weilding {} ({})", self.e_weapon.name, self.e_weapon.roll);
        println!("You have {} armor from wearing {} (+{}). ", self.condition.armor, self.e_armor.name, self.e_armor.bonus);
        
        if self.e_shield {
            println!("You have a shield contributing 1 armor");
        }
        else if self.i_shield {
            println!("You have an unequipped [Shield].");
        }
        else {
            println!("You have no shield available.");
        }

        println!("[Weapons]");
        for wep in &self.i_weapons
        {
            println!("*\t{} [{}]", wep.name, wep.key);
        }
        println!("[Armor]");
        for arm in &self.i_armor
        {
            println!("*\t{} [{}]", arm.name, arm.key);
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

    pub fn acquire_potion(&mut self) {
        self.i_potions += 1;
    }

    pub fn drink_potion(&mut self) {
        let mut heal = roll("1d4").total;
        heal += 2;
    
        let max_heal = self.condition.tot_hp - self.condition.c_hp;
        if heal > max_heal { heal = max_heal;}
    
        self.condition.c_hp += heal;
    
        println!("Drank potion for {} health. (Now at {}/{})", heal, self.condition.c_hp, self.condition.tot_hp);
    
        if max_heal == 0 {
            println!("You effectively wasted that didn't you?");
        }
    }
}

