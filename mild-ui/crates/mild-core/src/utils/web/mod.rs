//! web system utils
pub(crate) mod log;
pub(crate) mod timer;

/// Get the `window` object of web environment
pub(crate) fn get_window() -> web_sys::Window {
    web_sys::window().expect("Can't find the `window` object, make sure you're in web environment!")
}
