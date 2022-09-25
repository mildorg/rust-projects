pub mod components;
pub mod pages;
pub mod utils;

use stylist::yew::styled_component;
use yew::prelude::*;

use components::structs::Theme;
use pages::Login;
use utils::style::create_style;

const STYLE_FILE: &str = include_str!("./main.css");

#[styled_component(App)]
pub fn app() -> Html {
    let style = create_style(STYLE_FILE);

    let theme_state = use_state(Theme::default);

    html! {
        <ContextProvider<UseStateHandle<Theme>> context={theme_state}>
            <div class={style}>
                <Login />
            </div>
        </ContextProvider<UseStateHandle<Theme>>>
    }
}
