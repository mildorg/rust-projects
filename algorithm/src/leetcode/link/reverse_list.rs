// leetcode :24
// 定义一个函数，输入一个链表的头节点，反转该链表并输出反转后链表的头节点。
//  head
//   a    ->   b    ->  c  ->  null
//   c    ->   b    ->  a  ->  null
//  null   <-  a    <-  b  <-  c

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

pub struct Solution {}

impl Solution {
    // 解法一： 使用辅助栈
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head.is_none() {
            return head;
        }

        let mut stack = vec![];

        // 将所有元素入栈
        let mut cur = head;
        while let Some(node) = cur {
            stack.push(node.val);
            cur = node.next;
        }

        // 所有元素出栈重新构建链表
        let mut head = Some(Box::new(ListNode::new(stack.pop().unwrap())));
        let mut tail = &mut head;

        while let Some(v) = stack.pop() {
            if let Some(node) = tail {
                node.next = Some(Box::new(ListNode::new(v)));
                tail = &mut (node.next);
            }
        }

        head
    }

    // 解法 2 直接转换 next 指针指向
    pub fn reverse_list2(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut pre = None;
        let mut cur = head;

        while let Some(mut node) = cur {
            cur = node.next;
            node.next = pre;
            pre = Some(node);
        }

        pre
    }
}
