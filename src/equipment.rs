// enum WeaponTypes {
//     Crude,
//     Hand,
//     Field,
//     Noble,
//     Heavy,
// }

use crate::{creature::StatTypes, character::Character, roll::{Roll, roll}};

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub key: String,
    pub verb: String,
    pub roll: String,
    pub modifier: i32,
    pub bulky: bool,
    pub stat: StatTypes,
}

impl Weapon {
    pub fn new(n: &str, k: &str, v: &str, r: &str, m: i32, b: bool, t: StatTypes) -> Self {
        Self {
            name: String::from(n),
            key: String::from(k),
            verb: String::from(v),
            roll: String::from(r),
            modifier: m,
            bulky: b,
            stat: t,
        }
    }

    pub fn print(&self) -> String {
        let mut printed: String = self.name.clone();


        if self.modifier < 0 {
            printed.push_str("(-");
            printed.push_str(&self.modifier.to_string());
            printed.push_str(")");
        }
        else if self.modifier > 0 {
            printed.push_str("(+");
            printed.push_str(&self.modifier.to_string());
            printed.push_str(")");
        }

        if self.bulky
        {
            printed.push_str(" (B)");
        }

        return printed;
    }

    pub fn loot() -> Weapon {
        let roll = roll("1d10").total;
        
        let rapier = Weapon::new("Shiny Rapier", "srap", "slash", "1d6", 1, false, StatTypes::Finesse);
        let hefaxe = Weapon::new("Hefty Warhammer", "hefham", "swing hard", "1d8", 1, true, StatTypes::Power);
        let bbow = Weapon::new("Blackwood Bow", "bwbow", "shoot", "1d8", 1, true, StatTypes::Finesse);
        let clearorb = Weapon::new("Clear Orb (spell focus)", "corb", "attack using", "1d1", 1, true, StatTypes::Mind);
        let haxe = Weapon::new("Moonsteel Hand-axe", "msha", "hack", "1d8", 1, false, StatTypes::Power);

        let ssword = Weapon::new("Runed Longsword", "rls", "slash valiantly with", "1d8", 2, false, StatTypes::Power);
        let fbow = Weapon::new("Magma Bow", "magb", "shoot a fiery shot from", "1d10", 2, true, StatTypes::Finesse);
        let nsorb = Weapon::new("Night-Sky Orb", "nso", "attack using", "1d1", 4, true, StatTypes::Mind);
        let ugs = Weapon::new("Dragon-tail Greatsword", "dtg", "heave", "1d10", 2, true, StatTypes::Power);
        let bfkatana = Weapon::new("Billion-fold Katana", "bfk", "flash", "1d10", 3, true, StatTypes::Finesse);

        match roll {
            1 => return rapier,
            2 => return hefaxe,
            3 => return bbow,
            4 => return clearorb,
            5 => return haxe,
            6 => return ssword,
            7 => return fbow,
            8 => return nsorb,
            9 => return ugs,
            10 => return bfkatana,
            _ => return rapier, 
        }
    }
}

#[derive(Clone)]
pub struct Armor {
    pub name: String,
    pub key: String,
    pub bonus: u32,
    pub bulky: bool,
}

impl Armor {
    pub fn new(n: &str, k: &str, bonus: u32, bulky: bool) -> Self {
        Self {
            name: String::from(n),
            key: String::from(k),
            bonus: bonus,
            bulky: bulky,
        }
    }

    pub fn loot() -> Armor {
        let r = roll("1d6").total;

        let a = Armor::new("Protective robes", "pr", 1, false);
        let b = Armor::new("Runed Chains", "rc", 2, false);
        let c = Armor::new("Floating Guardians", "fg", 3, false);

        match r {
            1 | 2 => return a,
            3 | 4 | 5 => return b,
            6 => return c,
            _ => return a,
        }
    }
}

#[derive(Clone)]
pub struct Spell {
    pub name: String,
    pub key: String,
    pub cast_desc: String,
    pub cost: u32,
    pub damage: String,
    pub enemy_save: StatTypes,
    pub multi: bool,
}

impl Spell {
    pub fn new(n: &str, k: &str, desc: &str, cost: u32, dam: &str, save: StatTypes, mult: bool) -> Self {
        Self {
            name: String::from(n),
            key: String::from(k),
            cast_desc: String::from(desc),
            cost: cost,
            damage: String::from(dam),
            enemy_save: save,
            multi: mult,
        }
    }

    // pub fn loot() -> Spell {

    // }
}

pub fn random_spell(num: u32) -> Spell 
{
    let lightning_bolt = 
        Spell::new(
            "Lightning Bolt", 
            "lb",
            "Static surrounds your orb and an electric bolt leaps from your fingers.",
        1,
        "1d8",
            StatTypes::None,
            false
        );

    let fireball = 
            Spell::new(
                "Fireball",
                "f",
                "Your orb turns white and emits a thin beam, which explodes in a blinding flash.",
                2,
                "1d10",
                StatTypes::Finesse,
                true
            );

    let moonbeams = 
            Spell::new(
                "Moonbeams",
                "mb",
                "Your orb glows and refracts pale blue beams into the enemy.",
                1,
                "1d4",
                StatTypes::None,
                true
            );

    // let summon_imp = 
    //         Spell::new(
    //             "Summon Imp",
    //         )

        match num {
            1 | 2 => return lightning_bolt,
            3 => return fireball,
            4 => return moonbeams,
            _ => return lightning_bolt, 
        }
}

#[derive(Clone)]
pub enum ItemType {
    Weapon,
    Armor,
    Spell,
    Potion,
    Gold,
}

#[derive(Clone)]
pub struct Chest {
    pub name: String,
    pub key: String,
    pub context: String,
    pub items: Vec<ItemType>,
}

impl Chest {
    pub fn new(n: &str, k: &str, c: &str) -> Self {
        Self {
            name: String::from(n),
            key: String::from(k),
            context: String::from(c),
            items: Vec::new(),
        }
    }

    pub fn add(&mut self, it: ItemType) {
        self.items.push(it);
    }

    pub fn open(&self, c: &mut Character) {
        //loop
        for item in &self.items {
            //get each
            match item {
                ItemType::Weapon => {
                    let wep = Weapon::loot();
                    c.acquire_weapon(wep);
                },
                ItemType::Armor => {
                    let arm = Armor::loot();
                    println!("Found {}", arm.name);
                    c.i_armor.push(arm);
                },
                ItemType::Spell => {
                    let mut already_have = false;
                    let spell = random_spell(roll("1d4").total);
                    for s in &c.i_spells {
                        if s.name == spell.name {
                            already_have = true;
                            break;
                        }
                    }
                    if already_have {
                        println!("Already have {}!", spell.name);
                    }
                    else {
                        println!("Found {}.", spell.name);
                        c.learn_spell(spell);
                    }
                    
                }
                ItemType::Potion => todo!(),
                ItemType::Gold => todo!(),
            }
        }
    }
}