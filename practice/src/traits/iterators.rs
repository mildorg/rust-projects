pub fn learn() {
    let mut sequence = 0..3;

    println!("Four consecutive 'next' calls on 0..3");
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());
    println!("> {:?}", sequence.next());

    println!("Iterate through 0..3 using for");
    for i in 0..3 {
        println!("> {}", i);
    }

    println!("The first four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::new().take(4) {
        println!("> {}", i);
    }

    println!("The next four terms of the Fibonacci sequence are: ");
    for i in Fibonacci::new().skip(4).take(4) {
        println!("> {}", i);
    }

    let arr: [u32; 4] = [1, 3, 3, 7];
    println!("Iterate the following array {:?}", &arr);
    for i in arr.iter() {
        println!("> {}", i);
    }
}

struct Fibonacci {
    cur: u32,
    next: u32,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { cur: 0, next: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let new_next = self.cur + self.next;

        self.cur = self.next;
        self.next = new_next;

        Some(self.cur)
    }
}
