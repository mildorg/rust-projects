use std::fmt::{Display, Formatter, Result};

use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq)]
pub enum ButtonKind {
    Primary,
    Secondary,
}

impl Display for ButtonKind {
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            ButtonKind::Primary => write!(f, "button--primary"),
            ButtonKind::Secondary => write!(f, "button--secondary"),
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: AttrValue,
    #[prop_or(ButtonKind::Primary)]
    pub kind: ButtonKind,
}

#[function_component(Button)]
pub fn button(Props { text, kind }: &Props) -> Html {
    let classes = classes!("button", kind.to_string());

    html! {
        <button class={classes}>{text}</button>
    }
}
