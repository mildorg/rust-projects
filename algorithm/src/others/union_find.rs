use std::{collections::HashMap, hash::Hash, rc::Rc};

/// 并查集

/// map实现
pub struct UnionFindMap<T> {
    nodes_map: HashMap<Rc<T>, Rc<T>>,
    parents_map: HashMap<Rc<T>, Rc<T>>,
    size_map: HashMap<Rc<T>, u32>,
}

impl<T: Hash + Eq> UnionFindMap<T> {
    pub fn new(list: Vec<Rc<T>>) -> Self {
        let mut nodes_map = HashMap::new();
        let mut parents_map = HashMap::new();
        let mut size_map = HashMap::new();

        for value in list {
            nodes_map.insert(value.clone(), value.clone());
            parents_map.insert(value.clone(), value.clone());
            size_map.insert(value, 1);
        }

        Self {
            nodes_map,
            parents_map,
            size_map,
        }
    }

    pub fn is_same_set(&mut self, a: Rc<T>, b: Rc<T>) -> bool {
        let a = self.nodes_map.get(&a).cloned();
        let b = self.nodes_map.get(&b).cloned();

        if a.is_none() || b.is_none() {
            return false;
        }

        self.find_parent(a.unwrap()) == self.find_parent(b.unwrap())
    }

    pub fn union(&mut self, a: Rc<T>, b: Rc<T>) {
        if !self.is_same_set(a.clone(), b.clone()) {
            let a_head = self.find_parent(self.nodes_map.get(&a).unwrap().clone());
            let b_head = self.find_parent(self.nodes_map.get(&b).unwrap().clone());
            let a_size = self.size_map.get(&a_head).unwrap();
            let b_size = self.size_map.get(&b_head).unwrap();

            let (big, small) = if a_size >= b_size {
                (a_head, b_head)
            } else {
                (b_head, a_head)
            };

            self.parents_map.insert(small.clone(), big.clone());
            self.size_map.insert(big, a_size + b_size);
            self.size_map.remove(&small);
        }
    }

    // 给你一个节点，请你往上到不能再往上，把代表返回
    fn find_parent(&mut self, el: Rc<T>) -> Rc<T> {
        let mut cur = el;
        let mut path = vec![];

        while cur != *self.parents_map.get(&cur).unwrap() {
            path.push(cur.clone());
            cur = self.parents_map.get(&cur).unwrap().clone();
        }

        while !path.is_empty() {
            let node = path.pop().unwrap();
            self.parents_map.insert(node, cur.clone());
        }

        cur
    }
}
