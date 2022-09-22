use std::process::Child;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

const N_THREADS: u32 = 3;

pub fn learn() {
    let (sx, rx) = mpsc::channel::<u32>();

    let mut children = Vec::new();

    for id in 0..N_THREADS {
        let thread_sx = sx.clone();

        let child = thread::spawn(move || {
            thread_sx.send(id).unwrap();

            println!("thread {} finished", id);
        });

        children.push(child);
    }

    let mut ids = Vec::with_capacity(N_THREADS as usize);

    for _ in 0..N_THREADS {
        ids.push(rx.recv());
    }

    for child in children {
        child.join().expect("oops! the child thread panicked");
    }

    println!("{:?}", ids);
}
