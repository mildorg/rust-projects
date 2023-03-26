use std::{collections::HashMap, hash::Hash};

/// 并查集

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub struct Node<T> {
    value: T,
}

/// map实现
pub struct UnionFindMap<T> {
    nodes_map: HashMap<T, Node<T>>,
    parents_map: HashMap<Node<T>, Node<T>>,
    size_map: HashMap<Node<T>, u32>,
}

impl<T: Hash + Eq + Clone + Copy> UnionFindMap<T> {
    pub fn new(list: Vec<T>) -> Self {
        let mut nodes_map = HashMap::new();
        let mut parents_map = HashMap::new();
        let mut size_map = HashMap::new();

        for value in list {
            let node = Node { value };

            nodes_map.insert(value, node);
            parents_map.insert(node, node);
            size_map.insert(node, 1);
        }

        Self {
            nodes_map,
            parents_map,
            size_map,
        }
    }

    pub fn is_same_set(&mut self, a: &T, b: &T) -> bool {
        let a = self.nodes_map.get(a).copied();
        let b = self.nodes_map.get(b).copied();

        if a.is_none() || b.is_none() {
            return false;
        }

        self.find_parent(a.unwrap()) == self.find_parent(b.unwrap())
    }

    pub fn union(&mut self, a: T, b: T) {
        if !self.is_same_set(&a, &b) {
            let a_head = self.find_parent(*self.nodes_map.get(&a).unwrap());
            let b_head = self.find_parent(*self.nodes_map.get(&b).unwrap());
            let a_size = self.size_map.get(&a_head).unwrap();
            let b_size = self.size_map.get(&b_head).unwrap();

            let (big, small) = if a_size >= b_size {
                (a_head, b_head)
            } else {
                (b_head, a_head)
            };

            self.parents_map.insert(small, big);
            self.size_map.insert(big, a_size + b_size);
            self.size_map.remove(&small);
        }
    }

    // 给你一个节点，请你往上到不能再往上，把代表返回
    fn find_parent(&mut self, node: Node<T>) -> Node<T> {
        let mut cur = node;
        let mut path = vec![];

        while cur != *self.parents_map.get(&cur).unwrap() {
            path.push(cur);
            cur = *self.parents_map.get(&cur).unwrap();
        }

        while !path.is_empty() {
            let node = path.pop().unwrap();
            self.parents_map.insert(node, cur);
        }

        cur
    }
}
