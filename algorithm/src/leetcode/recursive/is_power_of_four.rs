#![allow(unused)]
///4的幂 https://leetcode.cn/problems/power-of-four/

struct Solution {}

impl Solution {
    pub fn is_power_of_four(n: i32) -> bool {
        n % 3 == 1 && (n & (n - 1) == 0)
    }

    pub fn is_power_of_four2(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        if n < 4 || n % 4 != 0 {
            return false;
        }

        Self::is_power_of_four2(n / 4)
    }
}
