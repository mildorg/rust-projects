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
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Box::new(ListNode::new(i32::MIN)); // 申请一个伪头结点
        let mut cur = &mut head;

        while let (Some(node1), Some(node2)) = (list1.as_ref(), list2.as_ref()) {
            let small = if node1.val <= node2.val { &mut list1 } else { &mut list2 };

            cur.next = small.take();
            cur = cur.next.as_mut().unwrap();
            *small = cur.next.take();
        }

        cur.next = list1.or(list2);
        head.next
    }


    /* 关于return L1的个人理解: 递归的核心在于,我只关注我这一层要干什么,返回什么,至于我的下一层(规模减1),我不管,我就是甩手掌柜.

    好,现在我要merge L1,L2.我要怎么做?

    显然,如果L1空或L2空,我直接返回L1或L2就行,这很好理解.
    如果L1第一个元素小于L2的? 那我得把L1的这个元素放到最前面,至于后面的那串长啥样 ,我不管. 我只要接过下级员工干完活后给我的包裹, 然后把我干的活附上去(令L1->next = 这个包裹)就行
    这个包裹是下级员工干的活,即merge(L1->next, L2)
    我该返回啥?

    现在不管我的下一层干了什么,又返回了什么给我, 我只要知道,假设我的工具人们都完成了任务, 那我的任务也就完成了,可以返回最终结果了
    最终结果就是我一开始接手的L1头结点+下级员工给我的大包裹,要一并交上去, 这样我的boss才能根据我给它的L1头节点往下找,检查我完成的工作
    */
    pub fn merge_two_lists2(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        match (l1, l2) {
            (None, None) => None,
            (Some(node1), None) => Some(node1),
            (None, Some(node2)) => Some(node2),
            (Some(mut node1), Some(mut node2)) => {
                if node1.val >= node2.val {
                    node2.next = Self::merge_two_lists2(Some(node1), node2.next);
                    Some(node2)
                } else {
                    node1.next = Self::merge_two_lists2(node1.next, Some(node2));
                    Some(node1)
                }
            }
        }
    }
}
