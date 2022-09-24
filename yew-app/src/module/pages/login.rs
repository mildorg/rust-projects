use stylist::yew::styled_component;
use yew::prelude::{html, use_state, Callback};
use yew::UseStateHandle;

use crate::module::components::{Button, ButtonKind, TextInput, Title};
use crate::module::utilities::style::create_style;

const STYLE_FILE: &str = include_str!("./style.css");

fn generate_changer<T: 'static>(state: &UseStateHandle<T>) -> Callback<T> {
    let cloned_state = state.clone();
    Callback::from(move |value| cloned_state.set(value))
}

#[styled_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| "".to_string());
    let password = use_state(|| "".to_string());

    let handle_username_change = generate_changer(&username);
    let handle_password_change = generate_changer(&password);

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
                        <Button text="Login"/>
                        <Button text="Cancel" kind={ButtonKind::Secondary}/>
                    </div>
                </form>
                <div class="form-value">
                    <p>{(*username).clone()}</p>
                    <p>{(*password).clone()}</p>
                </div>
            </div>
        </div>
    }
}
