use std::collections::HashMap;

use std::hash::Hash;

/// Build a `HashMap` from the keys in the iterator to the number of its
/// occurences.
pub fn build_hashmap_counter<K, I>(it: I) -> HashMap<K, u64>
where
    K: Eq + Hash,
    I: Iterator<Item = K>,
{
    let mut map = HashMap::new();

    for k in it {
        *map.entry(k).or_insert(0) += 1;
    }

    map
}
