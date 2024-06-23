#![allow(unused)]

use std::fmt::Debug;

trait Hashable {
    fn hash(&self) -> usize;
}

impl Hashable for String {
    fn hash(&self) -> usize {
        let mut res: usize = 5381;
        for i in self.bytes() {
            res = ((res << 5).wrapping_add(res)).wrapping_add(i as usize);
        }
        return res;
    }
}

#[derive(Default, Clone, Debug)]
struct Cell<Key, Val> {
    key: Key,
    val: Val,
    full: bool,
}

struct HashMap<Key, Val> {
    array: Vec<(Cell<Key, Val>)>,
    count_full: usize,
}

impl<Key: Default + Clone + Hashable + Debug + PartialEq, Val: Default + Clone + Debug> HashMap<Key, Val> {
    fn new() -> Self {
        const DEFAULT_CAP: usize = 11;
        Self {
            array: vec![Cell::<_, _>::default(); DEFAULT_CAP],
            count_full: 0,
        }
    }
    fn debug_dump(&self) {
        for cell in self.array.iter() {
            if cell.full {
                println!("{:?} -> {:?}", cell.key, cell.val);
            } else {
                println!("[ free ]");
            }
        }
    }
    fn extend(&mut self) {
        todo!();
    }


    fn insert(&mut self, key: Key, val: Val) {
        if let Some(existing_val) = self.get(&key) {
            *existing_val = val;
        } else {
            let mut idx: usize = key.hash() % self.array.len();
            while self.array[idx].full {
                idx = (idx + 1) % self.array.len();
            }
            self.array[idx].full = true;
            self.array[idx].key = key;
            self.array[idx].val = val;
            self.count_full += 1;
        }
    }
    fn peak(&self, key: &Key) -> Option<&Val> {
        let mut idx: usize = key.hash() % self.array.len();
        if !self.array[idx].full {
            return None;
        }
        return Some(&self.array[idx].val);
    }
    fn get(&mut self, key: &Key) -> Option<&mut Val> {
        let mut idx: usize = key.hash() % self.array.len();
        if self.count_full >= self.array.len() {
            self.extend()
        }
        assert!(self.count_full < self.array.len());

        while self.array[idx].full && self.array[idx].key != *key {
            idx = (idx + 1) % self.array.len();
        }
        if self.array[idx].full {
            assert_eq!(self.array[idx].key, *key);
            return Some(&mut self.array[idx].val);
        }
        return None;
    }
}

fn main() {}
