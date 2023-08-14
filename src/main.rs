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
    1) Sword (1d6) and Shield (+1)\n
    2) Greataxe (B) (1d8)\n
    3) Bow (B) (1d8) and Dagger (1d6)\n
    4) Clouded Orb (focus) and Spell (Random)\n
    \n>>");

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
                let sword = Weapon::new("Sword", "1d6", false);
                c.swap_weapon(sword);
                c.equip_shield();
            }
            else if selection == 2
            {
                let greataxe = Weapon::new("Greataxe", "1d8", true);
                c.swap_weapon(greataxe);
            }
            else if selection == 3
            {
                let bow = Weapon::new("Bow", "1d6", true);
                let dagger = Weapon::new("Dagger", "1d6", false);
                c.swap_weapon(bow);
                c.acquire_weapon(dagger);
            }
            else if selection == 4
            {
                let orb = Weapon::new("Clouded Orb", "focus", true);
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
}


pub fn dungeon_loop(c: &mut Character) {

    let mut dungeon: Dungeon = load_test_dungeon();

    enter_room(dungeon.cur_id(), &mut dungeon);
 
    loop {
        println!("------------------------------------------------------------------------");
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
        //check double-word commands
        if parts.len() == 2 { 
            //enter
            if parts[0].trim() == "enter"
            {
                let goto = parts[1].trim();
                
                lookup_room(goto, &mut dungeon);
            }
            
        }
        else {
            println!("Invalid action!");
            continue; 
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

    dun.cur = room.id;
}
