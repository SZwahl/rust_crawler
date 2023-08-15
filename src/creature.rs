#[derive(Clone)]
pub enum StatTypes {
    Power,
    Finesse,
    Mind,
    Health,
    None,
}

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
    pub eff_damage: String,
}

impl Attack {
    pub fn new(des: &str, dam: &str, sav: StatTypes, eff_t: StatTypes, eff_dam: &str) -> Self {
        Self {
            desc: String::from(des),
            damage: String::from(dam),
            save: sav,
            eff_type: eff_t,
            eff_damage: String::from(eff_dam),
        }
    }
}

#[derive(Clone)]
pub struct Creature {
    pub name: String,
    pub con: Condition,
    pub attacks: Vec<Attack>,
}

impl Creature {
    pub fn new(n: &str, con: Condition) -> Self {
        Self {
            name: String::from(n),
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