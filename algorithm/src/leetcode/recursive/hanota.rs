#![allow(unused)]

/// 汉诺塔问题: https://leetcode.cn/problems/hanota-lcci/

struct Solution {}


// a -> c
impl Solution {
    pub fn hanota(a: &mut Vec<i32>, b: &mut Vec<i32>, c: &mut Vec<i32>) {
        let len = a.len();
        if len == 0 { return; }

        Self::process(len, a, b, c);
    }

    fn process(n: usize, from: &mut Vec<i32>, bridge: &mut Vec<i32>, target: &mut Vec<i32>) {
        if n == 1 {
            target.push(from.pop().unwrap());
            return;
        }

        Self::process(n - 1, from, target, bridge);
        target.push(from.pop().unwrap());
        Self::process(n - 1, bridge, from, target);
    }
}