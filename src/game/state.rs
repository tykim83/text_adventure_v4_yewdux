use yewdux::prelude::*;

use super::map::{Direction, Location, Map, Room};

pub enum Action {
    GoTo(Direction),
}

#[derive(Clone)]
pub struct State {
    pub map: Map,
    pub current_location: Location,
}

impl State {
    pub fn get_current_room(&self) -> &Room {
        self.map.rooms.get(&self.current_location).expect("Something went wrong. It should always get a room.")
    }
}

// impl Default for State {
//     fn default() -> Self {
//         Self::new()
//     }
// }

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self {
            map: Map::new(),
            current_location: Location::Kitchen,
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::GoTo(direction) => {
                let room = self.get_current_room();
                self.current_location = *room.exit.get(&direction).unwrap();
                true
            }
        }
    }
}
