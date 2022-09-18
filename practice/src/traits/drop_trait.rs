pub fn learn() {
    let a = Droppable { name: "a" };

    // block A
    {
        let b = Droppable { name: "b" };

        //block B
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };

            println!("Exiting block B");
        }
        println!("Just exited block B");

        println!("Exiting block A");
    }
    println!("Just exited block A");

    // drop(a);

    println!("end of the learn function");
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}
