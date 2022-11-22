use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::utils::web::{console_log, get_window};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub on_exited: Callback<()>,
    #[prop_or(false)]
    pub pulsate: bool,
    #[prop_or_default]
    pub ripple_size: f32,
    #[prop_or_default]
    pub ripple_x: f32,
    #[prop_or_default]
    pub ripple_y: f32,
    #[prop_or_default]
    pub timeout: u32,
}

#[function_component(Ripple)]
pub fn ripple(
    Props {
        on_exited,
        pulsate,
        ripple_size,
        ripple_x,
        ripple_y,
        timeout,
    }: &Props,
) -> Html {
    let leaving = use_state_eq(|| false);

    use_effect_with_deps(
        move |(leaving, timeout, on_exited)| {
            leaving.set(true);

            let on_exited = on_exited.clone();

            let timer_id = Timeout::new(2000, move || {
                console_log("timer_id");
                on_exited.emit(())
            })
            .forget();

            move || {
                get_window().clear_timeout_with_handle(timer_id);
            }
        },
        (leaving, *timeout, on_exited.clone()),
    );

    html! {
        <span>
            <span></span>
        </span>
    }
}
