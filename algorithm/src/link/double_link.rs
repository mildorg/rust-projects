use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

pub struct DoubleList<T> {
    head: DoubleLink<T>,
    tail: DoubleLink<T>,
}

pub type DoubleLink<T> = Option<Rc<RefCell<DoubleNode<T>>>>;

pub struct DoubleNode<T> {
    pub elem: T,
    pub next: DoubleLink<T>,
    pub pre: DoubleLink<T>,
}

impl<T> DoubleNode<T> {
    pub fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            elem,
            next: None,
            pre: None,
        }))
    }
}

impl<T> DoubleList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
        }
    }

    pub fn from_head(head: DoubleLink<T>) -> Self {
        let mut tail = head.clone();

        while let Some(node) = tail.as_ref().and_then(|item| item.borrow().next.clone()) {
            tail = node.borrow().next.clone();
        }

        Self { head, tail }
    }

    pub fn from_tail(tail: DoubleLink<T>) -> Self {
        let mut head = tail.clone();

        while let Some(node) = head.as_ref().and_then(|item| item.borrow().pre.clone()) {
            head = node.borrow().pre.clone();
        }

        Self { head, tail }
    }

    pub fn push_front(&mut self, elem: T) {
        let new_head = DoubleNode::new(elem);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().pre = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
        }
    }

    pub fn push_back(&mut self, elem: T) {
        let new_tail = DoubleNode::new(elem);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().pre = Some(old_tail);
                self.tail = Some(new_tail);
            }
            None => {
                self.head = Some(new_tail.clone());
                self.tail = Some(new_tail);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                // pop 后是非空链表
                Some(new_head) => {
                    new_head.borrow_mut().pre.take();
                    self.head = Some(new_head);
                }
                // pop 后是空链表
                None => {
                    self.tail.take();
                }
            }

            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().pre.take() {
                // pop后 是非空链表
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                // pop 后是空链表
                None => {
                    self.head.take();
                }
            }

            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().elem
        })
    }

    pub fn peek_front(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_front_mut(&mut self) -> Option<RefMut<T>> {
        self.head
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn peek_back(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.elem))
    }

    pub fn peek_back_mut(&mut self) -> Option<RefMut<T>> {
        self.tail
            .as_mut()
            .map(|node| RefMut::map(node.borrow_mut(), |node| &mut node.elem))
    }

    pub fn take_head_tail(mut self) -> (DoubleLink<T>, DoubleLink<T>) {
        let head = self.head.take();
        let tail = self.tail.take();
        (head, tail)
    }
}

impl<T> Default for DoubleList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for DoubleList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T> FromIterator<T> for DoubleList<T> {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut list = DoubleList::new();

        for elem in iter.into_iter() {
            list.push_back(elem);
        }

        list
    }
}

pub struct IntoIter<T>(DoubleList<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop_front()
    }
}

impl<T> DoubleEndedIterator for IntoIter<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.pop_back()
    }
}

impl<T> IntoIterator for DoubleList<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

#[cfg(test)]
mod test {
    use super::DoubleList;
    #[test]
    fn basics() {
        let mut list = DoubleList::new();

        // Check empty list behaves right
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);

        // ---- back -----

        // Check empty list behaves right
        assert_eq!(list.pop_back(), None);

        // Populate list
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_back(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push_back(4);
        list.push_back(5);

        // Check normal removal
        assert_eq!(list.pop_back(), Some(5));
        assert_eq!(list.pop_back(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek() {
        let mut list = DoubleList::new();
        assert!(list.peek_front().is_none());
        assert!(list.peek_back().is_none());
        assert!(list.peek_front_mut().is_none());
        assert!(list.peek_back_mut().is_none());

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(&*list.peek_front().unwrap(), &3);
        assert_eq!(&mut *list.peek_front_mut().unwrap(), &mut 3);
        assert_eq!(&*list.peek_back().unwrap(), &1);
        assert_eq!(&mut *list.peek_back_mut().unwrap(), &mut 1);
    }

    #[test]
    fn into_iter() {
        let mut list = DoubleList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next_back(), Some(1));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn long_list() {
        let mut list = DoubleList::new();

        for i in 0..100_000 {
            list.push_front(i);
        }

        drop(list);
    }
}
