///The js system's console.log
#[allow(unused)]
pub(crate) fn info(str: &str) {
    web_sys::console::log_1(&str.into());
}

/// The js system's console.warn
#[allow(unused)]
pub(crate) fn warn(str: &str) {
    web_sys::console::warn_1(&str.into());
}

/// The js system's console.error
#[allow(unused)]
pub(crate) fn error(str: &str) {
    web_sys::console::error_1(&str.into());
}
