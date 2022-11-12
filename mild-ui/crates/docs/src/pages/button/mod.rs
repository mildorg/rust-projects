use yew::prelude::*;

use mild_core::button::{Button, ButtonKind};

#[function_component(ButtonDoc)]
pub(crate) fn button_doc() -> Html {
    html! {
        <Button text="button" kind={ButtonKind::Primary}/>
    }
}
