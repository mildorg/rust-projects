use stylist::yew::styled_component;
use yew::prelude::{html, Properties};

use crate::module::utilities::style::create_styles;

const STYLE_FILE: &str = include_str!("./style.css");

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

    html! {
        <div class={styles}>
            <button
                class={get_button_kind(&props.kind)}
                type={get_button_type(&props.button_type)}
            >
                {props.text}
            </button>
        </div>
    }
}
