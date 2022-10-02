use yew::prelude::*;
use yewdux::prelude::*;

use crate::components::{Button, ButtonKind};

#[derive(Clone, PartialEq, Store)]
pub struct HomeState {
    pub message: String,
    pub count: u32,
}

impl Default for HomeState {
    fn default() -> Self {
        Self {
            message: "Home page".to_string(),
            count: 0,
        }
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let (state, dispatch) = use_store::<HomeState>();
    let message = state.message.clone();

    let handle_click = dispatch.reduce_callback(|state| HomeState {
        count: state.count + 1,
        message: state.message.clone(),
    });

    html! {
        <div>
            <p>{message}</p>
            <Button text="plus one" kind={ButtonKind::Secondary} onclick={handle_click}/>
        </div>
    }
}
