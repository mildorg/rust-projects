pub fn learn() {
    explicit_annotation();
}

// explicit annotation
fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x  is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let x = 12;

    // let y: &'a i32 = &x;
}

fn explicit_annotation() {
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    failed_borrow();
}
