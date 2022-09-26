use stylist::yew::styled_component;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::utils::style::create_styles;

const STYLE_FILE: &str = include_str!("style.css");

#[derive(Properties, PartialEq)]
pub struct TextInputProps {
    pub name: String,
    #[prop_or_default]
    pub label: String,
    pub onchange: Callback<(String, Event)>,
}

#[styled_component(TextInput)]
pub fn text_input(props: &TextInputProps) -> Html {
    let TextInputProps {
        name,
        label,
        onchange,
    } = props;

    let styles = create_styles(vec![STYLE_FILE, "margin-bottom: 1rem;"]);

    let onchange = onchange.clone();

    let handle_change = Callback::from(move |e: Event| {
        let input: Option<HtmlInputElement> = e.target().and_then(|e| e.dyn_into().ok());
        input.map(|input| onchange.emit((input.value(), e)));
    });

    html! {
        <div class={styles}>
            <label for={name.clone()}> {label}</label>
            <input
                type="text"
                id={name.clone()}
                name={name.clone()}
                onchange={handle_change}
            />
        </div>
    }
}
