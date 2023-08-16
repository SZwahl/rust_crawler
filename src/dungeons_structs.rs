use std::borrow::BorrowMut;
use std::collections::HashMap;

use crate::creature::*;
use crate::character::*;
use crate::equipment::Spell;
use crate::roll::*;

#[derive(Clone)]
pub struct Room {
    pub id: u32,
    pub desc: String,
    pub exits: Vec<Link>,
    pub enemies: Vec<Creature>,
}

impl Room {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            desc: String::from(""),
            exits: Vec::new(),
            enemies: Vec::new(),
        }
    }

    pub fn add_desc(&mut self, desc: &str) {
        self.desc = String::from(desc);
    }

    pub fn add_link(&mut self, link: Link) {
        self.exits.push(link);
    }

    pub fn add_enemies(&mut self, enemy: Creature, amt: u32) {
        //adding just 1
        if amt == 1 {
            self.enemies.push(enemy);
            return;
        }

        //Clone and iterate for more
        for ix in 1..=amt{
            let mut e = enemy.clone();
           //e.name_s.push_str("_");
            e.key.push_str(ix.to_string().as_str()); 
            self.enemies.push(e);
        }
    }

    pub fn attack_enemy(&mut self, e_str: &str, c: &Character) -> IsDead {
        //Get enemies
        let room_enemies = &mut self.enemies;

        //loop and attack valid one
        for num in 0..room_enemies.len() {
            let e = &mut room_enemies[num];
            //is valid
            if &e.key == e_str {
                let damage_roll = roll(c.e_weapon.roll.as_str());
                let d_mod = c.get_wep_mod();
                let damage_total: i32 = damage_roll.total as i32 + i32::from(d_mod);
            
                //construct breakdown
                let mut brkdwn = damage_roll.individuals;
                //brkdwn.insert(0, '');
                brkdwn.push_str("+");
            
                //print breakdown
                println!("You {} your {}, rolling a total of {} ({}{})", c.e_weapon.verb, c.e_weapon.name, damage_total, brkdwn, d_mod);
            
            
                let condition: &mut Condition = &mut e.con;
                //apply to enemy
                let is_dead = condition.damage(damage_total, &String::from("You"), &e.name);

                match is_dead {
                    IsDead::Ok => {
                        println!("{} has {} hp left!", e.name, condition.c_hp);
                    }
                    IsDead::Dead => {
                        println!("{} dies!", e.name);
                        room_enemies.remove(num);
                    }
                    IsDead::Invalid => todo!(),
                }


                return is_dead;
            }
        }
        println!("Invalid enemy \"{}\"", e_str);
        return IsDead::Invalid;
    }

    pub fn cast_spell(&mut self, e_str: &str, s: &Spell, c: &Character){
        //Get enemies
        let room_enemies = &mut self.enemies;

        //Roll result
        let damage_roll = roll(&s.damage);
        let d_mod = c.get_wep_mod();
        let damage_total: i32 = damage_roll.total as i32 + i32::from(d_mod);
    
        //construct breakdown
        let mut brkdwn = damage_roll.individuals;
        //brkdwn.insert(0, '');
        brkdwn.push_str("+");
    
        //print breakdown
        println!("{}. You roll a total of {} ({}{})", s.cast_desc, damage_total, brkdwn, d_mod);

        //loop and cast spell on valid
        let mut num = 0;
        loop {
            let e = &mut room_enemies[num];
            //is valid
            if &e.key == e_str || s.multi { 
                let condition: &mut Condition = &mut e.con;
                //apply to enemy
                let is_dead = condition.damage(damage_total, &String::from("You"), &e.name);

                match is_dead {
                    IsDead::Ok => {
                        println!("{} has {} hp left!", e.name, condition.c_hp);
                        num+=1;
                    }
                    IsDead::Dead => {
                        println!("{} dies!", e.name);
                        room_enemies.remove(num);
                    }
                    IsDead::Invalid => todo!(),
                }

            }
            if num == room_enemies.len() { break; }
        }
    }
}

#[derive(Clone)]
pub struct Link {
    pub other: u32,
    pub name: String,
    pub extra: String,
}

impl Link {
    pub fn new(o: u32, n: &str, e: &str) -> Self {
        Self {
            other: o,
            name: String::from(n),
            extra: String::from(e),
        }
    }
}

pub struct Dungeon {
    pub map: HashMap<u32, Room>,
    pub cur: u32,
}

impl Dungeon {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
            cur: 1,
        }
    }

    pub fn cur_room(&self) -> &Room {
        return &self.map[&self.cur];
    }

    pub fn cur_id(&self) -> u32 {
        return self.cur;
    }
}