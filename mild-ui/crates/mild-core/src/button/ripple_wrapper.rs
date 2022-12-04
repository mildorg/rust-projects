use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;

use crate::utils::web::{log, timer};

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    class: AttrValue,
    #[prop_or_default]
    color: AttrValue,
    #[prop_or_default]
    center: bool,
    #[prop_or_default]
    component: AttrValue,
    #[prop_or_default]
    children: Children,
}

#[function_component()]
pub fn RippleWrapper(
    Props {
        class,
        color,
        center,
        component,
        children,
    }: &Props,
) -> Html {
    let ripples = use_state(|| vec![1]);
    let next_key = use_state_eq(|| 0);
    let timer_id = use_mut_ref(|| 0);
    let container_ref = use_node_ref();

    let handle_mouse_down = start(&container_ref, *center);

    // let handle_touch_start = Callback::from(|e| start(e));

    use_effect_with_deps(|_| move || timer::clear_timeout(*timer_id.borrow()), ());

    html! {
        <div
            style="width:100%;height:100%"
            title="abcd"
            ref={container_ref}
            onmousedown={handle_mouse_down}
            onmouseup={stop(&ripples)}
            onmouseleave={stop(&ripples)}

        ></div>
    }
}

fn start(container_ref: &NodeRef, center: bool) -> Callback<MouseEvent> {
    let container_ref = container_ref.clone();

    Callback::from(move |e: MouseEvent| {
        let element = container_ref.cast::<HtmlElement>();
        let client_x = e.client_x();
        let client_y = e.client_y();

        if let Some(el) = element {
            let rect = el.get_bounding_client_rect();

            let (ripple_x, ripple_y) = get_coordinates(&rect, client_x, client_y, center);
            let ripple_size = get_ripple_size(el, &rect, ripple_x, ripple_y, center);

            // let data = get_bounding_client_rect(el);

            // log::info(format!("{:?}", data).as_str());
            log::info(format!("{} {}", ripple_x, ripple_y).as_str());
            log::info(format!("{}", ripple_size).as_str())
        }
    })
}

fn get_coordinates(rect: &DomRect, client_x: i32, client_y: i32, center: bool) -> (f64, f64) {
    let x;
    let y;

    if center || client_x == 0 && client_y == 0 {
        x = (rect.width() / 2.0).round();
        y = (rect.height() / 2.0).round();
    } else {
        x = (client_x as f64 - rect.left()).round();
        y = (client_y as f64 - rect.top()).round();
    }

    (x, y)
}

fn get_ripple_size(
    el: HtmlElement,
    rect: &DomRect,
    ripple_x: f64,
    ripple_y: f64,
    center: bool,
) -> f64 {
    if center {
        (2.0 * rect.width().powi(2) + rect.height().powi(2) / 3.0).sqrt()
    } else {
        let x = (el.client_width() as f64 - ripple_x).abs().max(ripple_x) * 2.0 + 2.0;
        let y = (el.client_height() as f64 - ripple_y).abs().max(ripple_y) * 2.0 + 2.0;
        (x.powi(2) + y.powi(2)).sqrt()
    }
}

fn stop(ripples: &UseStateHandle<Vec<i32>>) -> Callback<MouseEvent> {
    let ripples = ripples.clone();

    Callback::from(move |_| {
        if !ripples.is_empty() {
            ripples.set(ripples[1..].to_vec())
        }
    })
}
