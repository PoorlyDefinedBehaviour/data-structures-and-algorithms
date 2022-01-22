//! 448. Find All Numbers Disappeared in an Array
//!
//! Given an array nums of n integers where nums[i] is in the range [1, n],
//! return an array of all the integers in the range [1, n] that do not appear in nums.
//!
//! Example 1:
//!
//! Input: nums = [4,3,2,7,8,2,3,1]
//! Output: [5,6]
//!
//! Example 2:
//!
//! Input: nums = [1,1]
//! Output: [2]
//!
//! Constraints:
//!
//!     n == nums.length
//!     1 <= n <= 105
//!     1 <= nums[i] <= n
//!
//! Follow up: Could you do it without extra space and in O(n) runtime? You may assume the returned list does not count as extra space.

use std::collections::HashSet;

/// time O(n) because we visit every number in `nums`.
/// space O(n) because we create a set from `nums`.
pub fn find_disappeared_numbers_1(nums: Vec<i32>) -> Vec<i32> {
    let nums_len = nums.len();

    let nums: HashSet<i32> = nums.into_iter().collect();

    let mut disappeared_numbers = Vec::new();

    for i in 1..=(nums_len as i32) {
        if !nums.contains(&i) {
            disappeared_numbers.push(i)
        }
    }

    disappeared_numbers
}

/// time O(n) because we visit each number in `nums`.
/// space O(1) because we modify `nums` to mark which numbers
/// in the range [1, n] appear in `nums`.
/// Note that the return value is not counted as extra memory
/// because it is required.
pub fn find_disappeared_numbers_2(mut nums: Vec<i32>) -> Vec<i32> {
    for i in 0..nums.len() {
        let num = nums[i].abs();

        let index = (num - 1) as usize;

        nums[index] = -(nums[index].abs());
    }

    (0..nums.len())
        .filter(|&i| nums[i] > -1)
        .map(|i| (i + 1) as i32)
        .collect()
}

fn main() {
    find_disappeared_numbers_2(vec![4, 3, 2, 7, 8, 2, 3, 1]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_disappeared_numbers() {
        let tests = vec![
            (vec![4, 3, 2, 7, 8, 2, 3, 1], vec![5, 6]),
            (vec![1, 1], vec![2]),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, find_disappeared_numbers_1(input.clone()));
            assert_eq!(expected, find_disappeared_numbers_2(input));
        }
    }
}
