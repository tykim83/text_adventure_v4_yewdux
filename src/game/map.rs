use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Map {
    pub rooms: HashMap<Location, Room>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            rooms: hashmap! {
                Location::GameRoom => {
                    Room {
                        name: String::from("Game room"),
                        description: String::from("There is a computer"),
                        exit: hashmap! { Direction::South => Door {
                                location: Location::Kitchen,
                                is_locked: false,
                            }
                        },
                    }
                },
                Location::Kitchen => {
                    Room {
                        name: String::from("Kitchen"),
                        description: String::from(
                            "It's really messy, dishes are still dirty and there old boxes of pizza on the floor.",
                        ),
                        exit: hashmap! { Direction::North => Door {
                                location: Location::GameRoom,
                                is_locked: true,
                            }
                        },
                    }
                },
            },
        }
    }
}

impl Default for Map {
    fn default() -> Self {
        Map::new()
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exit: HashMap<Direction, Door>,
}

#[derive(Debug, Clone)]
pub struct Door {
    pub location: Location,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Location {
    GameRoom,
    Kitchen,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}
