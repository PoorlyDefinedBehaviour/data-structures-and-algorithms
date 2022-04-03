//! 238. Product of Array Except Self
//!
//! Given an integer array nums, return an array answer such that answer[i] is equal to the product of all the elements of nums except nums[i].
//!
//! The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//!
//! You must write an algorithm that runs in O(n) time and without using the division operation.
//!
//! Example 1:
//!
//! Input: nums = [1,2,3,4]
//! Output: [24,12,8,6]
//!
//! Example 2:
//!
//! Input: nums = [-1,1,0,-3,3]
//! Output: [0,0,9,0,0]
//!
//! Constraints:
//!
//!     2 <= nums.length <= 105
//!     -30 <= nums[i] <= 30
//!     The product of any prefix or suffix of nums is guaranteed to fit in a 32-bit integer.
//!
//! Follow up: Can you solve the problem in O(1) extra space complexity? (The output array does not count as extra space for space complexity analysis.)

/// Solution:
///
/// Compute the product of the elements to the left of nums[i]
/// and then compute the product to the elements to the right of nums[i].
/// The product of nums except nums[i] is the product of the elements
/// elements to the left os nums[i] * the product of the elements to the right
/// of nums[i].
///
/// time O(n)
/// space O(1) (problem ignores output vec)
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut out = vec![1; nums.len()];

    let mut prefix = 1;

    for (i, num) in nums.iter().enumerate() {
        out[i] = prefix;
        prefix *= num;
    }

    let mut postfix = 1;

    for (i, num) in nums.iter().enumerate().rev() {
        out[i] *= postfix;
        postfix *= num
    }

    out
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let tests = vec![
            (vec![1, 2, 3, 4], vec![24, 12, 8, 6]),
            (vec![-1, 1, 0, -3, 3], vec![0, 0, 9, 0, 0]),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, product_except_self(input));
        }
    }
}
