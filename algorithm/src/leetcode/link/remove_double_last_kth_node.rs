use std::cmp::Ordering;

use crate::link::double_link::DoubleLink;

#[allow(dead_code)]
fn remove_kth_node<T>(mut link: DoubleLink<T>, k: u32) -> DoubleLink<T> {
    let mut cur = link.clone();
    let mut len = 0;

    while let Some(node) = cur {
        len += 1;
        cur = node.borrow().next.clone();
    }

    match len.cmp(&k) {
        Ordering::Less => link,
        Ordering::Equal => link.take().and_then(|node| {
            let new_link = node.borrow_mut().next.take();

            if let Some(item) = new_link.as_ref() {
                item.borrow_mut().pre.take();
            }

            new_link
        }),
        Ordering::Greater => {
            let mut pre = link.clone();
            let mut i = len - k - 1;

            while i > 0 && pre.is_some() {
                i -= 1;
                pre = pre.unwrap().borrow().next.clone();
            }

            if let Some(node) = pre.as_mut() {
                let next = node.borrow_mut().next.take().and_then(|item| {
                    item.borrow_mut().pre.take();
                    item.borrow_mut().next.take()
                });

                node.borrow_mut().next = next.clone();

                if let Some(item) = next {
                    item.borrow_mut().pre = pre;
                }
            }

            link
        }
    }
}

#[cfg(test)]
mod test {

    use crate::{
        link::double_link::{DoubleLink, DoubleList},
        utils::iter,
    };

    use super::remove_kth_node;

    #[test]
    fn basics_1() {
        let list = DoubleList::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![2, 3, 4, 7, 1];

        let (head, _) = list.take_head_tail();
        let link = remove_kth_node(head, 3);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    #[test]
    fn basics_2() {
        let list = DoubleList::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![2, 3, 4, 5, 7, 1];

        let (head, _) = list.take_head_tail();
        let link = remove_kth_node(head, 7);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    #[test]
    fn basics_3() {
        let list = DoubleList::from_iter(vec![2, 3, 4, 5, 7, 1]);
        let test = vec![3, 4, 5, 7, 1];

        let (head, _) = list.take_head_tail();
        let link = remove_kth_node(head, 6);
        assert_eq!(link_to_string(link), iter::to_sting(test));
    }

    fn link_to_string<T: ToString>(head: DoubleLink<T>) -> String {
        iter::to_sting(DoubleList::from_head(head))
    }
}
