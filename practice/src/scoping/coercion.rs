pub fn learn() {
    let first = 2;

    {
        let second = 3;

        println!("The product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
    }
}

fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first + second
}

fn choose_first<'a: 'b, 'b>(first: &'a i32, second: &'b i32) -> &'b i32 {
    first
}
