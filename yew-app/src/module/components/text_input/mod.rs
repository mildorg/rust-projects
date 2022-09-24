use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::{html, Callback, Event, Properties};

use crate::module::utilities::style::create_styles;

const STYLE_FILE: &str = include_str!("./style.css");

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: &'static str,
    #[prop_or_default]
    pub label: &'static str,
    pub onchange: Callback<String>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let name = props.name;
    let onchange = props.onchange.clone();

    let styles = create_styles(vec![STYLE_FILE, "margin-bottom: 1rem;"]);

    let handle_change = Callback::from(move |e: Event| {
        let input: Option<HtmlInputElement> = e.target().and_then(|e| e.dyn_into().ok());
        input.map(|input| onchange.emit(input.value()));
    });

    html! {
        <div class={styles}>
            <label for={name}> {props.label}</label>
            <input
                type="text"
                id={name}
                name={name}
                onchange={handle_change}
            />
        </div>
    }
}
