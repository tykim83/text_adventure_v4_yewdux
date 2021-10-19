// use yew::prelude::*;
// use yewdux::prelude::*;

// use crate::state::{Map, Action};


// pub struct Compass;

// impl Component for Compass {
//     type Message = ();
//     type Properties = AppDispatch;

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self
//     }

//     fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let rooms = &ctx.props().state().rooms;
//         // let increment = ctx.props().callback(|_| Action::);
//         html! {
//             <div> { "ciao"}
//                 // {
//                 // for (1..10).map(|item| {
//                 //     html_nested! {
//                 //         <div>{ "ciao" }</div>
//                 //     }
//                 // })
//                 // }
//             </div>
//         }
//         // html! {
//         //     <>
//         //         {
//         //         for rooms.iter().map(|room| {
//         //             html_nested! {
//         //                 <div>{ room }</div>
//         //             }
//         //         })
//         //         }
//         //     </>
//         // }
//     }
// }

// type AppDispatch = DispatchProps<ReducerStore<Map>>;
