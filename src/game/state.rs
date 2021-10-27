use gloo_console as console;

use super::{item::{ButtonType, ItemList, ItemLocation, ItemType}, map::{Direction, Location, Map, Room}};

#[derive(Clone, Debug)]
pub struct State {
    pub map: Map,
    pub objects: ItemList,
    pub current_location: Location,
    pub selected_item: Option<ComponentType>,
    pub log: Vec<String>,
}

impl State {
    pub fn get_item_name(&self, item_type: ItemType) -> &str {
        &self.objects.item_list.get(&item_type).expect("This should always return Some").name
    }

    pub fn get_item_room_description(&self, item_type: ItemType) -> Option<&str> {
        let item = self.objects.item_list.get(&item_type).expect("This should always return Some");
        Some(item.room_description.get(item.active_room_description?)?)
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

    pub fn get_inventory_items(&self) -> Vec<ItemType> {
        self.objects.item_list.iter().filter(|(_,v)| {
            matches!(v.location, Some(ItemLocation::Inventory))
        }).map(|(k,_)| k).copied().collect()
    }

    pub fn go_to_direction(&self, direction: &Direction) -> Location {
        let room = self.get_current_room();
        room.exit.get(direction).expect("Something went wrong. It should always get a location.").to_owned()
    }

    pub fn action_item(&mut self, button_type: ButtonType) {
        match button_type {
            ButtonType::Look => self.look_item(),
            ButtonType::Take => self.take_item(),
            ButtonType::Use => (),
        }
        self.selected_item = None;   
    }

    fn take_item(&mut self) {   
        // Refactor this
        let comp = &self.selected_item.expect("This should always ");
        let item = match comp {
            ComponentType::_Compass(item) => item,
            ComponentType::Items(item) => item,
            ComponentType::Inventory(item) => item,
        };
        let item = self.objects.item_list.get_mut(item).expect("This should always ");
        self.log.push(item.pick_up.log.to_owned());

        // Pick up item
        if item.pick_up.can_be_pick_up {
            item.location = Some(ItemLocation::Inventory);
        }

        // Modify room text
        if let Some((item, index)) = item.pick_up.item_description_to_be_changed {
            let item = self.objects.item_list.get_mut(&item).expect("This should always ");
            item.active_room_description = Some(index);
        }
    }

    fn look_item(&mut self) {  
        let comp = &self.selected_item.expect("An item should always be selected"); 
        let item = match comp {
            ComponentType::_Compass(item) => item,
            ComponentType::Items(item) => item,
            ComponentType::Inventory(item) => item,
        };
        let item = &self.objects.item_list.get(item).expect("This should always ").description;
        self.log.push(item.to_string());
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            map: Map::new(),
            current_location: Location::Kitchen,
            selected_item: None,
            objects: ItemList::new(),
            log: vec![],
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
pub enum ComponentType {
    _Compass(ItemType),
    Items(ItemType),
    Inventory(ItemType),
}