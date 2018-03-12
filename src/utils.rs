use std::collections::HashMap;

use std::hash::Hash;

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
