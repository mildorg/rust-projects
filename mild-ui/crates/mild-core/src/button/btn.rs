use std::fmt::{Display, Formatter, Result};
use yew::{prelude::*, virtual_dom::VNode};

use super::use_ripple::{use_ripple, UseRipple};
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
    #[prop_or_default]
    pub onkeydown: Callback<KeyboardEvent>,
    #[prop_or(Size::Medium)]
    pub size: Size,
    #[prop_or(AttrValue::from("button"))]
    pub tag: AttrValue,
}

#[function_component]
pub fn Button(props: &Props) -> Html {
    let Props {
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
        onkeydown,
        size,
        tag,
    } = props;

    let UseRipple {
        ripple_wrapper,
        focus_start,
        blur_stop,
        mouse_start,
        mouse_stop,
    } = use_ripple(false, 500, None);

    let disabled = *disabled;
    let is_link = !href.is_none();
    let tag = get_html_tag(tag, is_link);
    let tabindex = if disabled { -1 } else { 1 };
    let child_list = children.iter().collect::<Html>();

    let classes = get_classes(PClass {
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

    let ripple = if !is_link && !disabled {
        ripple_wrapper
    } else {
        VNode::default()
    };

    let handle_click = {
        let onclick = onclick.clone();

        Callback::from(move |event| {
            if !disabled {
                onclick.emit(event);
            }
        })
    };

    let handle_keydown = {
        let onkeydown = onkeydown.clone();

        Callback::from(move |event| {
            if !disabled {
                onkeydown.emit(event);
            }
        })
    };

    html! {
        <@{tag}
            id={id}
            class={classes}
            href={href}
            disabled={disabled}
            onclick={handle_click}
            onkeydown={handle_keydown}
            onfocus={focus_start}
            onmousedown={mouse_start}
            onblur={blur_stop}
            onmouseup={mouse_stop.clone()}
            onmouseleave={mouse_stop}
            tabindex={tabindex.to_string()}
            type={button_type}
        >
            <div>{child_list}</div>
            {ripple}
        </@>
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

/// button tag
fn get_html_tag(tag: &AttrValue, is_link: bool) -> String {
    if is_link {
        String::from("a")
    } else {
        tag.to_string()
    }
}

// button style
fn get_variant_style(variant: &ButtonVariant, is_link: bool) -> String {
    let style = match is_link {
        true => "link".to_string(),
        false => variant.to_string(),
    };

    format!("btn-{style}")
}

fn get_size_style(size: &Size, variant: &ButtonVariant) -> String {
    match variant {
        ButtonVariant::Circle => format!("btn-{}-{}", variant, size),
        _ => format!("btn-{size}"),
    }
}

struct PClass<'a> {
    class: &'a Classes,
    color: &'a Color,
    disabled: bool,
    elevation: bool,
    is_link: bool,
    size: &'a Size,
    variant: &'a ButtonVariant,
}

fn get_classes(params: PClass) -> Classes {
    let PClass {
        class,
        color,
        disabled,
        elevation,
        is_link,
        size,
        variant,
    } = params;

    let color = &format!("btn-{color}");
    let size_style = get_size_style(size, variant);
    let variant_style = get_variant_style(variant, is_link);

    let mut styles = vec!["btn", &variant_style, color, &size_style];

    if elevation && *variant == ButtonVariant::Circle || *variant == ButtonVariant::Contained {
        styles.push("btn-elevation");
    }

    if disabled {
        styles.push("btn-disabled");
    }

    classes!(prefixes(&styles), class.clone())
}
