use yewdux::prelude::*;

use super::map::{Location, Map};

pub enum Action {
    // GoTo(usize),
}

#[derive(Clone)]
pub struct State {
    pub map: Map,
    pub current_location: Location,
}

impl Default for State {
    fn default() -> Self {
        Self::new()
    }
}

impl Reducer for State {
    type Action = Action;

    fn new() -> Self {
        Self {
            map: Map::new(),
            current_location: Location::Kitchen,
        }
    }

    fn reduce(&mut self, _action: Self::Action) -> Changed {
        // match action {
        //     Action::GoTo(room) => {
        //         self.current_room = self.rooms[room].to_string();
        //         true
        //     }
        // }
        true
    }
}
