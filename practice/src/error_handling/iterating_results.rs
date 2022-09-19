use std::num::ParseIntError;

pub fn learn() {
    // test_filter_map();
    // test_map_err();
    // test_fail_entire_operation();
    test_partition();
}

fn test_filter_map() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<i32> = strings
        .into_iter()
        .filter_map(|s| s.parse::<i32>().ok())
        .collect();

    println!("Results: {:?}", numbers);
}

fn test_map_err() {
    let strings = vec!["42", "tofu", "93", "999", "18"];
    let mut errors = vec![];

    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();

    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}

fn test_fail_entire_operation() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Result<Vec<i32>, ParseIntError> =
        strings.into_iter().map(|s| s.parse::<i32>()).collect();

    println!("Result: {:?}", numbers);
}

fn test_partition() {
    let strings = vec!["tofu", "93", "18"];
    let (numbers, errors): (Vec<_>, Vec<_>) = strings
        .into_iter()
        .map(|s| s.parse::<i32>())
        .partition(Result::is_ok);

    let numbers: Vec<i32> = numbers.into_iter().map(Result::unwrap).collect();
    let errors: Vec<ParseIntError> = errors.into_iter().map(Result::unwrap_err).collect();

    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors);
}
