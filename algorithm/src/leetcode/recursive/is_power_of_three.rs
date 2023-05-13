#![allow(unused)]
///3 的幂: https://leetcode.cn/problems/power-of-three/

struct Solution {}

impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        if n == 1 {
            return true;
        }

        if n < 3 || n % 3 != 0 {
            return false;
        }

        Self::is_power_of_three(n / 3)
    }
}
