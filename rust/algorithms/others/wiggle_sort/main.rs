//! 508 · Wiggle Sort
//!
//! Description
//!
//! Given an unsorted array nums, reorder it in-place such that
//!
//! nums[0] <= nums[1] >= nums[2] <= nums[3]....
//!
//! Please sort the array in place and do not define additional arrays.
//! Example
//!
//! Example 1:
//!
//! Input: [3, 5, 2, 1, 6, 4]
//!
//! Output: [1, 6, 2, 5, 3, 4]
//!
//! Explanation: This question may have multiple answers, and [2, 6, 1, 5, 3, 4] is also ok.
//!
//! Example 2:
//!
//! Input: [1, 2, 3, 4]
//!
//! Output: [1, 4, 2, 3]

/// time O(n)
/// space O(1)
fn wiggle_sort(nums: &mut [i32]) {
    for i in 0..nums.len() - 1 {
        if (i & 1 == 0 && nums[i] > nums[i + 1]) || (i & 1 == 1 && nums[i] < nums[i + 1]) {
            let temp = nums[i];
            nums[i] = nums[i + 1];
            nums[i + 1] = temp;
        }
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wiggle_sort() {
        // NOTE: this problem seems perfect for property based testing.

        let tests = vec![vec![3, 5, 2, 1, 6, 4], vec![1, 2, 3, 4]];

        for mut input in tests {
            wiggle_sort(&mut input);

            for i in 1..input.len() {
                if i & 1 == 1 {
                    assert!(input[i - 1] <= input[i]);
                } else {
                    assert!(input[i - 1] >= input[i]);
                }
            }
        }
    }
}
