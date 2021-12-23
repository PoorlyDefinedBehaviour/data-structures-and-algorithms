// 283. Move Zeroes
//
// Given an integer array nums, move all 0's to the end of it while maintaining the relative order of the non-zero elements.
//
// Note that you must do this in-place without making a copy of the array.
//
// Example 1:
//
// Input: nums = [0,1,0,3,12]
// Output: [1,3,12,0,0]
//
// Example 2:
//
// Input: nums = [0]
// Output: [0]
//
// Constraints:
//
//     1 <= nums.length <= 104
//     -231 <= nums[i] <= 231 - 1
//
// Follow up: Could you minimize the total number of operations done?

// time O(n)
// space O(1)
pub fn move_zeroes(nums: &mut Vec<i32>) {
  let mut left = 0;

  for i in 0..nums.len() {
    if nums[i] > 0 {
      let temp = nums[left];
      nums[left] = nums[i];
      nums[i] = temp;
      left += 1;
    }
  }
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_move_zeroes() {
    let tests = vec![
      (vec![0, 1, 0, 3, 12], vec![1, 3, 12, 0, 0]),
      (vec![0], vec![0]),
    ];

    for (mut input, expected) in tests {
      move_zeroes(&mut input);
      assert_eq!(expected, input);
    }
  }
}
