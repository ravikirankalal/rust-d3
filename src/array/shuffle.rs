use rand::rng;
use rand::seq::SliceRandom;

pub fn shuffle<T>(array: &mut [T]) {
    array.shuffle(&mut rng());
}
