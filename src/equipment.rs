// enum WeaponTypes {
//     Crude,
//     Hand,
//     Field,
//     Noble,
//     Heavy,
// }

use crate::creature::StatTypes;

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub verb: String,
    pub roll: String,
    pub bulky: bool,
    pub stat: StatTypes,
}

impl Weapon {
    pub fn new(n: &str, v: &str, r: &str, b: bool, t: StatTypes) -> Self {
        Self {
            name: String::from(n),
            verb: String::from(v),
            roll: String::from(r),
            bulky: b,
            stat: t,
        }
    }

    pub fn print(&self) -> String {
        let mut printed: String = self.name.clone();

        if self.bulky
        {
            printed.push_str(" (B)");
        }

        return printed;
    }
}

#[derive(Clone)]
pub struct Armor {
    pub name: String,
    pub bonus: u32,
    pub bulky: bool,
}

impl Armor {
    pub fn new(n: &str, bonus: u32, bulky: bool) -> Self {
        Self {
            name: String::from(n),
            bonus: bonus,
            bulky: bulky,
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
}