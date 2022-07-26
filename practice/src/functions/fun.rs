pub fn learn() {
    fizz_buzz_to(100);
}

fn fizz_buzz_to(n: u32) {
    for m in 1..=n {
        fizz_buzz(m)
    }
}

fn fizz_buzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

fn is_divisible_by(n: u32, arg: u32) -> bool {
    if arg == 0 {
        return false;
    }

    n % arg == 0
}
