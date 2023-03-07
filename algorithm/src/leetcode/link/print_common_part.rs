use std::cmp::Ordering;

use crate::link::better_single_link::Link;

#[allow(dead_code)]
fn common_part(h1: &Link<u32>, h2: &Link<u32>) -> String {
    let mut h1 = h1;
    let mut h2 = h2;
    let mut result = String::new();

    while h1.is_some() && h2.is_some() {
        let node1 = h1.as_ref().unwrap();
        let node2 = h2.as_ref().unwrap();

        match node1.elem.cmp(&node2.elem) {
            Ordering::Less => h1 = &node1.next,
            Ordering::Greater => h2 = &node2.next,
            Ordering::Equal => {
                let ch = char::from_digit(node1.elem, 10).unwrap();
                result.push(ch);

                h1 = &node1.next;
                h2 = &node2.next;
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use crate::link::better_single_link::List;

    use super::common_part;

    #[test]
    fn basics() {
        let l1 = List::from_iter([1, 3, 5, 6, 7]);
        let l2 = List::from_iter([3, 6, 7, 8]);
        let head1 = l1.peek_node();
        let head2 = l2.peek_node();

        let common = common_part(head1, head2);
        assert_eq!("367", common);
    }
}
