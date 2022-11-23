pub(crate) fn info(str: &str) {
    web_sys::console::log_1(&str.into());
}
