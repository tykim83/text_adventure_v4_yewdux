use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::dispatch::DispatchProps;
use gloo_console as console;

use crate::game::map::Direction;
use crate::game::state::{State, Action};

pub struct Compass;

pub type MyCompass = WithDispatch<Compass>;

impl Component for Compass {
    type Message = ();
    type Properties = DispatchProps<ReducerStore<State>>; 

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = ctx.props().state();
        let current_room = state.get_current_room();

        console::log!(format!("compass: {:?}", state.current_location));

        html! {
            <div class="container">
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                            onclick={ctx.props().callback(|_| Action::GoTo(Direction::North))}
                            disabled={current_room.exit.get(&Direction::North).is_none()}> { "North" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
                <div class="row">
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                            onclick={ctx.props().callback(|_| Action::GoTo(Direction::West))} 
                            disabled={current_room.exit.get(&Direction::West).is_none()}> { "West" }
                        </button>
                    </div>
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                            onclick={ctx.props().callback(|_| Action::GoTo(Direction::East))} 
                            disabled={current_room.exit.get(&Direction::East).is_none()}> { "East" } 
                        </button>
                    </div>
                </div>
                <div class="row">
                    <div class="col-4"></div>
                    <div class="col-4">
                        <button class="btn btn-outline-primary compass-button"
                            onclick={ctx.props().callback(|_| Action::GoTo(Direction::South))} 
                            disabled={current_room.exit.get(&Direction::South).is_none()}> { "South" } 
                        </button>
                    </div>
                    <div class="col-4"></div>
                </div>
            </div>
        }
    }
}
