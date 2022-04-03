//! 217. Contains Duplicate
//!
//! Given an integer array nums, return true if any value appears at least twice in the array, and return false if every element is distinct.
//!
//! Example 1:
//!
//! Input: nums = [1,2,3,1]
//! Output: true
//!
//! Example 2:
//!
//! Input: nums = [1,2,3,4]
//! Output: false
//!
//! Example 3:
//!
//! Input: nums = [1,1,1,3,3,4,3,2,4,2]
//! Output: true
//!
//! Constraints:
//!
//!     1 <= nums.length <= 105
//!     -109 <= nums[i] <= 109

use std::collections::HashSet;

/// time O(n) where n = nums.len()
/// space O(n) where n = nums.len()
pub fn contains_duplicate(nums: &[i32]) -> bool {
    let mut seen = HashSet::new();

    for num in nums {
        if seen.contains(num) {
            return true;
        }

        seen.insert(num);
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let tests = vec![
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, contains_duplicate(&input));
        }
    }
}
