use std::error::{self, Error};
use std::fmt::{self, Display, Formatter};
use std::num::ParseIntError;
use std::result::{self, Result};

pub fn learn() {
    // test_custom_error_type();
    // test_option_mark();
    test_wrapping_errors();
}

// define an error type
type CustomResult<T> = Result<T, DoubleError>;

fn test_custom_error_type() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    print(double_first(&numbers));
    print(double_first(&empty));
    print(double_first(&strings));
}

#[derive(Debug, Clone)]
struct DoubleError;

impl Display for DoubleError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: &Vec<&str>) -> CustomResult<i32> {
    vec.first()
        .ok_or(DoubleError)
        .and_then(|first| first.parse::<i32>().map_err(|_| DoubleError).map(|n| n * 2))
}

fn print<T: Display, E: Display>(result: Result<T, E>) {
    match result {
        Ok(n) => println!("The first double is {}", n),
        Err(e) => println!("Err: {}", e),
    }
}

// other use of ?
fn test_option_mark() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("test_option_mark");
    print(double_first_v2(&numbers));
    print(double_first_v2(&empty));
    print(double_first_v2(&strings));
}

type BoxResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
struct EmptyVec;

impl Display for EmptyVec {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

impl Error for EmptyVec {}

fn double_first_v2(vec: &Vec<&str>) -> BoxResult<i32> {
    let first_number = vec.first().ok_or(EmptyVec)?.parse::<i32>()?;
    Ok(2 * first_number)
}

// wrapping errors
fn test_wrapping_errors() {
    let numbers = vec!["42", "93", "18"];
    let empty: Vec<&str> = vec![];
    let strings = vec!["tofu", "93", "18"];

    println!("test_wrapping_errors");
    print_v2(double_first_v3(&numbers));
    print_v2(double_first_v3(&empty));
    print_v2(double_first_v3(&strings));
}

type WrapResult<T> = Result<T, DoubleErrorV2>;

#[derive(Debug)]
enum DoubleErrorV2 {
    EmptyVec,
    Parse(ParseIntError),
}

impl Display for DoubleErrorV2 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            DoubleErrorV2::EmptyVec => {
                write!(f, "please use a vector with at least one element")
            }
            DoubleErrorV2::Parse(_) => {
                write!(f, "the provided string could not be parsed as int")
            }
        }
    }
}

impl Error for DoubleErrorV2 {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            DoubleErrorV2::EmptyVec => None,
            DoubleErrorV2::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for DoubleErrorV2 {
    fn from(err: ParseIntError) -> Self {
        DoubleErrorV2::Parse(err)
    }
}

fn double_first_v3(vec: &Vec<&str>) -> WrapResult<i32> {
    let first_number = vec.first().ok_or(DoubleErrorV2::EmptyVec)?.parse::<i32>()?;
    Ok(2 * first_number)
}

fn print_v2(result: WrapResult<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => {
            println!("Error: {}", e);
            if let Some(source) = e.source() {
                println!("  Caused by: {}", source);
            }
        }
    }
}
