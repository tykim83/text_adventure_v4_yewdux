use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::dispatch::DispatchProps;

mod game;
use crate::game::state::State;

mod components;
use crate::components::compass::MyCompass;

struct App;

impl Component for App {
    type Message = ();
    type Properties = AppDispatch;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = ctx.props().state();
        let current_room = state.get_current_room();
        html! {
            <div class="container-fluid p-5">
                <div class="row">
                    <div class="col-8">
                        <h1>{ &current_room.name }</h1>
                    </div>
                    <div class="col-4">
                        <MyCompass />
                    </div>
                </div>
            </div>
        }
    }
}

type AppDispatch = DispatchProps<ReducerStore<State>>;

fn main() {
    yew::start_app::<WithDispatch<App>>();
}