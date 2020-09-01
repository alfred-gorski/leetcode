use std::cmp::Ordering;
use std::collections::{BTreeSet, HashMap};

#[derive(Eq, Copy, Clone, PartialEq, Debug)]
struct Node {
    cnt: usize,
    time: usize,
    key: i32,
    value: i32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cnt
            .cmp(&other.cnt)
            .then_with(|| self.time.cmp(&other.time))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
#[allow(dead_code)]
pub struct LFUCache {
    capacity: i32,
    time: usize,
    key_table: HashMap<i32, Node>,
    s: BTreeSet<Node>,
}
#[allow(dead_code)]
impl LFUCache {
    fn new(capacity: i32) -> Self {
        Self {
            capacity,
            time: 0,
            key_table: HashMap::new(),
            s: BTreeSet::new(),
        }
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.capacity == 0 {
            return -1;
        }
        if let Some(res) = self.key_table.get_mut(&key) {
            self.s.remove(res);
            self.time += 1;
            res.cnt += 1;
            res.time = self.time;
            self.s.insert(*res);
            return res.value;
        } else {
            return -1;
        }
    }
    fn put(&mut self, key: i32, value: i32) {
        if self.capacity == 0 {
            return;
        }
        if let Some(res) = self.key_table.get_mut(&key) {
            self.s.remove(res);
            self.time += 1;
            res.cnt += 1;
            res.time = self.time;
            res.value = value;
            self.s.insert(*res);
        } else {
            if self.key_table.len() == self.capacity as usize {
                let first = self.s.iter().next().unwrap().clone();
                self.key_table.remove(&first.key);
                self.s.remove(&first);
            }
            self.time += 1;
            let cache = Node {
                cnt: 1,
                time: self.time,
                key,
                value,
            };
            self.key_table.insert(key, cache);
            self.s.insert(cache);
        }
    }
}
