//! Simple module containing a wrapper struct that allows to sort arbitrary data
//! according to some related orderable value.

use std::cmp::Ordering;

/// Struct that allows to order T values using values of Ds as the comparators.
#[derive(Clone, Debug)]
pub struct OrdWrapper<T, K> {
    data: T,
    key: K,
}

impl<T, K> Into<(T, K)> for OrdWrapper<T, K> {
    fn into(self) -> (T, K) {
        (self.data, self.key)
    }
}

impl<T, K> OrdWrapper<T, K> {
    /// Create a new OrdWrapper with the given data and the given key.
    pub fn new(data: T, key: K) -> Self {
        OrdWrapper { data, key }
    }

    /// Getter for the data.
    pub fn data(&self) -> &T {
        &self.data
    }

    /// Getter for the key.
    pub fn key(&self) -> &K {
        &self.key
    }
}

impl<T, K: Eq> Eq for OrdWrapper<T, K> {}

impl<T, K: PartialEq> PartialEq for OrdWrapper<T, K> {
    fn eq(&self, other: &Self) -> bool {
        self.key == other.key
    }
}

impl<T, K: Ord> PartialOrd for OrdWrapper<T, K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T, K: Ord> Ord for OrdWrapper<T, K> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.key.cmp(&other.key)
    }
}
