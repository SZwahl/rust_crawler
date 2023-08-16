use std::borrow::BorrowMut;
use std::io;

mod roll;
use crate::roll::*;
mod creature;
use crate::creature::*;
mod character;
use crate::character::*;
mod equipment;
use crate::equipment::*;
mod dungeons_structs;
use crate::dungeons_structs::*;
mod dungeons;
use crate::dungeons::test_dungeon::*;


fn main() {

    println!("-------------------------------------\n-------------------------------------");

    //Chargen
    let mut c = Character::new_character();
    c.print_character();

    wait_for_continue();

    //Choose equipment
    choose_equipment(&mut c);

    wait_for_continue();

    //load adventure
    println!("Beginning dungeon...");
    dungeon_loop(&mut c);
}

fn choose_equipment(c: &mut Character){
    println!("Choose your equipment: ");
    println!("
    1) Sword (1d6) and Shield (+1)
    2) Greataxe (B) (1d8)
    3) Bow (B) (1d8) and Dagger (1d6)
    4) Clouded Orb (focus) and Spell (Random)
    \n");

    loop {
            let mut selection = String::new();

            io::stdin()
                .read_line(&mut selection)
                .expect("Failed to read line!");

            let selection: u32 = match selection.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid choice"); 
                    continue;
                },
            };
            if selection > 4
            {
                println!("invalid choice");
                continue;
            }

            if selection == 1
            {
                let sword = Weapon::new("Sword", "slash", "1d6", false, StatTypes::Power);
                c.swap_weapon(sword);
                c.equip_shield();
            }
            else if selection == 2
            {
                let greataxe = Weapon::new("Greataxe", "swing", "1d8", true, StatTypes::Power);
                c.swap_weapon(greataxe);
            }
            else if selection == 3
            {
                let bow = Weapon::new("Bow", "shoot", "1d6", true, StatTypes::Finesse);
                let dagger = Weapon::new("Dagger", "stab with", "1d6", false, StatTypes::Finesse);
                c.swap_weapon(bow);
                c.acquire_weapon(dagger);
            }
            else if selection == 4
            {
                let orb = Weapon::new("Clouded Orb", "manipulate", "focus", true, StatTypes::Mind);
                c.swap_weapon(orb);
                let spell = random_spell(roll("1d4").total);
                c.learn_spell(spell);
            }

            break;
    }
}

fn wait_for_input(){
    println!("(What do you do?)");
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line!");
}

fn wait_for_continue(){
    println!("(continue...)");
    let mut entry = String::new();

    io::stdin()
        .read_line(&mut entry)
        .expect("Failed to read line!");

        println!("------------------------------------------------------------------------");
}


pub fn dungeon_loop(c: &mut Character) {

    let mut dungeon: Dungeon = load_test_dungeon();

    enter_room(dungeon.cur_id(), &mut dungeon);
 
    loop {
        println!("------------------------------------------------------------------------");
        let mut combat_action = false;

        //User types command
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line!");

        //command = command.to_lowercase();

        let parts: Vec<&str> = command.split(' ').collect();

        //check single-word commands
        if parts.len() == 0 { 
            println!("Please enter a command");
            continue; 
        }
        //look
        if parts[0].trim() == "look"
        {
            println!("{}.", dungeon.cur_room().desc);
            continue;
        }
        //Inventory
        if parts[0].trim() == "inventory"
        {
            c.print_inventory();
            continue;
        }
        if parts[0].trim() == "spells" {
            c.print_spells();
            continue;
        }
        //hp
        if parts[0] == "hp"
        {
            println!("You have {} HP left.", c.condition.c_hp);
            continue;
        }
        //stats
        if parts[0] == "stats" {
            c.print_character();
            continue;
        }
        //check double-word commands
        if parts.len() == 2 { 
            //enter
            if parts[0].trim() == "enter" {
                let goto = parts[1].trim();
                
                lookup_room(goto, &mut dungeon);
                continue;
            }
            //attack
            else if parts[0].trim() == "attack"
            {
                let is_dead = dungeon.map.get_mut(&dungeon.cur).map(|val| val.attack_enemy(parts[1].trim(), c));
                
                if is_dead.expect("uh") != IsDead::Invalid {
                    combat_action = true;
                }
            }
        }
        if parts.len() == 4 {
            //cast
            if parts[0].trim() == "cast" && parts[2].trim() == "on" {
                //check components valid
                let s_name = parts[1].trim();
                let e_name = parts[3].trim();
                let mut spell = random_spell(1);
                let mut s_valid = false;
                let mut enemy = String::from("");
                let mut e_valid = false;

                //Spell valid?
                for s in &c.i_spells {
                    if s_name == s.key {
                        spell = s.clone();
                        s_valid = true;
                        break;
                    }
                }
                if !s_valid {
                    println!("Invalid spell name!");
                    continue;
                }

                //Enemy valid?
                for e in &dungeon.cur_room().enemies {
                    if e_name == e.name {
                        e_valid = true;
                        break;
                    }
                }
                if !e_valid {
                    println!("Invalid enemy name!");
                    continue;
                }

                //Check cost
                if c.condition.c_mind >= spell.cost {
                    //Exact the price
                    c.condition.c_mind -= spell.cost;
                    //Cast the spell!
                    dungeon.map.get_mut(&dungeon.cur).map(|val| val.cast_spell(e_name, &spell, c));
                    combat_action = true;
                }
                else {
                    println!("Spell is too costly to the mind! You must rest.");
                    continue;
                }

            }
        }
        else {
            println!("Invalid action!");
            continue; 
        }


        if combat_action {
            //loop through enemies and they attack
            for e in &dungeon.cur_room().enemies {

                let damage_total = 2;

                //get condition
                let condition: &mut Condition = &mut c.condition;
                //apply to you
                let is_dead = condition.damage(damage_total, &e.name, &String::from("You"));

                if is_dead == IsDead::Ok {
                    println!("You have {} hp left!", condition.c_hp);
                }
                else if is_dead == IsDead::Dead {
                    println!("You died!");
                    return;
                }
            }
        }
    }

}

fn lookup_room(s: &str, d: &mut Dungeon) ->bool {
    //Loop and compare to exits
    for exit in &d.cur_room().exits
    {
        if exit.name.trim() == s.trim() {
            enter_room(exit.other, d);
            return true;
        }
    }

    println!("No {}", s);
    return false;
}

fn enter_room(r: u32, dun: &mut Dungeon) {
    
    //Lookup room
    let room: &Room = &dun.map[&r];

    println!("{}", room.desc);

    for l in &room.exits {
        println!("{}", l.extra);
    }

    if room.enemies.len() > 0 {
        let mut e_desc: String = String::from("Inside the room is ");
        
        for ix in 0..room.enemies.len() {
            e_desc.push_str(room.enemies[ix].name.as_str());
            e_desc.push_str("(");
            e_desc.push_str(room.enemies[ix].name_s.as_str());
            e_desc.push_str(")");

            if ix != room.enemies.len()-1 {
                e_desc.push_str(", ");
            }
        }

        println!("{e_desc}");
    }

    dun.cur = room.id;
}

