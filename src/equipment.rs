// enum WeaponTypes {
//     Crude,
//     Hand,
//     Field,
//     Noble,
//     Heavy,
// }

#[derive(Clone)]
pub struct Weapon {
    pub name: String,
    pub roll: String,
    pub bulky: bool,
}

impl Weapon {
    pub fn new(n: &str, r: &str, b: bool) -> Self {
        Self {
            name: String::from(n),
            roll: String::from(r),
            bulky: b,
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
pub enum SaveTypes {
    Power,
    Finesse,
    Mind,
    None,
}

#[derive(Clone)]
pub struct Spell {
    pub name: String,
    pub cast_desc: String,
    pub cost: u32,
    pub damage: String,
    pub enemy_save: SaveTypes,
    pub multi: bool,
}

impl Spell {
    pub fn new(n: &str, desc: &str, cost: u32, dam: &str, save: SaveTypes, mult: bool) -> Self {
        Self {
            name: String::from(n),
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
            "Static surrounds your orb and an electric bolt leaps from your fingers.",
        1,
        "1d8",
            SaveTypes::None,
            false
        );

    let fireball = 
            Spell::new(
                "Fireball",
                "Your orb turns white and emits a thin beam, which explodes in a blinding flash.",
                2,
                "1d10",
                SaveTypes::Finesse,
                true
            );

    let moonbeams = 
            Spell::new(
                "Moonbeams",
                "Your orb glows bale blue and refracts large discs of light into the enemy.",
                1,
                "1d6",
                SaveTypes::None,
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