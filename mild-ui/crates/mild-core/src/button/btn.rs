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
    /// If true, enable circle and contained button elevation
    #[prop_or(true)]
    pub elevation: bool,
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
        elevation,
        href,
        id,
        variant,
        onclick,
        size,
    }: &Props,
) -> Html {
    let styles = get_styles(Styles {
        class,
        color,
        href,
        size,
        variant,
        disabled: *disabled,
        elevation: *elevation,
    });

    let child_list = children.iter().collect::<Html>();

    let handle_click = {
        let onclick = onclick.clone();
        Callback::from(move |event| onclick.emit(event))
    };

    if is_link(href) {
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

// is link button
fn is_link(href: &str) -> bool {
    !href.is_empty()
}

// button style
fn get_variant_style(variant: &ButtonVariant, href: &str) -> String {
    match is_link(href) {
        true => "link".to_string(),
        false => variant.to_string(),
    }
}

fn get_size_style(size: &Size, variant: &ButtonVariant, href: &str) -> String {
    let default = format!("btn-{size}");

    if is_link(href) {
        return default;
    }

    match variant {
        ButtonVariant::Circle => format!("btn-{}-{}", variant, size),
        ButtonVariant::Outlined => format!("btn-{}-{}", variant, size),
        _ => default,
    }
}

struct Styles<'a> {
    class: &'a Classes,
    color: &'a Color,
    disabled: bool,
    elevation: bool,
    href: &'a str,
    size: &'a Size,
    variant: &'a ButtonVariant,
}

fn get_styles(
    Styles {
        class,
        color,
        disabled,
        elevation,
        href,
        size,
        variant,
    }: Styles,
) -> Classes {
    let size_style = get_size_style(size, variant, href);
    let variant_style = format!("btn-{}-{color}", get_variant_style(variant, href));

    let mut styles = vec!["btn", &size_style, &variant_style];

    if elevation && *variant == ButtonVariant::Circle || *variant == ButtonVariant::Contained {
        styles.push("btn-elevation");
    }

    if disabled {
        styles.push("btn-disabled");
    }

    classes!(prefixes(&styles), class.clone())
}
