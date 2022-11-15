use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub enum ButtonKind {
    Danger,
    Text,
    Primary,
    Secondary,
}

impl Display for ButtonKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let kind = match self {
            ButtonKind::Danger => "danger",
            ButtonKind::Text => "text",
            ButtonKind::Primary => "primary",
            ButtonKind::Secondary => "secondary",
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
    href: &str,
    disabled: bool,
    class: &Classes,
) -> Classes {
    let kind_class =
        if href.is_empty() { prefix(kind.to_string().as_str()) } else { prefix("link") };

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
    #[prop_or(ButtonKind::Primary)]
    pub kind: ButtonKind,
    pub size: Option<ButtonSize>,
    /// If pass href prop, display a link button
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub class: Classes,
    pub id: Option<String>,
}

#[function_component(Button)]
pub fn button(Props { children, kind, class, size, href, disabled, id }: &Props) -> Html {
    let class_list = get_classes(kind, size, href, *disabled, class);
    let child_list = children.iter().collect::<Html>();

    let handle_click = Callback::from(|_| {
        gloo_console::log!("click");
    });

    if !href.is_empty() {
        return html! {
            <a
                id={id.clone()}
                class={class_list}
                href={href.clone()}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            id={id.clone()}
            class={class_list}
            disabled={*disabled}
            onclick={handle_click}
        >
            {child_list}
        </button>
    }
}
