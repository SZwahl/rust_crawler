use crate::{dungeons_structs::*, creature::{Creature, Condition, Attack, StatTypes}, equipment::*};

pub fn load_test_dungeon() -> Dungeon {
    //Make dungeon
    let mut dun: Dungeon = Dungeon::new();

    //Make rooms
    let room1 = room1();
    let room2 = room2();
    let room3 = room3();
    let room4 = room4();
    let room5 = room5();
    
    //Add them all
    dun.map.insert(1, room1);
    dun.map.insert(2, room2);
    dun.map.insert(3, room3);
    dun.map.insert(4, room4);
    dun.map.insert(5, room5);

    return dun;
}

fn room1() -> Room {
    //Room 1 start point
    let mut room1 = Room::new(1);
    room1.add_desc("You are in narrow ravine, having just descended by rope. It's dim but bright enough to see.");
    room1.add_link(Link::new(2, "Tunnel", "Carved into the ravine wall is a jagged hole - a [Tunnel] - leading into the dark."));
    return room1;
}

fn room2() -> Room {
    //Room 2 skeleton chest room
    let mut room2 = Room::new(2);
    room2.add_desc("This room is circular, about 30 feet wide.");
    //Link to start
    room2.add_link(Link::new(1, "Tunnel", "The [Tunnel] extends back to the ravine you entered from."));
    room2.add_link(Link::new(3, "Crevice", "A [Crevice] you could squeeze through is on the right."));
    room2.add_link(Link::new(4, "Hallway", "A [Hallway] lit with braziers extends to your left."));    

    //Add skeletons
    let mut skeleton = Creature::new("Skeleton", "s", Condition::new(4, 6, 8, 2));
    skeleton.add_attack(
        Attack::new(
            "swings its rusty sword", 
            "1d4", 
            StatTypes::None, 
            StatTypes::None, 
            0)
    );
    room2.add_enemies(skeleton, 2);

    //Add chest
    let mut gilded_box = Chest::new(
        "Gilded [Box]", 
        "Box",
        "in a pile of ruined bones."
    );
    gilded_box.add(ItemType::Potion);
    room2.add_chest(gilded_box);
    return room2;
}

fn room3() -> Room {
    let mut room3 = Room::new(3);
    room3.add_desc("This room lowers into a shallow puddle about 20 feet square.");
    room3.add_link(Link::new(2, "Crevice", "The [Crevice] you came through extends darkly back."));

    //add bat
    let mut bat = Creature::new("Great Rotten Bat", "grb", Condition::new(7, 10, 12, 5));
    bat.add_attack(
        Attack::new(
            "shrieks, emitting a sonic boom",
            "1d6",
            StatTypes::None,
            StatTypes::Mind,
            2,
        )
    );
    bat.add_attack(
        Attack::new(
            "bites at you with razor-sharp fangs",
            "1d6",
            StatTypes::None,
            StatTypes::None,
            0
        )
    );
    room3.add_enemies(bat, 1);

    //Add chest
    let mut sunken_chest = Chest::new(
        "Sunken [Chest]", 
        "Chest",
        "shallowly visible in the dark clear water."
    );
    sunken_chest.add(ItemType::Weapon);
    sunken_chest.add(ItemType::Armor);
    sunken_chest.add(ItemType::Potion);
    sunken_chest.add(ItemType::Spell);
    room3.add_chest(sunken_chest);

    return room3;
}

fn room4() -> Room {
    let mut room4 = Room::new(4);
    room4.add_desc("This room is small and stuffy, crowded with stalagtites. Several fallen adventurers lay crumpled about the ground.");
    room4.add_link(Link::new(2, "Hallway", "The [Hallway] you came through is behind."));
    room4.add_link(Link::new(5, "Door", "An out-of-place looking Iron [Door] stands before you, cast with runes."));

    let mut wep = Chest::new(
        "[Bag]", 
        "Bag",
        "held by one of the adventurers, which seems to hold a weapon in good condition..."
    );
    wep.add(ItemType::Weapon);
    room4.add_chest(wep);

    return room4;
}

fn room5() -> Room {
    let mut room5 = Room::new(5);
    room5.add_desc("This room is big, about 40 feet long and 20 wide. \nAbout the walls are various arcane sigils scrawled in chalk, and the ceiling is hastily painted with a simulacrum of the night sky.");

    //Add wizard
    let mut cw = Creature::new("Celestial Warlock", "cw", Condition::new(8, 4, 4, 16));
    cw.add_attack(
        Attack::new(
            "emits an enfeebling wave",
            "1d1",
            StatTypes::None,
            StatTypes::Power,
            2
        )
    );

    let mut golem = Creature::new("Deep-Space Golem", "dsg", Condition::new(12, 16, 8, 2));
    golem.add_attack(
        Attack::new(
            "opens a mass of flesh, striking out with a number of limbs",
            "2d3",
            StatTypes::None,
            StatTypes::None,
            0
        )
    );
    room5.add_enemies(cw, 1);
    room5.add_enemies(golem, 1);

    return room5;
}