use wasm_bindgen::UnwrapThrowExt;

/// Get the `window` object of web environment
pub(crate) fn get_window() -> web_sys::Window {
    web_sys::window().expect_throw("Can't find the `window`, please check your environment!")
}
