//! 128. Longest Consecutive Sequence
//!
//! Given an unsorted array of integers nums, return the length of the longest consecutive elements sequence.
//!
//! You must write an algorithm that runs in O(n) time.
//!
//! Example 1:
//!
//! Input: nums = [100,4,200,1,3,2]
//! Output: 4
//! Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
//!
//! Example 2:
//!
//! Input: nums = [0,3,7,2,5,8,4,6,0,1]
//! Output: 9
//!
//! Constraints:
//!
//!     0 <= nums.length <= 105
//!     -109 <= nums[i] <= 109

use std::collections::{HashMap, HashSet};

/// # Big O
///
/// time O(n) where n = nums.len()
/// space O(n) where n = nums.len()
///
/// # Solution
///
/// Build a directed graph and just follow it in the end to find the longest consecutive sequence.
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut graph = HashMap::new();

    for num in nums.into_iter() {
        if graph.contains_key(&(num - 1)) {
            graph.insert(num - 1, Some(num));
        }

        graph.insert(
            num,
            if graph.contains_key(&(num + 1)) {
                Some(num + 1)
            } else {
                None
            },
        );
    }

    let mut visited = HashSet::new();
    let mut max_sequence_len = 1;

    for (key, value) in graph.iter() {
        if value.is_none() || visited.contains(key) {
            continue;
        }

        let mut sequence_len = 0;

        visited.insert(*key);

        let mut current = key - 1;

        loop {
            sequence_len += 1;

            visited.insert(current);

            if graph.get(&current).is_none() {
                break;
            }

            current -= 1;
        }

        loop {
            sequence_len += 1;

            visited.insert(current);

            if graph.get(&current).is_none() {
                break;
            }

            current += 1;
        }

        max_sequence_len = std::cmp::max(max_sequence_len, sequence_len);
    }

    max_sequence_len
}

/// time O(n)
/// space O(n)
pub fn longest_consecutive_2(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.clone().into_iter());

    let mut max_sequence_len = 0;

    for num in nums.into_iter() {
        // If the number is not at the start of a sequence, skip it.
        if set.contains(&(num - 1)) {
            continue;
        }

        // Find the length of the sequence that starts at `num`.
        let mut sequence_len = 1;
        let mut current = num + 1;
        while set.contains(&current) {
            sequence_len += 1;
            current += 1;
        }

        max_sequence_len = std::cmp::max(max_sequence_len, sequence_len);
    }

    max_sequence_len
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let tests = vec![
            (vec![9, 1, 4, 7, 3, -1, 0, 5, 8, -1, 6], 7),
            (vec![100, 4, 200, 1, 3, 2], 4),
            (vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1], 9),
            (vec![], 0),
            (vec![1], 1),
            (vec![1, 3, 5], 1),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, longest_consecutive(input.clone()));
            assert_eq!(expected, longest_consecutive_2(input));
        }
    }
}
