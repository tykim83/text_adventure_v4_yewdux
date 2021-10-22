use super::map::{Location, Map, Room};

#[derive(Clone, Debug)]
pub struct State {
    pub map: Map,
    pub current_location: Location,
}

impl State {
    pub fn get_current_room(&self) -> &Room {
        self.map.rooms.get(&self.current_location).expect("Something went wrong. It should always get a room.")
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