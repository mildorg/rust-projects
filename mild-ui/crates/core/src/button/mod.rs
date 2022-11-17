use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

#[derive(PartialEq, Eq, Clone)]
pub enum ButtonKind {
    Default,
    Text,
    Primary,
}

impl Display for ButtonKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let kind = match self {
            ButtonKind::Default => "default",
            ButtonKind::Text => "text",
            ButtonKind::Primary => "primary",
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

fn get_kind_style(kind: &ButtonKind, href: &str, danger: bool) -> String {
    match (href.is_empty(), danger) {
        (true, true) => prefix("danger"),
        (true, false) => prefix(kind.to_string().as_str()),
        (false, true) => prefix("danger-link"),
        (false, false) => prefix("link"),
    }
}

fn get_size_style(size: &Option<ButtonSize>) -> Option<String> {
    size.as_ref().map(|item| match item {
        ButtonSize::Sm => prefix("sm"),
        ButtonSize::Lg => prefix("lg"),
    })
}

fn get_styles(
    kind_style: String,
    size_style: Option<String>,
    class: &Classes,
    disabled: bool,
) -> Classes {
    let mut class_list = classes!("btn", kind_style, size_style, class.clone());

    if disabled {
        class_list.push("disabled");
    }

    class_list
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub danger: bool,
    #[prop_or_default]
    pub disabled: bool,
    /// If pass href prop, display a link button
    #[prop_or_default]
    pub href: String,
    pub id: Option<String>,
    #[prop_or(ButtonKind::Default)]
    pub kind: ButtonKind,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    pub size: Option<ButtonSize>,
}

#[function_component(Button)]
pub fn button(
    Props { children, class, danger, disabled, href, id, kind, onclick, size }: &Props,
) -> Html {
    let kind_style = get_kind_style(kind, href, *danger);
    let size_style = get_size_style(size);
    let styles = get_styles(kind_style, size_style, class, *disabled);

    let child_list = children.iter().collect::<Html>();

    let handle_click = {
        let onclick = onclick.clone();
        Callback::from(move |event| onclick.emit(event))
    };

    if !href.is_empty() {
        return html! {
            <a
                id={id.clone()}
                class={styles}
                href={href.clone()}
            >
                {child_list}
            </a>
        };
    }

    html! {
        <button
            id={id.clone()}
            class={styles}
            disabled={*disabled}
            onclick={handle_click}
        >
            {child_list}
        </button>
    }
}
