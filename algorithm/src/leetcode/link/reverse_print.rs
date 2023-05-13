#![allow(unused)]

/// 从尾到头打印链表 https://leetcode.cn/problems/cong-wei-dao-tou-da-yin-lian-biao-lcof/

use super::single_link::{Solution, ListNode};

impl Solution {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        let mut arr = vec![];
        Self::process(&mut arr, head);
        arr
    }

    fn process(collect: &mut Vec<i32>, head: Option<Box<ListNode>>) {
        match head {
            None => (),
            Some(node) => {
                Self::process(collect, node.next);
                collect.push(node.val);
            }
        }
    }
}

struct Solution2{}

impl Solution2 {
    pub fn reverse_print(head: Option<Box<ListNode>>) -> Vec<i32> {
        Self::process(vec![], head)
    }

    fn process(collect: Vec<i32>, head: Option<Box<ListNode>>) -> Vec<i32> {
        match head {
            None => collect,
            Some(node) => {
                let mut nums = Self::process(collect, node.next);
                nums.push(node.val);
                nums
            }
        }
    }
}