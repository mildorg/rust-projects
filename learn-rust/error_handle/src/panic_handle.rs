pub fn learn_panic() {
    // panic!("crash and burn");
    _backtrace();
}

fn _backtrace() {
    let v = vec![1, 2, 3];
    v[99];
}
