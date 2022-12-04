use std::fmt::{Display, Formatter, Result};

/// The mild ui css prefix
const STYLE_PREFIX: &str = "";

pub fn prefix(str: &str) -> String {
    format!("{STYLE_PREFIX}{str}")
}

pub fn prefixes(vec: &[&str]) -> Vec<String> {
    vec.iter().map(|str| prefix(str)).collect()
}

/// The mild ui size
#[derive(PartialEq, Eq, Clone)]
pub enum Size {
    Small,
    Medium,
    Large,
}

impl Display for Size {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let size = match self {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
        };

        write!(f, "{}", size)
    }
}

/// The mild ui color
#[derive(PartialEq, Eq, Clone)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Error,
}

impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let color = match self {
            Color::Primary => "primary",
            Color::Secondary => "secondary",
            Color::Success => "success",
            Color::Info => "info",
            Color::Warning => "warning",
            Color::Error => "error",
        };

        write!(f, "{}", color)
    }
}
