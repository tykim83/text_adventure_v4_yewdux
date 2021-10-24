use gloo_console as console;

use super::{item::{ButtonType, ItemList, ItemLocation, ItemType}, map::{Direction, Location, Map, Room}};

#[derive(Clone, Debug)]
pub struct State {
    pub map: Map,
    pub objects: ItemList,
    pub current_location: Location,
    pub selected_item: Option<ItemType>,
}

impl State {
    pub fn get_item_name(&self, item_type: ItemType) -> &str {
        &self.objects.item_list.get(&item_type).expect("This should always return Some").name
    }

    pub fn get_current_room(&self) -> &Room {
        self.map.rooms.get(&self.current_location).expect("Something went wrong. It should always get a room.")
    }

    pub fn get_current_room_items(&self) -> Vec<ItemType> {
        self.objects.item_list.iter().filter(|(_,v)| {
            match v.location {
                Some(ItemLocation::Room(room_location)) => room_location == self.current_location,
                _ => false,
            }
        }).map(|(k,_)| k).copied().collect()
    }

    pub fn go_to_direction(&self, direction: &Direction) -> Location {
        let room = self.get_current_room();
        room.exit.get(direction).expect("Something went wrong. It should always get a location.").to_owned()
    }

    pub fn action_item(&mut self, button_type: ButtonType) {
        console::log!(format!("Item: {:?} Action: {:?}", self.selected_item, button_type));
        self.selected_item = None;   
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            map: Map::new(),
            current_location: Location::Kitchen,
            selected_item: None,
            objects: ItemList::new(),
        }
    }
}