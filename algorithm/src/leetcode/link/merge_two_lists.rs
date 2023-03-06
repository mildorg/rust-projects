// 合并两个有序的链表
// leetcode: https://leetcode.cn/problems/he-bing-liang-ge-pai-xu-de-lian-biao-lcof/

#![allow(dead_code)]

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
    pub fn merge_two_lists(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut pre = Box::new(ListNode::new(i32::MIN)); // 申请一个伪头结点
        let mut tail = &mut pre;

        while l1.is_some() && l2.is_some() {
            if l1.as_ref().unwrap().val <= l2.as_ref().unwrap().val {
                tail.next = l1;
                l1 = tail.next.as_mut().and_then(|item| item.next.take());
            } else {
                tail.next = l2;
                l2 = tail.next.as_mut().and_then(|item| item.next.take());
            }

            tail = tail.next.as_mut().unwrap();
        }

        if l1.is_some() {
            tail.next = l1;
        }

        if l2.is_some() {
            tail.next = l2;
        }

        pre.next
    }
}
