//! 347. Top K Frequent Elements
//!
//! Given an integer array nums and an integer k, return the k most frequent elements. You may return the answer in any order.
//!
//! Example 1:
//!
//! Input: nums = [1,1,1,2,2,3], k = 2
//! Output: [1,2]
//!
//! Example 2:
//!
//! Input: nums = [1], k = 1
//! Output: [1]
//!
//! Constraints:
//!
//!     1 <= nums.length <= 105
//!     k is in the range [1, the number of unique elements in the array].
//!     It is guaranteed that the answer is unique.

use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    count: usize,
    num: i32,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.count.partial_cmp(&other.count)
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        self.count.cmp(&other.count)
    }
}

/// time O(k log n) where n = nums.len().
/// space O(n) where n = nums.len();
pub fn top_k_frequent(nums: Vec<i32>, k: usize) -> Vec<i32> {
    let mut occurrences = HashMap::new();

    for num in nums.into_iter() {
        let entry = occurrences.entry(num).or_insert(Entry { count: 0, num });

        entry.count += 1;
    }

    // This is O(n).
    let heap = BinaryHeap::from_iter(occurrences.into_values());

    heap.into_iter().take(k).map(|entry| entry.num).collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let tests = vec![(vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2])];

        for (list, k, expected) in tests {
            assert_eq!(expected, top_k_frequent(list, k));
        }
    }
}
