use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;

/// 并查集

/// 并查集 hash map 实现
pub struct UnionFind<T> {
    parents: HashMap<Rc<T>, Rc<T>>,
    sizes: HashMap<Rc<T>, u32>,
}

impl<T: Hash + Eq> UnionFind<T> {
    pub fn new(list: Vec<Rc<T>>) -> Self {
        let mut parents = HashMap::new();
        let mut sizes = HashMap::new();

        for value in list {
            parents.insert(value.clone(), value.clone());
            sizes.insert(value.clone(), 1);
        }

        Self { parents, sizes }
    }

    pub fn is_same_set(&mut self, a: Rc<T>, b: Rc<T>) -> bool {
        self.find_parent(a) == self.find_parent(b)
    }

    pub fn union(&mut self, a: Rc<T>, b: Rc<T>) {
        let a_head = self.find_parent(a);
        let b_head = self.find_parent(b);

        if a_head != b_head {
            let a_size = *self.sizes.get(&a_head).unwrap();
            let b_size = *self.sizes.get(&b_head).unwrap();
            let (big, small) = if a_size >= b_size {
                (a_head, b_head)
            } else {
                (b_head, a_head)
            };

            self.parents.insert(big.clone(), small.clone());
            self.sizes.insert(big, a_size + b_size);
            self.sizes.remove(&small);
        }
    }

    pub fn find_parent(&mut self, node: Rc<T>) -> Rc<T> {
        let mut cur = node;
        let mut path = vec![];

        while cur != self.parents.get(&cur).unwrap().clone() {
            path.push(cur.clone());
            cur = self.parents.get(&cur).unwrap().clone();
        }

        while !path.is_empty() {
            self.parents.insert(path.pop().unwrap(), cur.clone());
        }

        cur
    }
}
