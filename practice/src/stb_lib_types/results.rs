use std::result;

pub fn learn() {
    // println!("{}", op_v1(1.0, 10.0));
    op_v2(1.0, 10.0);
}

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NonPositiveLogarithm,
    NegativeSquareRoot,
}

type MathResult = Result<f64, MathError>;

fn div(x: f64, y: f64) -> MathResult {
    if y == 0.0 {
        Err(MathError::DivisionByZero)
    } else {
        Ok(x / y)
    }
}

fn sqrt(x: f64) -> MathResult {
    if x < 0.0 {
        Err(MathError::NegativeSquareRoot)
    } else {
        Ok(x.sqrt())
    }
}

fn ln(x: f64) -> MathResult {
    if x <= 0.0 {
        Err(MathError::NonPositiveLogarithm)
    } else {
        Ok(x.ln())
    }
}

/// `op(x,y)` === `sqrt(ln(x /y))`
fn op_v1(x: f64, y: f64) -> f64 {
    match div(x, y) {
        Err(why) => panic!("{:?}", why),
        Ok(ratio) => match ln(ratio) {
            Err(why) => panic!("{:?}", why),
            Ok(ln) => match sqrt(ln) {
                Err(why) => panic!("{:?}", why),
                Ok(sqrt) => sqrt,
            },
        },
    }
}

// option mark `?`
fn get_op(x: f64, y: f64) -> MathResult {
    let ratio = div(x, y)?;
    let ln = ln(ratio)?;

    sqrt(ln)
}

fn op_v2(x: f64, y: f64) {
    match get_op(x, y) {
        Ok(value) => println!("{}", value),
        Err(err) => {
            let why = match err {
                MathError::DivisionByZero => "division by zero",
                MathError::NonPositiveLogarithm => "logarithm of non-positive number",
                MathError::NegativeSquareRoot => "square root of negative number",
            };

            panic!("{}", why);
        }
    }
}
