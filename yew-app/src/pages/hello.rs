use yew::prelude::*;
use yewdux::prelude::*;

use super::home::HomeState;
use crate::components::{Button, ButtonKind};

#[function_component(Hello)]
pub fn hello() -> Html {
    let (state, dispatch) = use_store::<HomeState>();
    let count = state.count;

    let handle_click = dispatch.reduce_callback(|state| HomeState {
        message: "Hello world".to_string(),
        ..*state
    });

    html! {
        <div>
            <p>{count}</p>
            <Button text="update message" kind={ButtonKind::Secondary} onclick={handle_click}/>
        </div>
    }
}
