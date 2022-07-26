pub fn learn() {
    let closure = || println!("I'm a closure");

    call_me(closure);
    call_me(fun);
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn fun() {
    println!("I'm a function");
}
