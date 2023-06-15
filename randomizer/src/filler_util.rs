use rand::{rngs::StdRng, Rng};

/// Shuffles elements of a Vec
pub(crate) fn shuffle<K>(rng: &mut StdRng, mut vec: Vec<K>) -> Vec<K> {
    let mut shuffled: Vec<K> = Vec::new();

    while !vec.is_empty() {
        shuffled.push(vec.remove(rng.gen_range(0..vec.len())));
    }

    shuffled
}
