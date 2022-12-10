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
    pub href: Option<AttrValue>,
    pub id: Option<AttrValue>,
    #[prop_or(ButtonVariant::Outlined)]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or(Size::Medium)]
    pub size: Size,
    #[prop_or(ButtonTag::Button)]
    pub tag: ButtonTag,
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
        tag,
    }: &Props,
) -> Html {
    let disabled = *disabled;
    let is_link = !href.is_none();
    let tag = get_html_tag(tag, is_link);
    let child_list = children.iter().collect::<Html>();

    let styles = get_styles(Styles {
        class,
        color,
        disabled,
        is_link,
        size,
        variant,
        elevation: *elevation,
    });

    let button_type = if tag == "button" {
        Some(button_type.to_string())
    } else {
        None
    };

    let ripple = html! {
        if !is_link && !disabled { <RippleWrapper/>}
    };

    let handle_click = {
        let onclick = onclick.clone();

        Callback::from(move |event| {
            if !disabled {
                onclick.emit(event)
            }
        })
    };

    // let

    html! {
        <@{tag}
            id={id}
            class={styles}
            href={href}
            disabled={disabled}
            onclick={handle_click}
            type={button_type}
        >
            <div>{child_list}</div>
            {ripple}
        </@>
    }
}

/// The html tag for Button component
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonTag {
    Div,
    Span,
    Button,
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

/// button tag
fn get_html_tag(tag: &ButtonTag, is_link: bool) -> String {
    let html_tag = if is_link {
        "a"
    } else {
        match tag {
            ButtonTag::Div => "div",
            ButtonTag::Span => "span",
            ButtonTag::Button => "button",
        }
    };

    String::from(html_tag)
}

// button style
fn get_variant_style(variant: &ButtonVariant, is_link: bool) -> String {
    match is_link {
        true => "link".to_string(),
        false => variant.to_string(),
    }
}

fn get_size_style(size: &Size, variant: &ButtonVariant, is_link: bool) -> String {
    let default = format!("btn-{size}");

    if is_link {
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
    is_link: bool,
    size: &'a Size,
    variant: &'a ButtonVariant,
}

fn get_styles(
    Styles {
        class,
        color,
        disabled,
        elevation,
        is_link,
        size,
        variant,
    }: Styles,
) -> Classes {
    let size_style = get_size_style(size, variant, is_link);
    let variant_style = format!("btn-{}-{color}", get_variant_style(variant, is_link));

    let mut styles = vec!["btn", &size_style, &variant_style];

    if elevation && *variant == ButtonVariant::Circle || *variant == ButtonVariant::Contained {
        styles.push("btn-elevation");
    }

    if disabled {
        styles.push("btn-disabled");
    }

    classes!(prefixes(&styles), class.clone())
}
