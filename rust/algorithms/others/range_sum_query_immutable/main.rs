// Given an integer array nums, handle multiple queries of the following type:
//
// Calculate the sum of the elements of nums between indices
// left and right inclusive where left <= right.
// Implement the NumArray class:
//
// NumArray(int[] nums) Initializes the object with the integer array nums.
// int sumRange(int left, int right)
// Returns the sum of the elements of nums between indices
// left and right inclusive (i.e. nums[left] + nums[left + 1] + ... + nums[right]).
//
// Example 1:
//
// Input
// ["NumArray", "sumRange", "sumRange", "sumRange"]
// [[[-2, 0, 3, -5, 2, -1]], [0, 2], [2, 5], [0, 5]]
// Output
// [null, 1, -1, -3]
//
// Explanation
// NumArray numArray = new NumArray([-2, 0, 3, -5, 2, -1]);
// numArray.sumRange(0, 2); // return (-2) + 0 + 3 = 1
// numArray.sumRange(2, 5); // return 3 + (-5) + 2 + (-1) = -1
// numArray.sumRange(0, 5); // return (-2) + 0 + 3 + (-5) + 2 + (-1) = -3
//
// Constraints:
//
// 1 <= nums.length <= 104
// -105 <= nums[i] <= 105
// 0 <= left <= right < nums.length
// At most 104 calls will be made to sumRange.

#[derive(Debug)]
struct NumArray {
    prefix_sum: Vec<i32>,
}

impl NumArray {
    // time O(n)
    // space O(1)
    fn new(mut nums: Vec<i32>) -> Self {
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            nums[i] = sum;
        }
        Self { prefix_sum: nums }
    }

    // time O(1)
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        assert!(left <= right);
        let mut sum = self.prefix_sum[right as usize];
        if left > 0 {
            sum -= self.prefix_sum[left as usize - 1];
        }
        sum
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);
        assert_eq!(1, array.sum_range(0, 2));
        assert_eq!(-1, array.sum_range(2, 5));
        assert_eq!(-3, array.sum_range(0, 5));
    }
}
