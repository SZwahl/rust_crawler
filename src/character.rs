//use rand::Rng;

use crate::roll::*;
use crate::equipment::*;

pub struct Character {
    //Stats
    pub power: u32,
    pub finesse: u32,
    pub mind: u32,

    pub mod_power: i8,
    pub mod_finesse: i8,
    pub mod_mind: i8,

    //Condition
    pub hp: u32,
    pub armor: u32,

    //Current stats
    pub c_power: u32,
    pub c_finesse: u32,
    pub c_mind: u32,
    pub c_hp: u32,

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
        let hit_p = roll("2d4").total;
        
        Self {
            power: stat1,
            finesse: stat2,
            mind: stat3,

            mod_power: calc_mod(stat1),
            mod_finesse: calc_mod(stat2),
            mod_mind: calc_mod(stat3),

            hp: hit_p,
            armor: 0,
    
            c_power: stat1,
            c_finesse: stat2,
            c_mind: stat3,
            c_hp: hit_p,

            e_weapon: Weapon::new("Unarmed", "1d4", false),
            e_armor: Armor::new("Unarmored", 0, false),
            e_shield: false,

            i_weapons: Vec::new(),
            i_armor: Vec::new(),
            i_spells: Vec::new(),
        }
    }

    pub fn print_character(&self){
        println!("Your character has: \n\t{}/{} power ({}) \n\t{}/{} finesse ({}) \n\t{}/{} mind ({}).", 
            self.c_power, self.power, self.mod_power,
            self.c_finesse, self.finesse, self.mod_finesse,
            self.c_mind, self.mind, self.mod_mind);
        println!("Your Hit Protection is {}/{}", self.c_hp, self.hp);

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
        println!("Engraved {} to spell memory.", s.name);
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
        self.armor += 1;
        println!("Shield equipped. Your armor is now {}.", self.armor);
    }

    pub fn print_inventory(&self)
    {
        println!("You have are weilding {} ({})", self.e_weapon.name, self.e_weapon.roll);
        println!("You have {} armor from wearing {} (+{}). ", self.armor, self.e_armor.name, self.e_armor.bonus);
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
            println!("*\t{}", spel.name);
        }
    }
}

fn calc_mod(stat: u32) -> i8 {
    if stat == 18 { return 3; }
    else if stat >= 15 { return 2; }
    else if stat >= 12 { return 1; }
    else if stat >= 9 { return 0; }
    else if stat >= 6 { return -1; }
    else if stat >= 4 { return -2; }
    else { return -3; }
}

