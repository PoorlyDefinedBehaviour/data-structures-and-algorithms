//! 704. Binary Search
//!
//! Given an array of integers nums which is sorted in ascending order, and an integer target,
//! write a function to search target in nums. If target exists, then return its index. Otherwise, return -1.
//!
//! You must write an algorithm with O(log n) runtime complexity.
//!
//! Example 1:
//!
//! Input: nums = [-1,0,3,5,9,12], target = 9
//! Output: 4
//! Explanation: 9 exists in nums and its index is 4
//!
//! Example 2:
//!
//! Input: nums = [-1,0,3,5,9,12], target = 2
//! Output: -1
//! Explanation: 2 does not exist in nums so return -1
//!
//! Constraints:
//!
//!     1 <= nums.length <= 104
//!     -104 < nums[i], target < 104
//!     All the integers in nums are unique.
//!     nums is sorted in ascending order.

pub fn search(nums: &[i32], target: i32) -> Option<usize> {
    if nums.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = nums.len();

    while right >= left {
        let middle = (left + right) / 2;

        let value = nums[middle];

        if value == target {
            return Some(middle);
        }

        if target < value {
            right = middle - 1;
        } else {
            left = middle + 1;
        }
    }

    None
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let tests = vec![
            (vec![], 1, None),
            (vec![1], 1, Some(0)),
            (vec![1, 2], 1, Some(0)),
            (vec![1, 2], 2, Some(1)),
            (vec![1, 2, 3], 3, Some(2)),
            (vec![-1, 0, 3, 5, 9, 12], 9, Some(4)),
            (vec![-1, 0, 3, 5, 9, 12], 2, None),
        ];

        for (input, target, expected) in tests {
            assert_eq!(expected, search(&input, target))
        }
    }
}
