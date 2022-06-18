pub fn practice_declare_first() {
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding {}", a_binding);

    let another_binding: i32;
    // println!("another binding is {}", another_binding);

    another_binding = 1;
    println!("another binding is {}", another_binding);
}
