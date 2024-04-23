use std::collections::HashSet;
use std::hash::Hash;

pub fn all_unique<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut set = HashSet::new();
    iter.into_iter().all(|x| set.insert(x))
}
