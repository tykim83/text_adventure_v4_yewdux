use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console as console;
use std::rc::Rc;

use crate::game::map::Direction;
use crate::game::state::{State};

pub struct Compass {
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
}

pub enum Msg {
    State(Rc<State>),
    GoTo(Direction),
}

impl Component for Compass {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            },
            Msg::GoTo(direction) => {
                let location = self.state.go_to_direction(&direction);
                self.dispatch.reduce(move |s| s.current_location = location);
                self.dispatch.reduce(move |s| s.selected_item = None);
                self.dispatch.reduce(move |s| s.log = vec![]);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
  
        let current_room = self.state.get_current_room();
        // console::log!(format!("compass: {:?}", current_room));

        html! {
            <div class="container mb-3">
                <div class="card">
                    <div class="card-header text-info">
                        { "Directions" }
                    </div>
                    <div class="card-body">
                        <div class="row">
                            <div class="col-4"></div>
                            <div class="col-4">
                                <button class="btn btn-primary compass-button"
                                    onclick={ctx.link().callback(|_| Msg::GoTo(Direction::North))}
                                    disabled={current_room.exit.get(&Direction::North).is_none()}> { "North" } 
                                </button>
                            </div>
                            <div class="col-4"></div>
                        </div>
                        <div class="row">
                            <div class="col-4">
                                <button class="btn btn-primary compass-button"
                                    onclick={ctx.link().callback(|_| Msg::GoTo(Direction::West))} 
                                    disabled={current_room.exit.get(&Direction::West).is_none()}> { "West" }
                                </button>
                            </div>
                            <div class="col-4"></div>
                            <div class="col-4">
                                <button class="btn btn-primary compass-button"
                                    onclick={ctx.link().callback(|_| Msg::GoTo(Direction::East))} 
                                    disabled={current_room.exit.get(&Direction::East).is_none()}> { "East" } 
                                </button>
                            </div>
                        </div>
                        <div class="row">
                            <div class="col-4"></div>
                            <div class="col-4">
                                <button class="btn btn-primary compass-button"
                                    onclick={ctx.link().callback(|_| Msg::GoTo(Direction::South))} 
                                    disabled={current_room.exit.get(&Direction::South).is_none()}> { "South" } 
                                </button>
                            </div>
                            <div class="col-4"></div>
                        </div>

                    </div>
                </div> 
            </div>
        }
    }
}
