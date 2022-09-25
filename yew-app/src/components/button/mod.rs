use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlButtonElement;
use yew::prelude::*;

use crate::utils::style::create_styles;

const STYLE_FILE: &str = include_str!("style.css");

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonKind {
    Primary,
    Secondary,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: &'static str,
    pub button_type: Option<ButtonType>,
    pub kind: Option<ButtonKind>,
    #[prop_or_default]
    pub onclick: Callback<HtmlButtonElement>,
}

fn get_button_type(button_type: &Option<ButtonType>) -> &'static str {
    button_type
        .map(|b| match b {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        })
        .unwrap_or("button")
}

fn get_button_kind(kind: &Option<ButtonKind>) -> &'static str {
    kind.map(|k| match k {
        ButtonKind::Primary => "primary",
        ButtonKind::Secondary => "secondary",
    })
    .unwrap_or("primary")
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let styles = create_styles(vec![STYLE_FILE, "display: inline-block;"]);

    let ButtonProps {
        text,
        kind,
        button_type,
        ..
    } = props;

    let onclick = props.onclick.clone();

    let handle_click = Callback::from(move |e: MouseEvent| {
        let element = e
            .target()
            .and_then(|t| t.dyn_into::<HtmlButtonElement>().ok());
        element.map(|el| onclick.emit(el));
    });

    html! {
        <div class={styles}>
            <button
                class={get_button_kind(kind)}
                type={get_button_type(button_type)}
                onclick={handle_click}
            >
                {text}
            </button>
        </div>
    }
}
