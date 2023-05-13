/// 递归求斐波那契数列

#[allow(dead_code)]
pub fn fib(n: u32) -> u32 {
    if n <= 2 {
        return 1;
    }

    fib(n - 1) + fib(n - 2)
}

#[cfg(test)]
mod test {
    use super::fib;

    #[test]
    fn basis() {
        println!("{}", '3'.to_digit(10).unwrap());

        assert_eq!(1, fib(1));
        assert_eq!(1, fib(2));
        assert_eq!(2, fib(3));
        assert_eq!(3, fib(4));
        assert_eq!(5, fib(5));
        assert_eq!(6765, fib(20));
    }
}
