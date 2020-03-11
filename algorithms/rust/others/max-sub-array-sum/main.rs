/**
 * time O(n)
 * space O(1)
 */
 fn max_sub_array_sum(numbers: Vec<i32>) -> i32 {
  let mut sum = numbers[0];
  let mut current_sum = numbers[0];

  for number in numbers {
    current_sum = std::cmp::max(current_sum + number, number);
    sum = std::cmp::max(current_sum, sum);
  }

  sum
}

fn main() {
  /*
  Given an integer array nums,
  find the contiguous subarray (containing at least one number)
  which has the largest sum and return its sum.

  Example:

  Input: [-2,1,-3,4,-1,2,1,-5,4],
  Output: 6
  Explanation: [4,-1,2,1] has the largest sum = 6.
  */
  println!(
    "{:?}",
    max_sub_array_sum(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4])
  );
}
