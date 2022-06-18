enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: u32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            List::Cons(_, ref tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }

    fn stringify(&self) -> String {
        match *self {
            List::Cons(elem, ref tail) => {
                format!("{}, {}", elem, tail.stringify())
            }
            List::Nil => format!("Nill"),
        }
    }
}

pub fn practice_linked_list() {
    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
