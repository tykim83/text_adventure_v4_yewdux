use yew::prelude::*;
use yewdux::prelude::*;
use yewdux::dispatch::DispatchProps;

use crate::game::state::State;

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
        let map = &ctx.props().state().map;

        html! {
            <>
                {
                for map.rooms.iter().map(|(_location, room)| {
                    html_nested! {
                        <div>{ &room.name }</div>
                        // <button onclick={ctx.props().callback(move |_| Action::GoTo(index))}>{ room. }</button>
                    }})
                }
            </>
        }
    }
}
