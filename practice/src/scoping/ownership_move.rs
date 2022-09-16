pub fn learn() {
    // base();
    // mutability();
    partial_move();
}

fn destruct_box(c: Box<i32>) {
    println!("Destroying a box that contains {}", c);
}

fn base() {
    let x = 5u32;
    let y = x;

    println!("x is {}, y is {}", x, y);

    let a = Box::new(5);
    println!("a contains: {}", a);

    // let b = a;
    // println!("a contains: {}", a);

    // destruct_box(b);
    // println!("b contains: {}", b);
}

fn mutability() {
    let immutable_box = Box::new(5u32);
    // *immutable_box = 4;

    let mut mutable_box = immutable_box;
    *mutable_box = 4;

    println!("mutable_box contains: {}", mutable_box);
}

#[derive(Debug)]
struct Person {
    name: String,
    age: Box<u8>,
}

fn partial_move() {
    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person { name, ref age } = person;

    println!("The person.name is {}", name);
    println!("The person.age is {}", age);

    // println!("The person struct is {:?}", person);
    println!("The person's age from person struct is {}", person.age);
}
