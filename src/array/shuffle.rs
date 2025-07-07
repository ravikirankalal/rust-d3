use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn shuffle<T>(array: &mut [T]) {
    array.shuffle(&mut thread_rng());
}
