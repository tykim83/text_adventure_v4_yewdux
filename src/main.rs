use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::dispatch::DispatchProps;

mod state;
use crate::state::{Map, Action};

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
        let current_room = &ctx.props().state().current_room;
        html! {
            <>
            <h1>{ current_room }</h1>
            <MyCompass />
            </>
        }
    }
}

struct Compass;

type MyCompass = WithDispatch<Compass>;

impl Component for Compass {
    type Message = ();
    type Properties = DispatchProps<ReducerStore<Map>>; 

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let rooms = &ctx.props().state().rooms;

        html! {
            <>
                {
                for rooms.iter().enumerate().map(|(index, room)| {
                    html_nested! {
                        <button onclick={ctx.props().callback(move |_| Action::GoTo(index))}>{ room }</button>

                    }})
                }
            </>
        }
    }
}

type AppDispatch = DispatchProps<ReducerStore<Map>>;

fn main() {
    yew::start_app::<WithDispatch<App>>();
}