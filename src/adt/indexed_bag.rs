use super::{Countable, Counter, IdMap, IndexedBag};
use adt::idmap::{Iter as IdMapIter, IterMut as IdMapIterMut, Keys as IdMapKeys,
                 Values as IdMapValues};
use std::{cmp, fmt, hash};

pub type Iter<'a, IdT, T> = IdMapIter<'a, IdT, T>;
pub type IterMut<'a, IdT, T> = IdMapIterMut<'a, IdT, T>;
pub type Keys<'a, IdT, T> = IdMapKeys<'a, IdT, T>;
pub type Values<'a, IdT, T> = IdMapValues<'a, IdT, T>;

impl<IdT, T> IndexedBag<IdT, T>
    where IdT: Countable + hash::Hash + PartialOrd + cmp::Eq + Clone + fmt::Display
{
    pub fn new() -> Self {
        IndexedBag {
            counter: Counter::new(),
            map: IdMap::new(),
        }
    }

    pub fn get(&self, id: IdT) -> Option<&T> {
        self.map.get(&id)
    }

    pub fn get_mut(&mut self, id: IdT) -> Option<&mut T> {
        self.map.get_mut(&id)
    }

    pub fn insert(&mut self, t: T) -> IdT {
        let id = self.counter.next();
        let prev = self.map.insert(id.clone(), t);
        assert!(prev.is_none(), "duplicate id: {}", id.clone());
        id.clone()
    }

    pub fn remove(&mut self, id: IdT) {
        self.map.remove(&id);
        self.counter.recycle(id);
    }

    pub fn iter(&self) -> Iter<IdT, T> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<IdT, T> {
        self.map.iter_mut()
    }

    pub fn keys(&self) -> Keys<IdT, T> {
        self.map.keys()
    }

    pub fn values(&self) -> Values<IdT, T> {
        self.map.values()
    }

    pub fn existed(&self, id: IdT) -> bool {
        self.counter.is_leased(id)
    }
}
