use yew::prelude::*;

use crate::{styles::style_prefix, utils::web::timer::set_timeout};

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

#[function_component]
pub fn Ripple(
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
            let clear = set_timeout(*timeout, move || on_exited.emit(()));

            move || {
                clear();
            }
        },
        (leaving.clone(), *timeout, on_exited.clone()),
    );

    html! {
        <span
            class={get_ripple_class(*pulsate)}
            style={get_ripple_style(*ripple_size, *ripple_x, *ripple_y)}
        >
            <span class={get_child_class(*leaving, *pulsate)}/>
        </span>
    }
}

fn get_ripple_style(ripple_size: f32, ripple_x: f32, ripple_y: f32) -> String {
    let top = -(ripple_size / 2.0) + ripple_y;
    let left = -(ripple_size / 2.0) + ripple_x;
    format!("width: {ripple_size}px; height: {ripple_x}px; top: {top}px; left: {left}px;")
}

fn get_ripple_class(pulsate: bool) -> Classes {
    let prefix = |s: &str| style_prefix("ripple", s);
    let mut ripple_class = classes!(prefix(""), prefix("-visible"));

    if pulsate {
        ripple_class.push(prefix("-pulsate"));
    }

    ripple_class
}

fn get_child_class(leaving: bool, pulsate: bool) -> Classes {
    let prefix = |s: &str| style_prefix("ripple_child", s);
    let mut child_class = classes!(prefix(""));

    if leaving {
        child_class.push(prefix("-leaving"));
    }

    if pulsate {
        child_class.push(prefix("-pulsate"));
    }

    child_class
}
