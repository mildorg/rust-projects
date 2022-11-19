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

fn render_button_sizes() -> Vec<Html> {
    let sizes = vec![ButtonSize::Small, ButtonSize::Medium, ButtonSize::Large];

    let buttons=  sizes.iter().map(|size| {
        html! {
            <Button key={size.to_string()} size={size.clone()}>{to_first_upper(&size.to_string())}</Button>
        }
    }).collect::<Vec<Html>>();

    let link_buttons =sizes.iter().map(|size| {
        html! {
            <Button href="#" key={size.to_string()} size={size.clone()} >{to_first_upper(&size.to_string())}</Button>
        }
    }).collect::<Vec<Html>>();

    [buttons, link_buttons].concat()
}

#[function_component(ButtonDoc)]
pub(crate) fn button_doc() -> Html {
    let danger = "Danger";
    let primary = "Primary";
    let link = "Link";

    html! {
        <div class="button-doc">
            <div class="section">
                <h2>{"Button kind"}</h2>
                {render_buttons(false)}
            </div>

            <div class="section">
                <h2>{"Sizes"}</h2>
                {render_button_sizes()}
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
