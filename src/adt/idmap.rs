use std::collections::hash_map::{Iter as HIter, IterMut as HIterMut, Keys as HKeys,
                                 Values as HValues};

pub type Iter<'a, IdT, T> = HIter<'a, IdT, T>;
pub type IterMut<'a, IdT, T> = HIterMut<'a, IdT, T>;
pub type Keys<'a, IdT, T> = HKeys<'a, IdT, T>;
pub type Values<'a, IdT, T> = HValues<'a, IdT, T>;
