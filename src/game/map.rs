use maplit::hashmap;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Map {
    pub rooms: HashMap<Location, Room>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            rooms: hashmap!{
                Location::GameRoom => {
                    Room {
                        name: String::from("Game room"),
                        description: String::from("There is a computer"),
                        exit: hashmap! { Direction::South => Location::Kitchen },
                    }
                },
                Location::Kitchen => {
                    Room {
                        name: String::from("Kitchen"),
                        description: String::from(
                            "There is a table with a key on it. A door leading north.",
                        ),
                        exit: hashmap! { Direction::North => Location::GameRoom },
                    }
                },
            }
        }
    }

    pub fn get_current_room(&self, location: &Location) -> Room {
        self.rooms[location].clone()
    }
}

#[derive(Debug, Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub exit: HashMap<Direction, Location>,
}

impl Room {
    pub fn _get_direction(&self, dir: &Direction) -> Option<Location> {
        self.exit.get(dir).copied()
    }
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
    _East,
    _West,
}