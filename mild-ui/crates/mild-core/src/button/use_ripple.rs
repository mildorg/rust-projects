use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use web_sys::{DomRect, HtmlElement};
use yew::prelude::*;
use yew::virtual_dom::VNode;

use super::ripple::Ripple;
use crate::styles::prefix;
use crate::utils::web::timer::{clear_timeout, set_timeout};

pub struct UseRipple {
    pub ripple_wrapper: VNode,
    pub start: Callback<MouseEvent>,
    pub stop: Callback<MouseEvent>,
}

#[hook]
pub fn use_ripple(class: Option<Classes>, center: bool, timeout: Option<u32>) -> UseRipple {
    let ripples = use_state(Vec::new);
    let next_key = use_state_eq(|| 0);
    let container_ref = use_node_ref();
    let clear_ref = use_mut_ref(|| 0);

    let timeout = timeout.unwrap_or(500);
    let classes = classes!(class, prefix("ripple_wrapper"));
    let ripple_list = (*ripples).clone().into_iter().collect::<Vec<VNode>>();

    let stop = stop(&ripples, &clear_ref, timeout);
    let start = start(&container_ref, center, &next_key, &ripples);

    use_effect_with_deps(
        |clear_ref| {
            let clear_ref = clear_ref.clone();
            move || clear_timeout(*clear_ref.borrow())
        },
        clear_ref,
    );

    let ripple_wrapper = html! {
        <span ref={container_ref} class={classes} >
            {ripple_list}
        </span>
    };

    UseRipple {
        start,
        stop,
        ripple_wrapper,
    }
}

fn start(
    container_ref: &NodeRef,
    center: bool,
    next_key: &UseStateHandle<i32>,
    ripples: &UseStateHandle<Vec<VNode>>,
) -> Callback<MouseEvent> {
    let container_ref = container_ref.clone();
    let next_key = next_key.clone();
    let ripples = ripples.clone();

    Callback::from(move |e: MouseEvent| {
        let element = container_ref.cast::<HtmlElement>();
        let client_x = e.client_x();
        let client_y = e.client_y();

        if let Some(el) = element {
            let rect = el.get_bounding_client_rect();

            let (ripple_x, ripple_y) = get_coordinates(&rect, client_x, client_y, center);
            let ripple_size = get_ripple_size(el, &rect, ripple_x, ripple_y, center);

            create_ripple(ripple_x, ripple_y, ripple_size, &next_key, &ripples)
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

fn create_ripple(
    ripple_x: f64,
    ripple_y: f64,
    ripple_size: f64,
    next_key: &UseStateHandle<i32>,
    ripples: &UseStateHandle<Vec<VNode>>,
) {
    let next_key = next_key.clone();
    let mut new_ripples = ripples.deref().clone();

    let ripple = html! {
        <Ripple
            {ripple_x}
            {ripple_y}
            {ripple_size}
            key={*next_key}
        />
    };

    new_ripples.push(ripple);

    ripples.set(new_ripples);
    next_key.set(*next_key + 1);
}

fn stop(
    ripples: &UseStateHandle<Vec<VNode>>,
    clear_ref: &Rc<RefCell<i32>>,
    timeout: u32,
) -> Callback<MouseEvent> {
    let clear_ref = clear_ref.clone();
    let ripples = ripples.clone();
    let timeout = timeout / 2;

    Callback::from(move |_| {
        if !ripples.is_empty() {
            let clone_clear_ref = clear_ref.clone();
            let ripples = ripples.clone();

            let clear = set_timeout(timeout, move || {
                // clear the previous timer
                clear_timeout(*clone_clear_ref.borrow());
                ripples.set(ripples[1..].to_vec());
            });

            *clear_ref.borrow_mut() = clear;
        }
    })
}
