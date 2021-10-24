use maplit::hashmap;
use std::collections::HashMap;

use crate::game::item::ItemType;

use super::item::Item;

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
                        items: vec![],
                    }
                },
                Location::Kitchen => {
                    Room {
                        name: String::from("Kitchen"),
                        description: String::from(
                            "There is a table with a key on it. A door leading north.",
                        ),
                        exit: hashmap! { Direction::North => Location::GameRoom },
                        items: vec![
                            Item {
                                id: ItemType::Key,
                                name: String::from("Key"),
                                description: String::from("It's a key"),
                                can_pick_up: true,
                            },
                            Item {
                                id: ItemType::Table,
                                name: String::from("Table"),
                                description: String::from("It's a table"),
                                can_pick_up: false,
                            },
                            Item {
                                id: ItemType::Door,
                                name: String::from("Door"),
                                description: String::from("It's a door"),
                                can_pick_up: false,
                            },
                        ],
                    }
                },
            }
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
    pub exit: HashMap<Direction, Location>,
    pub items: Vec<Item>
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