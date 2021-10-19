use yewdux::prelude::*;

pub enum Action {
    GoTo(usize),
}

#[derive(Clone)]
pub struct Map {
    pub rooms: Vec<String>,
    pub current_room: String,
}

impl Default for Map {
    fn default() -> Self {
        Self::new()
    }
}

impl Reducer for Map {
    type Action = Action;

    fn new() -> Self {
        Self {
            rooms: vec![String::from("Room 1"), String::from("Room 2")],
            current_room: String::from("something"),
        }
    }

    fn reduce(&mut self, action: Self::Action) -> Changed {
        match action {
            Action::GoTo(room) => {
                self.current_room = self.rooms[room].to_string();
                true
            }
        }
    }
}
