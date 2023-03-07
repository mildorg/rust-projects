#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
}

pub type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct Node<T> {
    pub elem: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(elem: T) -> Self {
        Node { elem, next: None }
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        Self { head: None }
    }

    pub fn push(&mut self, elem: T) {
        self.push_node(Node { elem, next: None })
    }

    pub fn push_node(&mut self, mut node: Node<T>) {
        node.next = self.head.take();
        self.head = Some(Box::new(node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    pub fn peek_node(&self) -> &Link<T> {
        &self.head
    }

    pub fn peek_node_mut(&mut self) -> &mut Link<T> {
        &mut self.head
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            inner: self.head.as_deref(),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            inner: self.head.as_deref_mut(),
        }
    }

    pub fn form_iter(&mut self) {}
}

impl<T> Default for List<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> IntoIterator for List<T> {
    type Item = T;

    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> FromIterator<T> for List<T> {
    fn from_iter<A: IntoIterator<Item = T>>(iter: A) -> Self {
        let mut list = Self::new();

        for elem in iter.into_iter().collect::<Vec<T>>().into_iter().rev() {
            list.push(elem);
        }

        list
    }
}

pub struct Iter<'a, T> {
    inner: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.map(|node| {
            self.inner = node.next.as_deref();
            &node.elem
        })
    }
}

pub struct IterMut<'a, T> {
    inner: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.take().map(|node| {
            self.inner = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));

        if let Some(value) = list.peek_mut() {
            *value = 42;
        }

        assert_eq!(list.peek(), Some(&42));
        assert_eq!(list.pop(), Some(42));
    }

    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iter() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some(&3));
        assert_eq!(iter.next(), Some(&2));
        assert_eq!(iter.next(), Some(&1));
    }

    #[test]
    fn iter_mut() {
        let mut list = List::new();
        list.push(1);
        list.push(2);
        list.push(3);

        let mut iter = list.iter_mut();
        assert_eq!(iter.next(), Some(&mut 3));
        assert_eq!(iter.next(), Some(&mut 2));
        assert_eq!(iter.next(), Some(&mut 1));
    }
}
