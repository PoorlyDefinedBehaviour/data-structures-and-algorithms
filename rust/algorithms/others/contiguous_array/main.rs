// Given a binary array nums,
// return the maximum length of a contiguous subarray with an equal number of 0 and 1.
//
// Example 1:
//
// Input: nums = [0,1]
// Output: 2
// Explanation: [0, 1] is the longest contiguous subarray with an equal number of 0 and 1.
// Example 2:
//
// Input: nums = [0,1,0]
// Output: 2
// Explanation: [0, 1] (or [1, 0]) is a longest contiguous subarray with equal number of 0 and 1.
// Example 3:
//
// Input: nums = [0,1,1,1,1,1,0,0,0]
// Output: 6
// Explanation: [1,1,1,0,0,0] is the longest contiguous subarray with equal number of 0 and 1.
//
// Constraints:
//
// 1 <= nums.length <= 105
// nums[i] is either 0 or 1.

use std::collections::HashMap;

// time O(n)
// space O(n)
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    let mut prefix: HashMap<i32, i32> = HashMap::new();
    prefix.insert(0, -1);
    let mut sum = 0;
    let mut max_length = 0;

    for (i, num) in nums.into_iter().enumerate() {
        sum += if num == 1 { 1 } else { -1 };
        if let Some(j) = prefix.get(&sum) {
            max_length = std::cmp::max(max_length, i as i32 - j);
        } else {
            prefix.insert(sum, i as i32);
        }
    }

    max_length as i32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let cases = [
            (vec![0, 1], 2),
            (vec![0, 1, 0], 2),
            (vec![0, 1, 1, 1, 1, 1, 0, 0, 0], 6),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, find_max_length(input));
        }
    }
}
