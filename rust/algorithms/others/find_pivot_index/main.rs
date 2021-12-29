// 724. Find Pivot Index
// Given an array of integers nums, calculate the pivot index of this array.
//
// The pivot index is the index where the sum of all the numbers strictly
//to the left of the index is equal to the sum of all the numbers strictly to the index's right.
//
// If the index is on the left edge of the array, then the left sum is 0 because there are no elements to the left.
// This also applies to the right edge of the array.
//
// Return the leftmost pivot index. If no such index exists, return -1.
//
// Example 1:
//
// Input: nums = [1,7,3,6,5,6]
// Output: 3
// Explanation:
// The pivot index is 3.
// Left sum = nums[0] + nums[1] + nums[2] = 1 + 7 + 3 = 11
// Right sum = nums[4] + nums[5] = 5 + 6 = 11
//
// Example 2:
//
// Input: nums = [1,2,3]
// Output: -1
// Explanation:
// There is no index that satisfies the conditions in the problem statement.
//
// Example 3:
//
// Input: nums = [2,1,-1]
// Output: 0
// Explanation:
// The pivot index is 0.
// Left sum = 0 (no elements to the left of index 0)
// Right sum = nums[1] + nums[2] = 1 + -1 = 0
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -1000 <= nums[i] <= 1000

// time O(n)
// space O(1)
//
// This is a kind of a sliding window problem:
//
// We just sum numbers starting from the left,
// and check if the sum of the numbers to the right of the current number
// is equal to the sum of the numbers in the left.
// We know that the sum of the numbers to the right
// is the sum of all numbers - the sum of numbers to the left(which we have).
pub fn pivot_index(nums: Vec<i32>) -> i32 {
    let nums_sum: i32 = nums.iter().sum();

    let mut left_side_sum = 0;

    for (i, num) in nums.iter().enumerate() {
        let right_side_sum = nums_sum - left_side_sum;

        left_side_sum += num;

        if right_side_sum == left_side_sum {
            return i as i32;
        }
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pivot_index() {
        let tests = vec![
            (vec![1, 7, 3, 6, 5, 6], 3),
            (vec![1, 2, 3], -1),
            (vec![2, 1, -1], 0),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, pivot_index(input));
        }
    }
}
