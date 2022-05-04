fn main() {
    // let mut x = 5;
    // println!("Variable x is {}", x);
    //
    // x = 6;
    // println!("Variable x is {}", x);
    //
    // // const
    // const THREE_TIMES_IN_SECONDS: u32 = 60 * 60 * 3;
    // println!("THREE_TIMES_IN_SECONDS is {}", THREE_TIMES_IN_SECONDS);
    //
    // // shadow
    // let x = x + 1;
    // {
    //     let x = x * x;
    //     println!("The value of x in the inner scope is {}", x);
    // }
    //
    // println!("x is {}", x);

    // let spaces = "   ";
    // let spaces = spaces.len();
    // println!("{}", spaces);

    // let mut  spaces = "   ";
    // spaces = spaces.len();

    let guess: u32 = "42".parse().expect("Not a number!");
    println!("guess is {}", guess);
}
