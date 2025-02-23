use crate::Error;
use rand::{rngs::StdRng, Rng};
use std::collections::BTreeMap;
use std::hash::Hash;

/// Shuffles elements of a Vec
pub(crate) fn shuffle<K>(rng: &mut StdRng, mut vec: Vec<K>) -> Vec<K> {
    if vec.len() < 2 {
        return vec;
    }

    let mut shuffled: Vec<K> = Vec::new();

    while !vec.is_empty() {
        shuffled.push(vec.remove(rng.gen_range(0..vec.len())));
    }

    shuffled
}

/// Randomly pair together entries from a Vec and return the resultant Map.
pub(crate) fn pair_randomly<T>(rng: &mut StdRng, vec: Vec<T>) -> crate::Result<BTreeMap<T, T>>
where
    T: Eq + Hash + Clone + Ord,
{
    if vec.len() % 2 != 0 {
        return Err(Error::internal("Cannot pair entries in Vec with odd length."));
    }

    let shuffled = shuffle(rng, vec);
    let mut map: BTreeMap<T, T> = Default::default();

    let mut i = 0;
    while i < shuffled.len() {
        let x = shuffled[i].clone();
        let y = shuffled[i + 1].clone();

        map.insert(x.clone(), y.clone());
        map.insert(y, x);

        i += 2;
    }

    Ok(map)
}
