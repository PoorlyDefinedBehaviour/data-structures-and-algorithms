use std::collections::HashMap;

/// The alternating sum of a 0-indexed array is defined as
/// the sum of the elements at even indices minus the sum of the elements at odd indices.
///
/// For example, the alternating sum of [4,2,5,3] is (4 + 5) - (2 + 3) = 4.
/// Given an array nums, return the maximum alternating sum of any subsequence of nums
/// (after reindexing the elements of the subsequence).
///
/// A subsequence of an array is a new array generated from the original array by deleting some elements
/// (possibly none) without changing the remaining elements' relative order.
/// For example, [2,7,4] is a subsequence of [4,2,3,7,2,1,4] (the underlined elements), while [2,4,2] is not.
///
/// Example 1:
///
/// Input: nums = [4,2,5,3]
/// Output: 7
/// Explanation: It is optimal to choose the subsequence [4,2,5] with alternating sum (4 + 5) - 2 = 7.
/// Example 2:
///
/// Input: nums = [5,6,7,8]
/// Output: 8
/// Explanation: It is optimal to choose the subsequence [8] with alternating sum 8.
/// Example 3:
///
/// Input: nums = [6,2,1,2,4,5]
/// Output: 10
/// Explanation: It is optimal to choose the subsequence [6,1,5] with alternating sum (6 + 5) - 1 = 10.
///
/// Constraints:
///
/// 1 <= nums.length <= 105
/// 1 <= nums[i] <= 105

// time O(n)
// space O(n)
fn max_alternating_sum(nums: Vec<i32>) -> i32 {
    fn go(nums: &[i32], cache: &mut HashMap<(usize, bool), i32>, i: usize, even: bool) -> i32 {
        if i >= nums.len() {
            return 0;
        }
        if let Some(sum) = cache.get(&(i, even)) {
            return *sum;
        }

        let num = if even { nums[i] } else { -nums[i] };

        let max_sum = std::cmp::max(
            num + go(nums, cache, i + 1, !even),
            go(nums, cache, i + 1, even),
        );

        cache.insert((i, even), max_sum);
        max_sum
    }

    go(&nums, &mut HashMap::new(), 0, true)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_alternating_sum() {
        let cases = [
            (vec![4, 2, 5, 3], 7),
            (vec![5, 6, 7, 8], 8),
            (vec![6, 2, 1, 2, 4, 5], 10),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, max_alternating_sum(input));
        }
    }
}
