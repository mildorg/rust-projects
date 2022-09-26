use stylist::yew::styled_component;
use yew::prelude::*;

use crate::utils::style::create_styles;

const STYLE_FILE: &str = include_str!("style.css");

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum ButtonType {
    Button,
    Submit,
    Reset,
}

#[allow(unused)]
#[derive(PartialEq, Eq)]
pub enum ButtonKind {
    Link,
    Primary,
    Secondary,
}

#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
    /// button type: `button`,`submit` and `rest`. Default is `button`.
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    /// button kind: `primary`, `secondary` and `link`. Default is `primary`,
    #[prop_or(ButtonKind::Primary)]
    pub kind: ButtonKind,

    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    let ButtonProps {
        text,
        kind,
        button_type,
        onclick,
    } = props;

    let styles = create_styles(vec![STYLE_FILE, "display: inline-block;"]);

    let btn_type = match button_type {
        ButtonType::Button => "button",
        ButtonType::Submit => "submit",
        ButtonType::Reset => "reset",
    };

    let btn_class = match kind {
        ButtonKind::Primary => "primary",
        ButtonKind::Secondary => "secondary",
        ButtonKind::Link => "link",
    };

    if ButtonKind::Link == *kind {
        return html! {
            <div class={styles}>
                <a class={btn_class} onclick={onclick}>{text}</a>
            </div>
        };
    }

    html! {
        <div class={styles}>
            <button class={btn_class} type={btn_type} onclick={onclick} >
                {text}
            </button>
        </div>
    }
}
