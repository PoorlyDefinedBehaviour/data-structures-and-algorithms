// 977. Squares of a Sorted Array
//
// Given an integer array nums sorted in non-decreasing order,
// return an array of the squares of each number sorted in non-decreasing order.
//
// Example 1:
//
// Input: nums = [-4,-1,0,3,10]
// Output: [0,1,9,16,100]
// Explanation: After squaring, the array becomes [16,1,0,9,100].
// After sorting, it becomes [0,1,9,16,100].
//
// Example 2:
//
// Input: nums = [-7,-3,2,3,11]
// Output: [4,9,9,49,121]
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -104 <= nums[i] <= 104
//     nums is sorted in non-decreasing order.
//
// Follow up: Squaring each element and sorting the new array is very trivial,
// could you find an O(n) solution using a different approach?

// time O(n log n)
// space O(n)
pub fn sorted_squares_naive(nums: Vec<i32>) -> Vec<i32> {
  let mut squared_nums: Vec<_> = nums.iter().map(|x| x.pow(2)).collect();

  squared_nums.sort();

  squared_nums
}

// time O(n)
// space O(n)
//
// Take advantage of knowing that the vector is always sorted
// in ascending order and that any number squared is a positive number.
pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
  let mut squared_nums = vec![0; nums.len()];

  let mut right = nums.len() - 1;

  let mut left = 0;

  let mut i = nums.len();

  while left <= right {
    i -= 1;

    if nums[right].abs() > nums[left].abs() {
      squared_nums[i] = nums[right].pow(2);
      right -= 1;
    } else {
      squared_nums[i] = nums[left].pow(2);
      left += 1;
    }
  }

  squared_nums
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_sorted_squares() {
    let tests = vec![
      (vec![-4, -1, 0, 3, 10], vec![0, 1, 9, 16, 100]),
      (vec![-7, -3, 2, 3, 11], vec![4, 9, 9, 49, 121]),
    ];

    for (input, expected) in tests {
      assert_eq!(expected, sorted_squares_naive(input.clone()));
      assert_eq!(expected, sorted_squares(input));
    }
  }
}
