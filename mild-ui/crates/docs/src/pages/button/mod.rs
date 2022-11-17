use yew::prelude::*;

use mild_core::button::{Button, ButtonKind, ButtonSize};

use crate::utils::string::to_first_upper;

fn render_buttons(disabled: bool) -> Vec<Html> {
    let kinds = vec![ButtonKind::Primary, ButtonKind::Default, ButtonKind::Text];

    let link_button = html! { <Button key={"link"} {disabled} href="#">{"Link"}</Button>};

    let mut buttons: Vec<Html> = kinds
        .into_iter()
        .map(|kind| {
            html! {
                <Button
                    id={kind.to_string()}
                    key={kind.to_string()}
                    kind={kind.clone()}
                    disabled={disabled}
                >
                    {to_first_upper(&kind.to_string())}
                </Button>
            }
        })
        .collect();

    buttons.push(link_button);

    buttons
}

#[function_component(ButtonDoc)]
pub(crate) fn button_doc() -> Html {
    let danger = "Danger";
    let default = "Default";
    let primary = "Primary";
    let link = "Link";
    let small = "Small";
    let large = "Lager";

    html! {
        <div class="button-doc">
            <div class="section">
                <h2>{"Button kind"}</h2>
                {render_buttons(false)}
            </div>

            <div class="section">
                <h2>{"Sizes"}</h2>
                <Button size={ButtonSize::Sm}>{small}</Button>
                <Button >{default}</Button>
                <Button size={ButtonSize::Lg} >{large}</Button>
                <Button size={ButtonSize::Sm}  href="#">{small}</Button>
                <Button href="#">{default}</Button>
                <Button size={ButtonSize::Lg}  href="#">{large}</Button>
            </div>

            <div class="section">
                <h2>{"Disabled state"}</h2>
                {render_buttons(true)}
            </div>

            <div class="section">
                <h2>{"Danger state"}</h2>
                <Button danger={true}>{danger}</Button>
                <Button href="#" danger={true}>{link}</Button>
            </div>

            <div class="section">
                <h2>{"Event"}</h2>
                <Button kind={ButtonKind::Primary}>{primary}</Button>
            </div>
        </div>
    }
}
