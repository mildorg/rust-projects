use std::thread;

const N_THREADS: u8 = 10;

pub fn learn() {
    let mut children = vec![];

    for i in 0..N_THREADS {
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        let _ = child.join();
    }
}
