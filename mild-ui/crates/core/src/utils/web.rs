//! web system utils

/// Get the `window` object of web environment
pub(crate) fn get_window() -> web_sys::Window {
    web_sys::window().expect("Can't find the `window` object, make sure you're in web environment!")
}

pub(crate) fn console_log(str: &str) {
    web_sys::console::log_1(&str.into());
}
