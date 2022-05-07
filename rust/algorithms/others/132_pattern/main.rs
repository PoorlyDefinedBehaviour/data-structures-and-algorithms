//! 456. 132 Pattern
//!
//! Given an array of n integers nums, a 132 pattern is a subsequence of three integers nums[i], nums[j] and nums[k] such that i < j < k and nums[i] < nums[k] < nums[j].
//!
//! Return true if there is a 132 pattern in nums, otherwise, return false.
//!
//! Example 1:
//!
//! Input: nums = [1,2,3,4]
//! Output: false
//! Explanation: There is no 132 pattern in the sequence.
//!
//! Example 2:
//!
//! Input: nums = [3,1,4,2]
//! Output: true
//! Explanation: There is a 132 pattern in the sequence: [1, 4, 2].
//!
//! Example 3:
//!
//! Input: nums = [-1,3,2,0]
//! Output: true
//! Explanation: There are three 132 patterns in the sequence: [-1, 3, 2], [-1, 3, 0] and [-1, 2, 0].
//!
//! Constraints:
//!
//!     n == nums.length
//!     1 <= n <= 2 * 105
//!     -109 <= nums[i] <= 109

use std::collections::VecDeque;

/// time O(n)
/// space O(n)
pub fn find132pattern(nums: Vec<i32>) -> bool {
    if nums.is_empty() {
        return false;
    }

    let mut stack: VecDeque<(i32, i32)> = VecDeque::new();

    let mut current_min = nums[0];

    for num in nums.into_iter() {
        while !stack.is_empty() && num >= stack.back().unwrap().0 {
            let _ = stack.pop_back();
        }

        if !stack.is_empty() && num > stack.back().unwrap().1 {
            return true;
        }

        stack.push_back((num, current_min));
        current_min = std::cmp::min(current_min, num);
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find132pattern() {
        let tests = vec![
            (vec![1, 2, 3, 4], false),
            (vec![3, 1, 4, 2], true),
            (vec![-1, 3, 2, 0], true),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, find132pattern(input));
        }
    }
}
