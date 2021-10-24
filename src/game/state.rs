use super::{item::Item, map::{Direction, Location, Map, Room}};

#[derive(Clone, Debug)]
pub struct State {
    pub map: Map,
    pub current_location: Location,
}

impl State {
    pub fn get_current_room(&self) -> &Room {
        self.map.rooms.get(&self.current_location).expect("Something went wrong. It should always get a room.")
    }

    pub fn get_current_room_items(&self) -> Vec<Item> {
        self.map.rooms.get(&self.current_location).expect("Something went wrong. It should always get a room.").items.to_owned()
    }

    pub fn go_to_direction(&self, direction: &Direction) -> Location {
        let room = self.get_current_room();
        room.exit.get(direction).expect("Something went wrong. It should always get a location.").to_owned()
    }
}

impl Default for State {
    fn default() -> Self {
        Self {
            map: Map::new(),
            current_location: Location::Kitchen,
        }
    }
}