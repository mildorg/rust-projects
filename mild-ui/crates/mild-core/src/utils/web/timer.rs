use gloo_timers::callback::Timeout;

use super::get_window;

/// Use the clearTimeout method from the js system
pub(crate) fn clear_timeout(id: i32) {
    get_window().clear_timeout_with_handle(id);
}

/// Use setTimeout method from the js system
pub(crate) fn set_timeout<F: 'static + FnOnce()>(timeout: u32, handler: F) -> i32 {
    Timeout::new(timeout, handler).forget()
}
