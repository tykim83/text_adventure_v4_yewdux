use yew::prelude::*;
use yewdux::prelude::*;
use gloo_console as console;
use std::rc::Rc;

use crate::game::{item::{ButtonType, Item, ItemType}, state::State};

pub struct ItemsComponent {
    state: Rc<State>,
    dispatch: Dispatch<BasicStore<State>>,
}

impl ItemsComponent {
    fn hide_buttons(&self) -> &'static str {
        if self.state.selected_item.is_none() {
            return "d-none"
        }
        "d-flex"
    }
}

pub enum Msg {
    State(Rc<State>),
    SetActive(ItemType),
    ItemAction(ButtonType),
}

impl Component for ItemsComponent {
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
            Msg::SetActive(item) => {
                self.dispatch.reduce(move |s| s.selected_item = Some(item));
                true
            },
            Msg::ItemAction(button_type) => {
                self.dispatch.reduce(move |s| s.action_item(button_type));
                false
            },
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
  
        let item_types = self.state.get_current_room_items();
        // console::log!(format!("compass: {:?}", items));

        html! {
            <div class="container">
                <div class="card">

                    <div class="card-header text-info">
                        { "Items" }
                    </div>

                    <div class="card-body">
                        
                    {
                        for item_types.into_iter().map(|item_type| {
                            html! {
                                <input 
                                    value={ self.state.get_item_name(item_type).to_owned() } 
                                    onclick={ctx.link().callback(move |_| Msg::SetActive(item_type))} 
                                    class="btn btn-outline-secondary m-2" type="button" />
                            }
                        }) 
                    }
                    
                        
                    </div>

                    <div class="card-footer text-muted">
                        <div class={classes!(self.hide_buttons())}>
                            <input 
                                value={ "Look At" }
                                onclick={ctx.link().callback(|_| Msg::ItemAction(ButtonType::Look))}  
                                class="btn btn-primary m-2"
                                type="button" />
                            <input 
                                value={ "Take" }
                                onclick={ctx.link().callback(|_| Msg::ItemAction(ButtonType::Take))}  
                                class="btn btn-primary m-2" 
                                type="button" />
                        </div>
                    </div>

                </div> 
            </div>
        }
    }
}
