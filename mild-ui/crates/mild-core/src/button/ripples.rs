use gloo_timers::callback::Timeout;
use yew::prelude::*;

use crate::utils::web::{get_window, log, timer::set_timeout};

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

            let clear = set_timeout(*timeout, move || {
                log::info("timer_id");
                on_exited.emit(())
            });

            move || {
                clear();
            }
        },
        (leaving, *timeout, on_exited.clone()),
    );

    html! {
        <span style={get_style(*ripple_size, *ripple_x, *ripple_y)}>
            <span></span>
        </span>
    }
}

fn get_style(ripple_size: f32, ripple_x: f32, ripple_y: f32) -> String {
    let top = -(ripple_size / 2.0) + ripple_y;
    let left = -(ripple_size / 2.0) + ripple_x;
    format!("width: {ripple_size}px; height: {ripple_x}px; top: {top}px; left: {left}px;")
}
