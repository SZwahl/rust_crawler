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
        pub hp: u32,
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

            hp: hp,
            armor: 0,
    
            c_power: pow,
            c_finesse: fin,
            c_mind: mi,
            c_hp: hp,
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

pub struct Creature {
    pub name: String,
    pub con: Condition,
}
