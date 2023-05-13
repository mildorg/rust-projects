#![allow(unused)]

/// 移除链表元素: https://leetcode.cn/problems/remove-linked-list-elements/
/// 给你一个链表的头节点 head 和一个整数 val，
/// 请你删除链表中所有满足 Node.val == val 的节点，并返回 新的头节点 。

use super::single_link::{ListNode, Solution};

// Definition for singly-linked list.
impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        head.and_then(|mut node| {
            let right = Self::remove_elements(node.next, val);

            if node.val == val {
                right
            } else {
                node.next = right;
                Some(node)
            }
        })
    }
}
