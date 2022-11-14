use std::fmt::{Display, Formatter, Result};

use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Eq, Clone)]
pub enum ButtonKind {
    Link,
    Danger,
    Default,
    Info,
    Primary,
    Secondary,
    Success,
    Warning,
}

impl Display for ButtonKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let kind = match self {
            ButtonKind::Link => "link",
            ButtonKind::Danger => "danger",
            ButtonKind::Default => "default",
            ButtonKind::Info => "info",
            ButtonKind::Primary => "primary",
            ButtonKind::Secondary => "secondary",
            ButtonKind::Success => "success",
            ButtonKind::Warning => "warning",
        };
        write!(f, "{}", kind)
    }
}

#[derive(PartialEq, Eq)]
pub enum ButtonSize {
    Sm,
    Lg,
}

fn prefix(s: &str) -> String {
    format!("btn-{}", s)
}

fn get_classes(
    kind: &ButtonKind,
    size: &Option<ButtonSize>,
    class: &Classes,
    disabled: bool,
) -> Classes {
    let kind_class = prefix(kind.to_string().as_str());

    let size_class = size.as_ref().map(|item| match item {
        ButtonSize::Sm => prefix("sm"),
        ButtonSize::Lg => prefix("lg"),
    });

    let mut class_list = classes!("btn", kind_class, size_class, class.clone());

    if disabled {
        class_list.push("disabled");
    }

    class_list
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or(ButtonKind::Default)]
    pub kind: ButtonKind,
    #[prop_or_default]
    pub class: Classes,
    pub size: Option<ButtonSize>,
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(Button)]
pub fn button(Props { children, kind, class, size, href, disabled }: &Props) -> Html {
    let class_list = get_classes(kind, size, class, *disabled);
    let child_list = children.iter().collect::<Html>();

    let handle_click = Callback::from(|_| {
        gloo_console::log!("click");
    });

    if *kind == ButtonKind::Link && href.is_some() {
        return html! {
            <a
                class={class_list}
                href={href.as_ref().unwrap().clone()}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            class={class_list}
            disabled={*disabled}
            onclick={handle_click}
        >
            {child_list}
        </button>
    }
}
