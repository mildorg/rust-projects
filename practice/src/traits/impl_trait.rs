use core::num;
use std::io;
use std::iter::{self, Chain, Cycle};
use std::vec::{self, IntoIter};

pub fn learn() {
    // test_combine_vecs();
    // test_make_adder_function();
    test_double_positive();
}

fn parse_csv_document(src: impl io::BufRead) -> io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            line.map(|line| {
                line.split(',')
                    .map(|entry| String::from(entry.trim()))
                    .collect()
            })
        })
        .collect()
}

// fn combine_vees(v: Vec<i32>, u: Vec<i32>) -> Cycle<Chain<IntoIter<i32>, IntoIter<i32>>> {
//     v.into_iter().chain(u.into_iter()).cycle()
// }

fn test_combine_vecs() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];

    let mut v3 = combine_vecs(v1, v2);

    assert_eq!(Some(1), v3.next());
    assert_eq!(Some(2), v3.next());
    assert_eq!(Some(3), v3.next());
    assert_eq!(Some(4), v3.next());
    assert_eq!(Some(5), v3.next());

    println!("all done");
}

fn combine_vecs(v: Vec<i32>, u: Vec<i32>) -> impl Iterator<Item = i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

fn test_make_adder_function() {
    let plus_one = make_adder_function(1);
    println!("plus_one(2) is: {}", plus_one(2))
}

fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| x + y;
    closure
}

fn test_double_positive() {
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positive(&singles);
    let doubles: Vec<i32> = doubles.collect();
    assert_eq!(doubles, vec![4, 6]);
}

fn double_positive(numbers: &Vec<i32>) -> impl Iterator<Item = i32> + '_ {
    numbers.iter().filter(|x| x > &&0).map(|x| x * 2)
}
