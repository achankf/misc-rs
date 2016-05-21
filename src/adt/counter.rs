use std::collections::HashSet;
use std::{cmp, hash};
use super::{Countable, Counter};

impl<IdT> Counter<IdT>
    where IdT: Countable + hash::Hash + PartialOrd + cmp::Eq + Clone
{
    pub fn new() -> Counter<IdT> {
        Counter {
            next: IdT::zero(),
            recycled: HashSet::new(),
        }
    }

    pub fn next(&mut self) -> IdT {
        // get the next value from either self.recycled or self.next
        let ret = match self.recycled.iter().take(1).next() {
            Some(id) => (*id).clone(),
            None => {
                let ret = self.next.clone();
                self.next = self.next.next();
                return ret;
            }
        };

        self.recycled.remove(&ret);
        ret
    }

    pub fn recycle(&mut self, id: IdT) {
        assert!(id < self.next,
                "how come recycling an id that hasn't been issued");
        let retval = self.recycled.insert(id);
        assert!(retval,
                "should not recycle an id that has already been recycled");
    }

    pub fn is_leased(&self, id: IdT) -> bool {
        id < self.next && !self.recycled.contains(&id)
    }
}
