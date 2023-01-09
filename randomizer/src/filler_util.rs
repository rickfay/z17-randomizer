use rand::{rngs::StdRng, Rng};

/// Shuffles elements of a Vec
pub(crate) fn shuffle<K>(mut vec: Vec<K>, rng: &mut StdRng) -> Vec<K> {
    let mut shuffled: Vec<K> = Vec::new();

    while !vec.is_empty() {
        shuffled.push(vec.remove(rng.gen_range(0..vec.len())));
    }

    shuffled
}
