use maplit::hashmap;
use std::collections::HashMap;

use super::map::Location;

#[derive(Debug, Clone)]
pub struct ItemList {
    pub item_list: HashMap<ItemType, Item>,
}

impl ItemList {
    pub fn new() -> Self {
        Self {
            item_list: hashmap! {
                ItemType::Key => {
                    Item {
                        name: String::from("Key"),
                        description: String::from("It's a key"),
                        can_pick_up: true,
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                    }
                },
                ItemType::Table => {
                    Item {
                        name: String::from("Table"),
                        description: String::from("It's a table"),
                        can_pick_up: false,
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                    }
                },
                ItemType::Door => {
                    Item {
                        name: String::from("Door"),
                        description: String::from("It's a door"),
                        can_pick_up: false,
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                    }
                },
                ItemType::Game => {
                    Item {
                        name: String::from("Monekey Island"),
                        description: String::from("It's a copy of Monekey Island"),
                        can_pick_up: true,
                        location: Some(ItemLocation::Inventory),
                    }
                },
            }                      
        }
    }
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub description: String,
    pub can_pick_up: bool,
    pub location: Option<ItemLocation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ItemType {
    Key,
    Table,
    Door,
    Game,
}

#[derive(Clone, Debug, Copy)]
pub enum ItemLocation {
    Room(Location),
    Inventory,
}

#[derive(Clone, Debug, Copy)]
pub enum ButtonType {
    Look,
    Take,
}