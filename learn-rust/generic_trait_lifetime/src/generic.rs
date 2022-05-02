pub fn learn_generic() {
    _test_largest();
}

fn _largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    return largest;
}

fn _test_largest() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = _largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['a', 'y', 'm', 'q'];
    let result = _largest(&char_list);
    println!("The largest char is {}", result);
}
