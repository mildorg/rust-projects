use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

use super::ripple_wrapper::RippleWrapper;
use crate::styles::{prefixes, Color, Size};

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(ButtonType::Button)]
    pub button_type: ButtonType,
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or(Color::Primary)]
    pub color: Color,
    #[prop_or_default]
    pub disabled: bool,
    /// If pass href prop, display a link button
    #[prop_or_default]
    pub href: AttrValue,
    pub id: Option<AttrValue>,
    #[prop_or(ButtonVariant::Outlined)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Size::Medium)]
    pub size: Size,
}

#[function_component]
pub fn Button(
    Props {
        button_type,
        children,
        class,
        color,
        disabled,
        href,
        id,
        variant,
        onclick,
        size,
    }: &Props,
) -> Html {
    let styles = get_styles(variant, color, size, href, *disabled, class);

    let child_list = children.iter().collect::<Html>();

    let handle_click = {
        let onclick = onclick.clone();
        Callback::from(move |event| onclick.emit(event))
    };

    if !href.is_empty() {
        return html! {
            <a
                id={id}
                class={styles}
                href={href}
                onclick={handle_click}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            id={id}
            class={styles}
            disabled={*disabled}
            onclick={handle_click}
            type={button_type.to_string()}
        >
            <div>{child_list}</div>
            <RippleWrapper />
        </button>
    }
}

/// The html button type
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonType {
    Button,
    Reset,
    Submit,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let btn_type = match self {
            ButtonType::Button => "button",
            ButtonType::Reset => "reset",
            ButtonType::Submit => "submit",
        };

        write!(f, "{}", btn_type)
    }
}

/// The kind for button
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonVariant {
    Circle,
    Contained,
    Outlined,
    Text,
}

impl Display for ButtonVariant {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let kind = match self {
            ButtonVariant::Text => "text",
            ButtonVariant::Circle => "circle",
            ButtonVariant::Contained => "contained",
            ButtonVariant::Outlined => "outlined",
        };
        write!(f, "{}", kind)
    }
}

// button style
fn get_variant(variant: &ButtonVariant, href: &str) -> String {
    match href.is_empty() {
        true => variant.to_string(),
        false => "link".to_string(),
    }
}

pub fn get_styles(
    variant: &ButtonVariant,
    color: &Color,
    size: &Size,
    href: &str,
    disabled: bool,
    class: &Classes,
) -> Classes {
    let variant_style = format!("btn-{}-{color}", get_variant(variant, href));
    let mut styles = vec!["btn", &variant_style];

    let size_style = if *variant == ButtonVariant::Outlined && href.is_empty() {
        format!("btn-outlined-{size}")
    } else {
        format!("btn-{size}")
    };

    styles.push(&size_style);

    if disabled {
        styles.push("btn-disabled");
    }

    classes!(prefixes(&styles), class.clone())
}
