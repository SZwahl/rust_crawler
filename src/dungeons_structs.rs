use std::collections::HashMap;


pub struct Room {
    pub id: u32,
    pub desc: String,
    pub exits: Vec<Link>,
}

impl Room {
    pub fn new(id: u32) -> Self {
        Self {
            id: id,
            desc: String::from(""),
            exits: Vec::new(),
        }
    }

    pub fn add_desc(&mut self, desc: &str) {
        self.desc = String::from(desc);
    }

    pub fn add_link(&mut self, link: Link) {
        self.exits.push(link);
    }
}

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