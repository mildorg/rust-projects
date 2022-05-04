use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guessing() {
    println!("猜数!");
    println!("请输入一个数!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("输入的不是一个数字");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("您输入的数是: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("太小了"),
            Ordering::Greater => println!("太大了"),
            Ordering::Equal => {
                println!("您赢了");
                break;
            }
        }
    }
}
