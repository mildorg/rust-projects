pub fn learn() {
    drink("water");
    drink("lemonade")
}

fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic = "abort") {
            println!("This is not your party. Run!!!");
        } else {
            ah();
            panic!("AAAaaaa!!!");
        }
    }

    println!("Some refreshing {} is all I need.", beverage);
}

#[cfg(panic = "unwind")]
fn ah() {
    println!("Split it out!!!");
}

#[cfg(not(panic = "unwind"))]
fn ah() {
    println!("Split it out!!!");
}
