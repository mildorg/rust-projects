#![allow(unused)]

use super::single_link::{ListNode, Solution};

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        // 将链表中的元素放到栈中
        let mut stack = vec![];
        let mut cur = head.as_ref();

        while let Some(node) = cur {
            stack.push(node.val);
            cur = node.next.as_ref();
        }

        // 弹出栈顶的元素，依次与链表比较，直到栈空
        cur = head.as_ref();
        while !stack.is_empty() {
            let node = cur.unwrap();
            if stack.pop().unwrap() != node.val {
                return false;
            }

            cur = node.next.as_ref();
        }

        true
    }

    pub fn is_palindrome2(head: Option<Box<ListNode>>) -> bool {
        if head.is_none() {
            return true;
        }

        // 将链表中的元素放到数组中
        let mut nums = vec![];
        let mut cur = head.as_ref();

        while let Some(node) = cur {
            nums.push(node.val);
            cur = node.next.as_ref();
        }

        // 判断数组是否是回文
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left < right {
            if nums[left] != nums[right] {
                return false;
            }

            left += 1;
            right -= 1;
        }

        true
    }

    // 递归法
    pub fn is_palindrome3(head: Option<Box<ListNode>>) -> bool {
        Self::check(&head, &head).1
    }

    fn check<'a>(head: &'a Option<Box<ListNode>>, cur: &'a Option<Box<ListNode>>) -> (&'a Option<Box<ListNode>>, bool) {
        match cur {
            None => (head, true),
            Some(node) => {
                let (h, pre) = Self::check(head, &node.next);
                let temp = h.as_ref().unwrap();
                (&temp.next, pre && temp.val == node.val)
            }
        }
    }
}