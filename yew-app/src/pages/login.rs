use std::ops::Deref;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::UseStateHandle;
use yew_router::prelude::*;

use crate::components::{structs::Theme, Button, ButtonKind, TextInput, Title};
use crate::router::Route;
use crate::utils::style::create_style;

const STYLE_FILE: &str = include_str!("style.css");

#[derive(Default, Clone)]
struct FormData {
    username: String,
    password: String,
}

fn generate_changer<F>(state: &UseStateHandle<FormData>, f: F) -> Callback<(String, Event)>
where
    F: Fn(&UseStateHandle<FormData>, String) -> FormData + 'static,
{
    let cloned_state = state.clone();

    Callback::from(move |(value, _event)| {
        let data = f(&cloned_state, value);
        cloned_state.set(data);
    })
}

#[styled_component(Login)]
pub fn login() -> Html {
    let state = use_state(|| FormData::default());
    let theme_context = use_context::<UseStateHandle<Theme>>().unwrap();
    let history = use_history().unwrap();

    let handle_username_change = generate_changer(&state, move |cloned_state, username| FormData {
        username,
        ..cloned_state.deref().clone()
    });

    let handle_password_change = generate_changer(&state, move |cloned_state, password| FormData {
        password,
        ..cloned_state.deref().clone()
    });

    let handle_submit = {
        let submit_state = state.clone();
        let theme_context = theme_context.clone();

        Callback::from(move |_| {
            let theme = Theme {
                light: submit_state.username.to_string(),
                dark: submit_state.password.to_string(),
            };
            theme_context.set(theme)
        })
    };

    let go_home = Callback::from(move |_| history.push(Route::Home));

    html! {
        <div class={create_style(STYLE_FILE)}>
            <div class="form-container">
                <form>
                    <Title title="Login"/>
                    <div>
                        <TextInput name="username" label="Username:" onchange={handle_username_change}/>
                        <TextInput name="password" label="Password:" onchange={handle_password_change}/>
                    </div>
                    <div class="button-container">
                        <Button text="Login" onclick={handle_submit}/>
                        <Button text="Cancel" kind={ButtonKind::Secondary}/>
                    </div>
                    <div class="button-container">
                        <Button text="Go Home" kind={ButtonKind::Link} onclick={go_home} />
                        <Link<Route> to={Route::Hello}>{"To Hello"}</Link<Route>>
                    </div>
                </form>
                <div class="form-value">
                    <p>{"Form data:"}</p>
                    <p>{&state.username}</p>
                    <p>{&state.password}</p>
                    <br/>
                    <p>{"Form theme:"}</p>
                    <p>{&theme_context.light}</p>
                    <p>{&theme_context.dark}</p>
                </div>
            </div>
        </div>
    }
}
