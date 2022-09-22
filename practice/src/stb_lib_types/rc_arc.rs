use std::{rc::Rc, sync::Arc, thread, time::Duration};

pub fn learn() {
    // test_rc();
    test_arc();
}

fn test_rc() {
    let rc_example = "Rc example".to_string();

    {
        println!("--- rc_a is created ---");
        let rc_a = Rc::new(rc_example);
        println!("Reference count of  rc_a: {} ", Rc::strong_count(&rc_a));

        {
            println!("--- rc_a is cloned to rc_b ---");
            let rc_b = Rc::clone(&rc_a);
            println!("Reference count of  rc_a: {} ", Rc::strong_count(&rc_a));
            println!("Reference count of  rc_b: {} ", Rc::strong_count(&rc_b));

            println!("rc_a and rc_b are equal: {}", rc_a.eq(&rc_b));

            println!("Length of the value inside rc_a: {}", rc_a.len());
            println!("Value of rc_b: {}", rc_b);

            println!("--- rc_b is dropped out of scope ---");
        }

        println!("Reference count of rc_a: {}", Rc::strong_count(&rc_a));
        println!("--- rc_a is dropped out of scope ---");
    }

    // println!("rc_examples: {}", rc_example);
}

// test atomically reference count
fn test_arc() {
    let apple = Arc::new("the same apple");

    for _ in 0..10 {
        let arc_apple = Arc::clone(&apple);

        thread::spawn(move || {
            println!("{}", arc_apple);
        });
    }

    thread::sleep(Duration::from_secs(1));
}
