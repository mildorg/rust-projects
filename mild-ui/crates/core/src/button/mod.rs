use std::fmt::{Display, Formatter, Result};
use std::mem;
use std::rc::Rc;

use gloo_console::log;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(PartialEq, Eq)]
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

#[derive(Properties, PartialEq, Eq)]
pub struct Props {
    pub text: AttrValue,
    #[prop_or(ButtonKind::Primary)]
    pub kind: ButtonKind,
}

struct Unit {}

impl yew::Reducible for Unit {
    type Action = u32;

    fn reduce(self: Rc<Self>, _action: Self::Action) -> Rc<Self> {
        todo!()
    }
}

#[function_component(Button)]
pub fn button(Props { text, kind }: &Props) -> Html {
    let classes = classes!("button", kind.to_string());
    let state = use_state(|| ButtonKind::Primary);

    log!("Use_state -> ", mem::size_of_val(&state));
    log!("Rc Use_state -> ", mem::size_of::<Rc<ButtonKind>>());
    log!("ButtonKind -> ", mem::size_of::<ButtonKind>());
    log!("Props -> ", mem::size_of::<Props>());
    log!("UseReducerHandle -> ", mem::size_of::<yew::UseReducerHandle<Unit>>());
    log!("Vec -> ", mem::size_of_val(&vec!["abc".to_owned(), "efg".to_owned()]));

    html! {
        <button class={classes}>{text}</button>
    }
}

#[function_component(Test)]
pub fn test() -> Html {
    html! {
        <div></div>
    }
}
