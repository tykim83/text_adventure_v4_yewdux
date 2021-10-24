use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console as console;
use std::rc::Rc;

use crate::game::{item::Item, state::State};

pub struct ItemsComponent {
    state: Rc<State>,
    _dispatch: Dispatch<BasicStore<State>>,
    selected_item: Option<Item>
}

impl ItemsComponent {
    fn hide_buttons(&self) -> &'static str {
        if self.selected_item.is_none() {
            return "d-none"
        }
        "d-flex"
    }
}

pub enum Msg {
    State(Rc<State>),
    SetActive(Item),
}

impl Component for ItemsComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            _dispatch: Dispatch::bridge_state(ctx.link().callback(Msg::State)),
            state: Default::default(),
            selected_item: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::State(state) => {
                self.state = state;
                self.selected_item = None;
                true
            },
            Msg::SetActive(item) => {
                self.selected_item = Some(item);
                true
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
  
        let items = self.state.get_current_room_items();
        // console::log!(format!("compass: {:?}", items));

        html! {
            <div class="container">
                <div class="card">

                    <div class="card-header text-info">
                        { "Items" }
                    </div>

                    <div class="card-body">
                        
                    {
                        for items.into_iter().map(|item| {
                            
                            html! {
                                <input 
                                    value={item.name.clone()} 
                                    onclick={ctx.link().callback(move |_| Msg::SetActive(item.clone()))} 
                                    class="btn btn-outline-secondary m-2" type="button" />
                            }
                        }) 
                    }
                    
                        
                    </div>

                    <div class="card-footer text-muted">
                        <div class={classes!(self.hide_buttons())}>
                            <button onclick={|_| console::log!("click")} class="btn btn-primary">{ "Look At" }</button>
                            <button onclick={|_| console::log!("click")} class="btn btn-primary">{ "Take" }</button>
                        </div>
                    </div>

                </div> 
            </div>
        }
    }
}
