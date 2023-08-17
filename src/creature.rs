#[derive(Clone, PartialEq)]
pub enum StatTypes {
    Power,
    Finesse,
    Mind,
    Health,
    None,
}

#[derive(PartialEq)]
pub enum IsDead {
    Ok,
    Dead,
    Invalid,
}

#[derive(Copy, Clone)]
pub struct Condition {
        //Stats
        pub power: u32,
        pub finesse: u32,
        pub mind: u32,
    
        pub mod_power: i8,
        pub mod_finesse: i8,
        pub mod_mind: i8,
    
        //Condition
        pub tot_hp: u32,
        pub armor: u32,
    
        //Current stats
        pub c_power: u32,
        pub c_finesse: u32,
        pub c_mind: u32,
        pub c_hp: u32,
}

impl Condition {
    pub fn new(hp: u32, pow: u32, fin: u32, mi: u32) -> Self {
        Self {
            power: pow,
            finesse: fin,
            mind: mi,

            mod_power: calc_mod(pow),
            mod_finesse: calc_mod(fin),
            mod_mind: calc_mod(mi),

            tot_hp: hp,
            armor: 0,
    
            c_power: pow,
            c_finesse: fin,
            c_mind: mi,
            c_hp: hp,
        }
    }

    pub fn damage(&mut self, dam: i32, n1: &String, n2: &String) -> IsDead {        
        //subtract armor from damage
        let mut tot_dam = dam - self.armor as i32;
        if tot_dam < 0 { tot_dam = 0; }

        //subtract damage from hit protect
        let mut uh: i32 = self.c_hp as i32;
        uh -= tot_dam;
        if uh < 0 { uh = 0; }
        self.c_hp = uh as u32;

        let mut doesdo = "";

        //lol
        if n1 == "You" { doesdo = "do";}
        else { doesdo = "does"; }

        println!("{} {} {} damage to {}.", n1, doesdo, tot_dam, n2);

        if self.c_hp == 0 {
            return IsDead::Dead;
        }

        return IsDead::Ok;
    }

    pub fn damage_stat(&mut self, dam: i32, st: StatTypes) -> IsDead {
        //subtract damage from current
        let mut new_total: i32 = 0;

        if st == StatTypes::Power {
            new_total = self.c_power as i32;
        }
        else if st == StatTypes::Finesse {
            new_total = self.c_finesse as i32;
        }
        else if st == StatTypes::Mind {
            new_total = self.c_mind as i32;
        }

        new_total -= dam;
        if new_total < 0 { new_total = 0; }

        //Apply
        if st == StatTypes::Power {
            self.c_power = new_total as u32;
            self.mod_power = calc_mod(self.c_power);
            println!("Your power decreases by {}! Now {}/{} ({}).", dam, self.c_power, self.power, self.mod_power);
        }
        else if st == StatTypes::Finesse {
            self.c_finesse = new_total as u32;
            self.mod_finesse = calc_mod(self.c_finesse);
            println!("Your finesse decreases by {}! Now {}/{} ({}).", dam, self.c_finesse, self.finesse, self.mod_finesse);
        }
        else if st == StatTypes::Mind {
            self.c_mind = new_total as u32;
            self.mod_mind = calc_mod(self.c_mind);
            println!("Your mind decreases by {}! Now {}/{} ({}).", dam, self.c_mind, self.mind, self.mod_mind);
        }

        if new_total == 0 {
            println!("Stat decreased to 0!");
            return IsDead::Dead;
        }
        else { return IsDead::Ok; }
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

#[derive(Clone)]
pub struct Attack {
    pub desc: String,
    pub damage: String,
    pub save: StatTypes,
    pub eff_type: StatTypes,
    pub eff_damage: u32,
}

impl Attack {
    pub fn new(des: &str, dam: &str, sav: StatTypes, eff_t: StatTypes, eff_dam: u32) -> Self {
        Self {
            desc: String::from(des),
            damage: String::from(dam),
            save: sav,
            eff_type: eff_t,
            eff_damage: eff_dam,
        }
    }
}

#[derive(Clone)]
pub struct Creature {
    pub name: String,
    pub key: String,
    pub con: Condition,
    pub attacks: Vec<Attack>,
}

impl Creature {
    pub fn new(n: &str, n_s: &str, con: Condition) -> Self {
        Self {
            name: String::from(n),
            key: String::from(n_s),
            con: con,
            attacks: Vec::new(),
        }
    }

    pub fn get_condition(&mut self) -> &mut Condition {
        return &mut self.con;
    }

    pub fn add_attack(&mut self, a: Attack) {
        self.attacks.push(a);
    }
}