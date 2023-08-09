use crate::dungeons_structs::*;

pub fn load_test_dungeon() -> Room {
    //Make rooms
    let mut room1 = Room::new(1);
    room1.add_desc("You are in narrow ravine, your rope dangling from your descent.");
    room1.add_link(Link::new(2, "Door", "Carved into the ravine wall is a jagged hole - a tunnel - leading into the dark."));

    let mut room2 = Room::new(2);
    room2.add_desc("This room is circular, about 30 feet wide.");
    room1.add_link(Link::new(1, "Door", "that extends into a dark hallway."));

    return room1;
}