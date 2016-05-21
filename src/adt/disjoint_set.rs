use super::{Counter, DisjointSet, IdMap};

impl DisjointSet {
    pub fn new() -> Self {
        DisjointSet {
            counter: Counter::new(),
            nodes: IdMap::new(),
        }
    }

    pub fn add(&mut self) -> u32 {
        let ret = self.counter.next();
        self.nodes.insert(ret, (ret, 0));
        ret
    }

    pub fn union(&mut self, left: u32, right: u32) -> u32 {
        let (l_root, l_rank) = self.find(left);
        let (r_root, r_rank) = self.find(right);

        if l_root == r_root {
            l_root // either left or right is fine
        } else if l_rank < r_rank {
            self.nodes.insert(l_root, (r_root, l_rank));
            l_root
        } else if l_rank > r_rank {
            self.nodes.insert(r_root, (l_root, r_rank));
            r_root
        } else {
            self.nodes.insert(l_root, (r_root, r_rank + 1));
            l_root
        }
    }

    fn find(&mut self, id: u32) -> (u32, u8) {
        let mut prev = id;
        let mut path = Vec::with_capacity(4); // i.e. "large enough for practical purposes"
        loop {
            let (parent, rank) = *self.nodes.get(&prev).unwrap();
            if prev == parent {
                let ret = (parent, rank);
                for node in path {
                    // path compression
                    self.nodes.insert(node, ret);
                }
                return ret;
            }
            path.push(prev);
            prev = parent;
        }
    }

    pub fn is_same_set(&mut self, left: u32, right: u32) -> bool {
        self.find(left).0 == self.find(right).0
    }
}

#[test]
pub fn test() {
    use std::iter;

    let mut sets = DisjointSet::new();
    for _ in iter::repeat(0).take(5) {
        let ret = sets.add();
        assert!(sets.is_same_set(ret, ret));
    }

    sets.union(0, 1);
    assert!(sets.is_same_set(0, 1));
    assert!(!sets.is_same_set(0, 2));
    assert!(!sets.is_same_set(0, 3));
    assert!(!sets.is_same_set(0, 4));
    assert!(!sets.is_same_set(1, 2));
    assert!(!sets.is_same_set(1, 3));
    assert!(!sets.is_same_set(1, 4));

    sets.union(3, 4);
    assert!(sets.is_same_set(3, 4));
    assert!(!sets.is_same_set(3, 0));
    assert!(!sets.is_same_set(3, 1));
    assert!(!sets.is_same_set(3, 2));
    assert!(!sets.is_same_set(4, 0));
    assert!(!sets.is_same_set(4, 1));
    assert!(!sets.is_same_set(4, 2));

    // 3 is alone
    assert!(!sets.is_same_set(2, 0));
    assert!(!sets.is_same_set(2, 1));
    assert!(!sets.is_same_set(2, 3));
    assert!(!sets.is_same_set(2, 4));

    sets.union(1, 4);
    assert!(sets.is_same_set(0, 1));
    assert!(sets.is_same_set(0, 3));
    assert!(sets.is_same_set(0, 4));
    assert!(sets.is_same_set(1, 0));
    assert!(sets.is_same_set(1, 3));
    assert!(sets.is_same_set(1, 4));
    assert!(sets.is_same_set(3, 0));
    assert!(sets.is_same_set(3, 1));
    assert!(sets.is_same_set(3, 4));
    assert!(sets.is_same_set(4, 0));
    assert!(sets.is_same_set(4, 1));
    assert!(sets.is_same_set(4, 3));
    assert!(!sets.is_same_set(2, 0));
    assert!(!sets.is_same_set(2, 1));
    assert!(!sets.is_same_set(2, 3));
    assert!(!sets.is_same_set(2, 0));
    assert!(!sets.is_same_set(2, 4));
}
