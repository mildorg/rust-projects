pub fn learn() {
    // base();
    // mutability();
    // aliasing();
    ref_pattern();
}

fn eat_box(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is {}", borrowed_i32);
}

fn base() {
    let boxed = Box::new(5);
    let stacked = &6;

    borrow_i32(&boxed);
    borrow_i32(&stacked);

    {
        let ref_to_i32 = &boxed;

        // eat_box(boxed);
        borrow_i32(ref_to_i32);
    }

    eat_box(boxed);
}

#[derive(Debug, Clone, Copy)]
struct Book {
    author: &'static str,
    title: &'static str,
    year: u32,
}

fn borrow_book(book: &Book) {
    println!(
        "I immutably borrowed {} - {} edition",
        book.title, book.year
    );
}

fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn mutability() {
    let immutable_book = Book {
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Batch",
        year: 1979,
    };

    let mut mutable_book = immutable_book;

    borrow_book(&immutable_book);
    borrow_book(&mutable_book);

    new_edition(&mut mutable_book);
    // new_edition(&mut immutable_book);
}

struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn aliasing() {
    let mut point = Point { x: 0, y: 0, z: 0 };

    let borrowed_point = &point;
    let another_point = &point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_point.y, point.z
    );

    // let mutable_borrow = &mut point;

    println!(
        "Point has coordinates: ({}, {}, {})",
        borrowed_point.x, another_point.y, point.z
    );

    let mutable_borrow = &mut point;

    mutable_borrow.x = 5;
    mutable_borrow.y = 2;
    mutable_borrow.z = 1;

    // let y = &point.y;

    println!(
        "Point has coordinates: ({}, {}, {})",
        mutable_borrow.x, mutable_borrow.y, mutable_borrow.z
    );

    let new_borrowed_point = &point;
    println!(
        "Point has coordinates: ({}, {}, {})",
        new_borrowed_point.x, new_borrowed_point.y, new_borrowed_point.z
    );
}

#[derive(Clone, Copy)]
struct RPoint {
    x: i32,
    y: i32,
}

fn ref_pattern() {
    let c = 'Q';
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    let point = RPoint { x: 0, y: 0 };

    let copy_of_x = {
        let RPoint {
            x: ref ref_to_x,
            y: _,
        } = point;
        *ref_to_x
    };

    let mut mutable_point = point;

    {
        let RPoint {
            x: _,
            y: ref mut mut_ref_to_y,
        } = mutable_point;

        *mut_ref_to_y = 1;
    }

    println!("point is ({}, {})", point.x, point.y);
    println!("mutable_point is ({},{})", mutable_point.x, mutable_point.y);

    let mut mutable_type = (Box::new(5u32), 3u32);

    {
        let (_, ref mut last) = mutable_type;
        *last = 2u32;
    }

    println!("tuple is {:?}", mutable_type);
}
