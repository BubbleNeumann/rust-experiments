// 2022-08-26
// binary search

#![allow(dead_code)]

use rand::Rng;

fn generate_sorted_vec(len: usize) -> Vec<u32> {
    let mut vec = Vec::with_capacity(len);
    vec.push(rand::thread_rng().gen_range(0..100));

    for i in 1..len {
        let prev = vec[i - 1];
        vec.push(rand::thread_rng().gen_range(0..100) + prev);
    }
    vec
}

fn binary_search(vec: Vec<u32>, val: u32) -> usize {
    let mut left = 0;
    let mut right = vec.len();

    while vec[left] != val {
        let mid: usize = right / 2;
        if vec[mid] > val {
            right = mid;
        } else {
            left = mid;
        }
    }
    left
}

fn main() { }

#[cfg(test)]
mod tests {
    use crate::{binary_search, generate_sorted_vec};

    #[test]
    fn test_binary_search() {
        let vec = generate_sorted_vec(20);
        let index = 10;
        let val = vec[index];
        assert_eq!(binary_search(vec, val), index);
    }
}
