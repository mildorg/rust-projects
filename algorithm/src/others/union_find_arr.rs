/// 并查集 - 数组实现
/// 测试链接 : https://www.nowcoder.com/questionTerminal/e7ed657974934a30b2010046536a5372
use std::io::*;

const MAX_N: usize = 1_000_001;

pub struct UnionFind {
    parents: [usize; MAX_N],
    sizes: [usize; MAX_N],
    path: [usize; MAX_N],
}

impl UnionFind {
    pub fn new() -> Self {
        Self {
            parents: [0; MAX_N],
            sizes: [0; MAX_N],
            path: [0; MAX_N],
        }
    }

    pub fn init(&mut self, n: usize) {
        for i in 1..=n {
            self.parents[i] = i;
            self.sizes[i] = 1;
        }
    }

    pub fn same_set(&mut self, a: usize, b: usize) -> bool {
        self.find_head(a) == self.find_head(b)
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let a_head = self.find_head(a);
        let b_head = self.find_head(b);

        if a_head != b_head {
            // 找到数据量较大/小的集合代表节点
            let (big, small) = if self.sizes[a_head] >= self.sizes[b_head] {
                (a_head, b_head)
            } else {
                (b_head, a_head)
            };

            self.parents[small] = big;
            self.sizes[big] += self.sizes[small];
            self.sizes[small] = 0;
        }
    }

    pub fn find_head(&mut self, n: usize) -> usize {
        let mut cur = n;
        let mut h = 0;

        // 获取最顶层的head
        while cur != self.parents[cur] {
            self.path[h] = cur;
            h += 1;
            cur = self.parents[cur];
        }

        // 将当前的节点的代表节点设置为head
        for i in (0..h).rev() {
            self.parents[self.path[i]] = cur;
        }

        cur
    }
}

impl Default for UnionFind {
    fn default() -> Self {
        Self::new()
    }
}

pub fn exe() {
    let stdin = stdin();
    let mut uf = UnionFind::new();

    for line in stdin.lock().lines() {
        let ll = line.unwrap();
        let numbers: Vec<&str> = ll.split(' ').collect();

        let a = numbers[0].trim().parse::<usize>().unwrap_or(0);
        let b = numbers[1].trim().parse::<usize>().unwrap_or(0);

        if numbers.len() == 2 {
            uf.init(a);
        } else {
            let c: usize = numbers[2].trim().parse().unwrap_or(0);

            match a {
                1 => {
                    if uf.same_set(b, c) {
                        println!("Yes")
                    } else {
                        println!("No")
                    }
                }
                2 => uf.union(b, c),
                _ => {}
            }
        }
    }
}
