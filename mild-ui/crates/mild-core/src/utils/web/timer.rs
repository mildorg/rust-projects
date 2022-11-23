use gloo_timers::callback::Timeout;

use super::get_window;

/// Use setTimeout method of the js system
pub(crate) fn set_timeout<F: 'static + FnOnce()>(timeout: u32, handler: F) -> impl FnOnce() {
    let id = Timeout::new(timeout, handler).forget();

    move || get_window().clear_timeout_with_handle(id)
}
