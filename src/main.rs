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
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let current_location = &ctx.props().state().current_location;
        let current_room = ctx.props().state().map.get_current_room(current_location);
        html! {
            <>
            <h1>{ &current_room.name }</h1>
            <MyCompass />
            </>
        }
    }
}

type AppDispatch = DispatchProps<ReducerStore<State>>;

fn main() {
    yew::start_app::<WithDispatch<App>>();
}