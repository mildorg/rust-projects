#![allow(unused)]

/// 反转链表 https://leetcode.cn/problems/reverse-linked-list/
/// 递归解答

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        Self::process(head, None)
    }

    fn process(
        head: Option<Box<ListNode>>,
        mut pre: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        head.and_then(|mut node| match node.next {
            None => {
                node.next = pre;
                Some(node)
            }
            Some(next) => {
                node.next = pre;
                pre = Some(node);
                Self::process(Some(next), pre)
            }
        })
    }
}
