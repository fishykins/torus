use std::{collections::HashSet, hash::Hash};
use crate::Tags;

pub struct ItemList<T>
where
    T: Hash + Eq + Clone,
{
    valid: HashSet<T>,
    invalid: HashSet<T>,
}

impl<T> ItemList<T>
where
    T: Hash + Eq + Clone + Default,
{
    pub fn new() -> Self {
        ItemList {
            valid: HashSet::new(),
            invalid: HashSet::new(),
        }
    }

    /// Adds a blocked item to the list. Returns false if the item was already in a list.
    pub fn add_blocked(&mut self, item: T) -> bool {
        let removed = self.valid.remove(&item);
        let added = self.invalid.insert(item.clone());
        !removed && !added
    }

    /// Adds a valid item to the list. Returns false if the item was already in a list.
    pub fn add_valid(&mut self, item: T) -> bool {
        let removed = self.invalid.remove(&item);
        let added = self.valid.insert(item.clone());
        !removed && !added
    }

    /// Checks to see if the given item is valid according to the list.
    pub fn check(&self, item: &T) -> bool {
        (!self.invalid.contains(&item) || self.invalid.len() == 0)
            && (self.valid.contains(&item) || self.valid.len() == 0)
    }

    pub fn check_all(&self, tags: &Tags<T>) -> bool {
        let mut iter = tags.clone().into_iter();
        iter.all(|tag: T| self.check(&tag))
    }
}
