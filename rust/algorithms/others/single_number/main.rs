/// 136. Single Number
///
/// Given a non-empty array of integers nums, every element appears twice except for one. Find that single one.
///
/// You must implement a solution with a linear runtime complexity and use only constant extra space.
/// ///
/// Example 1:
///
/// Input: nums = [2,2,1]
/// Output: 1
///
/// Example 2:
///
/// Input: nums = [4,1,2,1,2]
/// Output: 4
///
/// Example 3:
///
/// Input: nums = [1]
/// Output: 1
///
/// Constraints:
///
///     1 <= nums.length <= 3 * 104
///     -3 * 104 <= nums[i] <= 3 * 104
///     Each element in the array appears twice except for one element which appears only once.

/// time O(n)
/// space O(1)
pub fn single_number(nums: Vec<i32>) -> i32 {
  let mut number = 0;

  for num in nums {
    // xoring a number with itself equals 0
    // and xoring a number with 0 equals the number
    // because of that, `number` will be the only number
    // that appears only once in `nums` after we xored
    // every number in `nums`.
    //
    // This works because each element in the vector appears twice
    // while only one element appears once.
    //
    // Given nums = [4, 1, 2, 1, 2]
    // 0 ^ 4 = 4 -- any number ^ 0 is equal to number
    // 4 ^ 1 = 5 -- first time 1 is xored
    // 5 ^ 2 = 7 -- first time 2 is xored
    // 7 ^ 1 = 6 -- second time 1 is xored so it cancels itself
    // 6 ^ 2 = 4 -- second time 2 is xored so it cancels itself
    number ^= num;
  }

  number
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_number() {
    let tests = vec![(vec![2, 2, 1], 1), (vec![4, 1, 2, 1, 2], 4), (vec![1], 1)];

    for (input, expected) in tests {
      assert_eq!(expected, single_number(input));
    }
  }
}
