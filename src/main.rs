use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console as console;
use std::rc::Rc;

mod game;
use crate::game::state::State;

mod components;
use crate::components::compass::Compass;

struct App {
    state: Rc<State>,
    _dispatch: Dispatch<BasicStore<State>>,
}

enum Msg {
    State(Rc<State>),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let current_room = self.state.get_current_room();

        html! {
            <div class="container-fluid p-5">
                <div class="row">
                    <div class="col-8">
                        <h1>{ &current_room.name }</h1>
                    </div>
                    <div class="col-4">
                        <Compass />
                    </div>
                </div>
            </div>
        }
    }
}

pub fn main() {
    yew::start_app::<App>();
}
