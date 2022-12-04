use yew::prelude::*;

use crate::styles::prefix;
use crate::utils::web::timer::set_timeout;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub ripple_size: f64,
    #[prop_or_default]
    pub ripple_x: f64,
    #[prop_or_default]
    pub ripple_y: f64,
    #[prop_or_default]
    pub timeout: u32,
}

#[function_component]
pub fn Ripple(
    Props {
        ripple_size,
        ripple_x,
        ripple_y,
        timeout,
    }: &Props,
) -> Html {
    let exiting = use_state_eq(|| false);
    let item_style = get_item_style(*ripple_size, *ripple_x, *ripple_y);

    use_effect_with_deps(
        move |(exiting, timeout)| {
            let exiting = exiting.clone();
            let clear = set_timeout(*timeout, move || exiting.set(true));

            move || {
                clear();
            }
        },
        (exiting.clone(), *timeout),
    );

    html! {
        <span class={get_class(*exiting)} >
            <span
                style={item_style}
                class={get_item_class(*exiting)}
            />
        </span>
    }
}

fn get_class(exiting: bool) -> Classes {
    let mut ripple_class = classes!(prefix("ripple"));

    if exiting {
        ripple_class.push(prefix("ripple-exiting"))
    }

    ripple_class
}

fn get_item_class(exiting: bool) -> Classes {
    let mut item_class = classes!(prefix("ripple_item"));

    if !exiting {
        item_class.push(prefix("ripple_item-entering"));
    }

    item_class
}

fn get_item_style(ripple_size: f64, ripple_x: f64, ripple_y: f64) -> String {
    let top = -(ripple_size / 2.0) + ripple_y;
    let left = -(ripple_size / 2.0) + ripple_x;
    format!("width: {ripple_size}px; height: {ripple_size}px; top: {top}px; left: {left}px;")
}
