#![allow(unused)]

/// 有效时间的数目: https://leetcode.cn/problems/number-of-valid-clock-times/
struct Solution {}

impl Solution {
    // hh::mm
    pub fn count_time(time: String) -> i32 {
        let t = time.chars().collect::<Vec<char>>();

        // 枚举小时上的可能性
        let mut h = 1;
        match ('?' == t[0], '?' == t[1]) {
            (true, true) => h = 24,
            (true, false) => {
                if t[1].to_digit(10).unwrap() <= 3 {
                    h = 3;
                } else {
                    h = 2;
                }
            }
            (false, true) => {
                if t[0].to_digit(10).unwrap() == 2 {
                    h = 4;
                } else {
                    h = 10;
                }
            }
            (false, false) => h = 1,
        }

        // 枚举分钟上的可能性
        let mut m = 1;
        match ('?' == t[3], '?' == t[4]) {
            (true, true) => m = 60,
            (true, false) => m = 6,
            (false, true) => m = 10,
            (false, false) => m = 1,
        }

        h * m
    }

    pub fn count_time2(time: String) -> i32 {
        let t = time.chars().collect::<Vec<char>>();

        let h = match (t[0], t[1]) {
            ('?', '?') => 24,
            ('?', '0'..='3') => 3,
            ('?', '4'..='9') => 2,
            ('0' | '1', '?') => 10,
            ('2', '?') => 4,
            _ => 1,
        };

        let m = match (t[3], t[4]) {
            ('?', '?') => 60,
            ('?', _) => 6,
            (_, '?') => 10,
            _ => 1,
        };

        h * m
    }
}
