use gloo_console as console;

use super::{
    item::{ButtonType, ComponentType, ItemList, ItemLocation, ItemType},
    map::{Direction, Location, Map, Room},
};

#[derive(Clone, Debug)]
pub struct State {
    pub map: Map,
    pub objects: ItemList,
    pub current_location: Location,
    pub selected_item: Option<ComponentType>,
    pub log: Vec<String>,
    pub is_game_over: bool,
}

impl State {
    pub fn get_item_name(&self, item_type: ItemType) -> &str {
        &self
            .objects
            .item_list
            .get(&item_type)
            .expect("This should always return Some")
            .name
    }

    pub fn get_item_room_description(&self, item_type: ItemType) -> Option<&str> {
        let item = self
            .objects
            .item_list
            .get(&item_type)
            .expect("This should always return Some");
        Some(item.room_description.get(item.active_room_description?)?)
    }

    pub fn get_current_room(&self) -> &Room {
        self.map
            .rooms
            .get(&self.current_location)
            .expect("Something went wrong. It should always get a room.")
    }

    pub fn get_current_room_items(&self) -> Vec<ItemType> {
        self.objects
            .item_list
            .iter()
            .filter(|(_, v)| match v.location {
                Some(ItemLocation::Room(room_location)) => room_location == self.current_location,
                _ => false,
            })
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    pub fn get_inventory_items(&self) -> Vec<ItemType> {
        self.objects
            .item_list
            .iter()
            .filter(|(_, v)| matches!(v.location, Some(ItemLocation::Inventory)))
            .map(|(k, _)| k)
            .copied()
            .collect()
    }

    pub fn go_to_direction(&self, direction: &Direction) -> Option<Location> {
        let room = self.get_current_room();
        let door = room.exit
            .get(direction)?;
            
        if !door.is_locked { Some(door.location.to_owned()) } else { None }
    }

    pub fn action_item(&mut self, button_type: ButtonType) {
        match button_type {
            ButtonType::Look => self.look_item(),
            ButtonType::Take => self.take_item(),
            ButtonType::Use => self.use_item(),
        }
        self.selected_item = None;
    }

    fn take_item(&mut self) {
        // Refactor this
        let comp = &self.selected_item.expect("This should always ");
        let item = match comp {
            ComponentType::Items(item) => item,
            ComponentType::Inventory(item) => item,
        };
        let item = self
            .objects
            .item_list
            .get_mut(item)
            .expect("This should always ");
        self.log.push(item.pick_up.log.to_owned());

        // Pick up item
        if item.pick_up.can_be_pick_up {
            item.location = Some(ItemLocation::Inventory);
        }

        // Modify room text
        if let Some((item, index)) = item.pick_up.item_description_to_be_changed {
            let item = self
                .objects
                .item_list
                .get_mut(&item)
                .expect("This should always ");
            item.active_room_description = Some(index);
        }
    }

    fn look_item(&mut self) {
        let comp = &self.selected_item.expect("An item should always be selected");
        let item = match comp {
            ComponentType::Items(item) => item,
            ComponentType::Inventory(item) => item,
        };
        let item = &self
            .objects
            .item_list
            .get(item)
            .expect("This should always ")
            .description;
        self.log.push(item.to_string());
    }

    fn use_item(&mut self) {
        let comp = &self.selected_item.expect("An item should always be selected");
        let item = match comp {
            ComponentType::Items(item) => item,
            ComponentType::Inventory(item) => item,
        };

        match item {
            ItemType::Key if self.current_location == Location::Kitchen => {
                let room = self.map.rooms.get_mut(&Location::Kitchen).expect("Should always get a room");
                let door = room.exit.get_mut(&Direction::North).expect("Should always get a door");
                door.is_locked = false;
                let item = self.objects.item_list.get_mut(&ItemType::Key).expect("Should always return an item");
                item.location = None;
                self.log.push("The door is now unlocked".to_string());
            },
            ItemType::Game if self.current_location == Location::GameRoom => {
                self.is_game_over = true;
            },
            _ => self.log.push("I cant do it".to_string()),
        }
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
            is_game_over: false,
        }
    }
}
