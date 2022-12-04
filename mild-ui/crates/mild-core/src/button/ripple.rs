use yew::prelude::*;

use crate::styles::prefixes;

#[derive(PartialEq, Properties)]
pub struct Props {
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
        ripple_size,
        ripple_x,
        ripple_y,
    }: &Props,
) -> Html {
    let entering = use_state_eq(|| false);
    let style = get_style(*ripple_size, *ripple_x, *ripple_y);

    use_effect_with_deps(
        move |entering| {
            entering.set(true);
        },
        entering.clone(),
    );

    html! {
        <span  style={style} class={prefixes(&["ripple","ripple-visible"])} >
            <span class={get_item_class(*entering)} />
        </span>
    }
}

fn get_style(ripple_size: f64, ripple_x: f64, ripple_y: f64) -> String {
    let top = -(ripple_size / 2.0) + ripple_y;
    let left = -(ripple_size / 2.0) + ripple_x;
    format!("width: {ripple_size}px; height: {ripple_size}px; top: {top}px; left: {left}px;")
}

fn get_item_class(entering: bool) -> Classes {
    let mut item_class = vec!["ripple_item"];

    if entering {
        item_class.push("ripple_item-entering");
    }

    classes!(prefixes(&item_class))
}
