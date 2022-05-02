#[allow(dead_code)]

pub fn learn_type() {
    let a = 10;
    // let b = 10_u32;
    let b = 20;
    let c = 0;
    // c = 50_i32;

    let e = add(add(a, b), add(b, c));

    println!("The value of e is {}", e);
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
