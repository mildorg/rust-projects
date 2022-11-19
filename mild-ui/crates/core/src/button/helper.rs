use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

/// The html button type
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonType {
    Button,
    Reset,
    Submit,
}

impl Display for ButtonType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

/// The size for button
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

impl Display for ButtonSize {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let size = match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        };

        write!(f, "{}", size)
    }
}

/// The color for button
#[derive(PartialEq, Eq, Clone)]
pub enum ButtonColor {
    Default,
    Inherit,
    Primary,
    Secondary,
}

impl Display for ButtonColor {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let color = match self {
            ButtonColor::Default => "default",
            ButtonColor::Inherit => "inherit",
            ButtonColor::Primary => "primary",
            ButtonColor::Secondary => "secondary",
        };

        write!(f, "{}", color)
    }
}

// button style
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

pub fn get_styles(
    kind: &ButtonKind,
    size: &ButtonSize,
    href: &str,
    danger: bool,
    disabled: bool,
    class: &Classes,
) -> Classes {
    let kind_style = get_kind_style(kind, href, danger);
    let size_style = prefix(&size.to_string());

    let mut class_list = classes!("btn", kind_style, size_style, class.clone());

    if disabled {
        class_list.push("disabled");
    }

    class_list
}
