//! Blazingly fast bogosort because Rust.
extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

/// Blazingly fast bogosort because Rust.
pub fn bogosort(mut vector: Vec<i32>) -> Vec<i32> {
    while !is_sorted(&vector) {
        vector.shuffle(&mut thread_rng())
    }
    vector
}

fn is_sorted(vector: &Vec<i32>) -> bool {
    vector.windows(2).all(|window| window[0] <= window[1])
}

#[cfg(test)]
mod tests {
    use crate::bogosort;

    #[test]
    fn it_works() {
        let vector = vec![5, 3, 1, 2, 4];
        let sorted_vector = bogosort(vector);
        assert_eq!(sorted_vector, vec![1, 2, 3, 4, 5]);
    }
}
