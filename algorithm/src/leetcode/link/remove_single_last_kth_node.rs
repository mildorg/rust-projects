use std::{cmp::Ordering, fmt::Debug};

use crate::link::better_single_link::Link;

#[allow(dead_code)]
fn remove_kth_node<T: Debug>(mut link: Link<T>, k: u32) -> Link<T> {
    let mut cur = &link;
    let mut len = 0;

    // 获取链表长度
    while let Some(node) = cur {
        len += 1;
        cur = &node.next;
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
        link::better_single_link::{Link, List},
        utils::iter,
    };

    use super::remove_kth_node;

    #[test]
    fn basics_1() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![2, 3, 4, 7, 1];

        let link = remove_kth_node(list.take_head(), 3);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    #[test]
    fn basics_2() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![2, 3, 4, 5, 7, 1];

        let link = remove_kth_node(list.take_head(), 7);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    #[test]
    fn basics_3() {
        let list = List::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![3, 4, 5, 7, 1];

        let link = remove_kth_node(list.take_head(), 6);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    fn link_to_string<T: ToString>(link: Link<T>) -> String {
        iter::to_sting(List::from_link(link))
    }
}
