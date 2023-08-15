use crate::{dungeons_structs::*, creature::{Creature, Condition, Attack, StatTypes}};

pub fn load_test_dungeon() -> Dungeon {
    //Make dungeon
    let mut dun: Dungeon = Dungeon::new();

    //Make rooms
    let mut room1 = Room::new(1);
    room1.add_desc("You are in narrow ravine, having just descended by rope. It's dim but bright enough to see.");
    room1.add_link(Link::new(2, "Tunnel", "Carved into the ravine wall is a jagged hole - a [Tunnel] - leading into the dark."));
    
    dun.map.insert(1, room1);

    let mut room2 = Room::new(2);
    room2.add_desc("This room is circular, about 30 feet wide.");
    room2.add_link(Link::new(1, "Tunnel", "The [Tunnel] extends back to the ravine you entered from."));
        
    let mut skeleton = Creature::new("Skeleton", "S", Condition::new(4, 6, 8, 2));
    skeleton.add_attack(
        Attack::new(
            "swings its sword", 
            "1d6", 
            StatTypes::None, 
            StatTypes::None, 
            "0")
    );
    room2.add_enemies(skeleton, 2);
    
    dun.map.insert(2, room2);


    return dun;
}