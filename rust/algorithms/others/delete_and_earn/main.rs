// 740. Delete and Earn
//
// You are given an integer array nums.
// You want to maximize the number of points you get by performing the following operation any number of times:
//
// Pick any nums[i] and delete it to earn nums[i] points.
// Afterwards, you must delete every element equal to nums[i] - 1 and every element equal to nums[i] + 1.
//
// Return the maximum number of points you can earn by applying the above operation some number of times.
//
// Example 1:
//
// Input: nums = [3,4,2]
// Output: 6
// Explanation: You can perform the following operations:
// - Delete 4 to earn 4 points. Consequently, 3 is also deleted. nums = [2].
// - Delete 2 to earn 2 points. nums = [].
// You earn a total of 6 points.
//
// Example 2:
//
// Input: nums = [2,2,3,3,3,4]
// Output: 9
// Explanation: You can perform the following operations:
// - Delete a 3 to earn 3 points. All 2's and 4's are also deleted. nums = [3,3].
// - Delete a 3 again to earn 3 points. nums = [3].
// - Delete a 3 once more to earn 3 points. nums = [].
// You earn a total of 9 points.
//
// Constraints:
//
//     1 <= nums.length <= 2 * 104
//     1 <= nums[i] <= 104

// time O(n log n)
// space O(n)
pub fn delete_and_earn(mut nums: Vec<i32>) -> i32 {
    // sort_unstable is faster for i32 without noticeable difference in the sorting order.
    nums.sort_unstable();

    let mut dp = vec![0; nums.len()];

    for i in 0..nums.len() {
        // If the previous value is equal to the current number - 1, we ignore the previous value.
        let previous = if i == 0 || nums[i] - 1 == nums[i - 1] {
            0
        } else {
            // If it is not equal to the current number - 1, we will use the previous value.
            dp[i - 1]
        };

        // Given [a, b, c] and we are currently looking at c,
        // if a is equal to c - 1, we won't use a.
        let previous_previous = if i < 2 || nums[i] - 1 == nums[i - 2] {
            0
        } else {
            // if a is not equal to c - 1, we will use it.
            dp[i - 2]
        };

        // We are trying to maximize the amount we can earn, so we have two choices:
        //
        // If the previous value is the same as the current one, we can just can take it.
        //
        // If the previous value is not the same as the current one,
        // we need to take the previous value or the value that comes before it,
        // we will take whichever is greater.
        let value = if previous == previous_previous {
            previous + previous_previous
        } else {
            std::cmp::max(previous, previous_previous)
        };

        // This is a dynamic programming problem, to solve it,
        // we can just look at each number and ask the question:
        // What is greatest amount i can get if i take this number?
        //
        // Note: We are caching the amount we can get for each number,
        // but we actually just need the last 2 numbers.
        dp[i] = nums[i] + value;
    }

    dp.iter().max().cloned().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_and_earn() {
        let tests = vec![(vec![3, 4, 2], 6), (vec![2, 2, 3, 3, 3, 4], 9)];

        for (input, expected) in tests {
            assert_eq!(expected, delete_and_earn(input));
        }
    }
}
