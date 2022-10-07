use yew::prelude::*;
use yew::start_app;

use mild_core::button::{Button, ButtonKind};

#[function_component(Hello)]
fn hello() -> Html {
    html! {
        <Button text="button" kind={ButtonKind::Secondary}/>
    }
}

fn main() {
    start_app::<Hello>();
}
