pub fn learn() {
    let x = 7;

    let print = || println!("{}", x);

    apply(print);
}

fn apply<F>(f: F)
where
    F: Fn(),
{
    f();
}
