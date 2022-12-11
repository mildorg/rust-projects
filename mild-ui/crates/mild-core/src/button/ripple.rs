use yew::prelude::*;

use crate::styles::prefixes;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub is_bubble: bool,
    #[prop_or_default]
    pub ripple_size: f64,
    #[prop_or_default]
    pub ripple_x: f64,
    #[prop_or_default]
    pub ripple_y: f64,
}

#[function_component]
pub fn Ripple(
    Props {
        is_bubble,
        ripple_size,
        ripple_x,
        ripple_y,
    }: &Props,
) -> Html {
    let entering = use_state_eq(|| false);

    let is_bubble = *is_bubble;
    let style = get_style(*ripple_size, *ripple_x, *ripple_y);

    use_effect_with_deps(
        move |entering| {
            entering.set(true);
        },
        entering.clone(),
    );

    html! {
        <span  style={style} class={get_class(is_bubble)} >
            <span  class={get_item_class(*entering,is_bubble)} />
        </span>
    }
}

fn get_style(ripple_size: f64, ripple_x: f64, ripple_y: f64) -> String {
    let top = -(ripple_size / 2.0) + ripple_y;
    let left = -(ripple_size / 2.0) + ripple_x;
    format!("width: {ripple_size}px; height: {ripple_size}px; top: {top}px; left: {left}px;")
}

fn get_class(is_bubble: bool) -> Classes {
    let mut class = vec!["ripple"];

    if is_bubble {
        class.push("ripple-bubble");
    } else {
        class.push("ripple-visible");
    }

    classes!(prefixes(&class))
}

fn get_item_class(entering: bool, is_bubble: bool) -> Classes {
    let mut item_class = vec!["ripple_item"];

    if is_bubble {
        item_class.push("ripple_item-bubble");
        return classes!(prefixes(&item_class));
    }

    if entering {
        item_class.push("ripple_item-entering");
    }

    classes!(prefixes(&item_class))
}
