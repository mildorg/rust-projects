#![allow(unused)]

/// 2 的幂 https://leetcode.cn/problems/power-of-two/
struct Solution {}

impl Solution {
    pub fn is_power_of_two1(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        if n < 1 || n % 2 != 0 {
            return false;
        }

        Self::is_power_of_two1(n / 2)
    }

    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1) == 0)
    }
}
