//! 209. Minimum Size Subarray Sum
//!
//! Given an array of positive integers nums and a positive integer target,
//! return the minimal length of a contiguous subarray [numsl, numsl+1, ..., numsr-1, numsr]
//! of which the sum is greater than or equal to target. If there is no such subarray, return 0 instead.
//!
//! Example 1:
//!
//! Input: target = 7, nums = [2,3,1,2,4,3]
//! Output: 2
//! Explanation: The subarray [4,3] has the minimal length under the problem constraint.
//!
//! Example 2:
//!
//! Input: target = 4, nums = [1,4,4]
//! Output: 1
//!
//! Example 3:
//!
//! Input: target = 11, nums = [1,1,1,1,1,1,1,1]
//! Output: 0
//!
//!
//!
//! Constraints:
//!
//!     1 <= target <= 109
//!     1 <= nums.length <= 105
//!     1 <= nums[i] <= 105

/// Sliding window problem.
///
/// time O(n).
/// space O(1).
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    let mut sub_array_length: i32 = 0;

    let mut sum = 0;

    let mut left = 0;
    let mut right = 0;

    while left < nums.len() {
        while sum < target && right < nums.len() {
            sum += nums[right];
            right += 1;
        }

        let current_sub_array_length = (right - left) as i32;

        if sum >= target && (sub_array_length == 0 || current_sub_array_length < sub_array_length) {
            sub_array_length = current_sub_array_length
        }

        sum -= nums[left];
        left += 1;
    }

    sub_array_length
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_sub_array_len() {
        let tests = vec![
            (7, vec![2, 3, 1, 2, 4, 3], 2),
            (4, vec![1, 4, 4], 1),
            (11, vec![1, 1, 1, 1, 1, 1, 1, 1], 0),
        ];

        for (target, nums, expected) in tests {
            assert_eq!(expected, min_sub_array_len(target, nums))
        }
    }
}
