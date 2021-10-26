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
                        room_description: vec! {},
                        active_room_description: None,
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                        pick_up: PickUp {
                            can_be_pick_up: true,
                            log: String::from("I've got a key"),
                            item_description_to_be_changed: Some((ItemType::Table, 1)),
                        }
                    }
                },
                ItemType::Table => {
                    Item {
                        name: String::from("Table"),
                        description: String::from("It's a table"),
                        room_description: vec! {
                            String::from("There is an old wooden table with some empty beers on it, also a key is on it."),
                            String::from("There is an old wooden table with some empty beers on it."),
                        },
                        active_room_description: Some(0),
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                        pick_up: PickUp {
                            can_be_pick_up: false,
                            log: String::from("I can't do it."),
                            item_description_to_be_changed: None,
                        }
                    }
                },
                ItemType::Door => {
                    Item {
                        name: String::from("Door"),
                        description: String::from("It's a door"),
                        room_description: vec! {
                            String::from("A door leading north."),
                        },
                        active_room_description: Some(0),
                        location: Some(ItemLocation::Room(Location::Kitchen)),
                        pick_up: PickUp {
                            can_be_pick_up: false,
                            log: String::from("I can't do it."),
                            item_description_to_be_changed: None,
                        }
                    }
                },
                ItemType::Game => {
                    Item {
                        name: String::from("Monekey Island"),
                        description: String::from("It's a copy of Monekey Island"),
                        room_description: vec! {},
                        active_room_description: None,
                        location: Some(ItemLocation::Inventory),
                        pick_up: PickUp {
                            can_be_pick_up: false,
                            log: String::from("I can't do it."),
                            item_description_to_be_changed: None,
                        }
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
    pub room_description: Vec<String>,
    pub active_room_description: Option<usize>,
    pub location: Option<ItemLocation>,
    pub pick_up: PickUp,
}

#[derive(Clone, Debug)]
pub struct PickUp {
    pub can_be_pick_up: bool,
    pub log: String,
    pub item_description_to_be_changed: Option<(ItemType, usize)>,
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