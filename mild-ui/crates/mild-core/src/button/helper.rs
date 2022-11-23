use std::fmt::{Display, Formatter, Result};
use yew::prelude::*;

use crate::styles::{style_prefix, Color, Size};

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
    let prefix = |s: &str| style_prefix("btn-", s);

    let variant = get_variant(variant, href);
    let variant_color = prefix(&format!("{variant}-{color}"));
    let size = prefix(&size.to_string());

    let mut class_list = classes!("btn", variant_color, size, class.clone());

    if disabled {
        class_list.push("disabled");
    }

    class_list
}
