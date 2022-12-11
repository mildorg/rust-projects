///The js system's console.log
#[allow(unused)]
pub fn info<T: ToString>(msg: T) {
    web_sys::console::log_1(&msg.to_string().into());
}

/// The js system's console.warn
#[allow(unused)]
pub fn warn<T: ToString>(msg: T) {
    web_sys::console::warn_1(&msg.to_string().into());
}

/// The js system's console.error
#[allow(unused)]
pub fn error<T: ToString>(msg: T) {
    web_sys::console::error_1(&msg.to_string().into());
}
