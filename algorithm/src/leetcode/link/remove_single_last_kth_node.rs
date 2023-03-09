use std::{cmp::Ordering, fmt::Debug};

use crate::link::better_single_link::Link;

#[allow(dead_code)]
fn remove_kth_node<T: Debug>(mut link: Link<T>, k: u32) -> Link<T> {
    let mut cur = &link;
    let mut len = 0;

    // 获取链表长度
    while cur.is_some() {
        len += 1;
        cur = &cur.as_ref().unwrap().next;
    }

    match len.cmp(&k) {
        Ordering::Less => link,
        Ordering::Equal => link.unwrap().next,
        Ordering::Greater => {
            let mut pre = &mut link;
            let mut i = len - k - 1;

            while i > 0 && pre.is_some() {
                pre = &mut pre.as_mut().unwrap().next;
                i -= 1;
            }

            if let Some(node) = pre {
                let next = node.next.take().and_then(|mut node| node.next.take());
                node.next = next;
            }

            link
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        leetcode::link::remove_single_last_kth_node::remove_kth_node,
        link::better_single_link::{Link, List},
    };

    #[test]
    fn basics_1() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test_list = List::from_iter(vec![2, 3, 4, 7, 1]);

        let link = remove_kth_node(list.take_head(), 3);
        assert_eq!(link_to_string(link), list_to_sting(test_list));
    }

    #[test]
    fn basics_2() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test_list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);

        let link = remove_kth_node(list.take_head(), 7);
        assert_eq!(link_to_string(link), list_to_sting(test_list));
    }

    #[test]
    fn basics_3() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test_list = List::from_iter(vec![3, 4, 5, 7, 1]);

        let link = remove_kth_node(list.take_head(), 6);
        assert_eq!(link_to_string(link), list_to_sting(test_list));
    }

    fn list_to_sting<T: ToString>(list: List<T>) -> String {
        list.into_iter()
            .map(|elem| elem.to_string())
            .collect::<String>()
    }

    fn link_to_string<T: ToString>(link: Link<T>) -> String {
        list_to_sting(List::from_link(link))
    }
}
