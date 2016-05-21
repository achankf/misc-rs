mod counter;
mod disjoint_set;
pub mod idmap;
pub mod indexed_bag;

use std::collections::{HashMap, HashSet};
use std::{cmp, hash};

pub struct DisjointSet {
    counter: Counter<u32>,
    nodes: IdMap<u32, (u32, u8)>, // (id, rank)
}

pub trait Countable {
    fn zero() -> Self;
    fn one() -> Self;
    fn next(&self) -> Self;
}

pub struct Counter<T>
    where T: Countable
{
    next: T,
    recycled: HashSet<T>,
}

pub struct IndexedBag<IdT, T>
    where IdT: Countable + hash::Hash + PartialOrd + cmp::Eq + Clone
{
    counter: Counter<IdT>,
    map: IdMap<IdT, T>,
}

pub type IdMap<IdT, T> = HashMap<IdT, T>;

impl Countable for u32 {
    fn zero() -> u32 {
        0
    }

    fn one() -> u32 {
        1
    }

    fn next(&self) -> u32 {
        self + 1
    }
}
