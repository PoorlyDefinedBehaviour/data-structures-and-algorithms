/*
* time O(2n) -> O(n)
* space O(2n) -> O(n)
**/
fn find_numbers(numbers: Vec<i32>) -> i32 {
  return numbers
    .iter()
    .map(|number| number.to_string().len())
    .filter(|length| length & 1 == 0)
    .count() as i32;
}

fn main() {
  /*
  Given an array nums of integers, return how many of them contain an even number of digits.

  Example:

  Input: nums = [12,345,2,6,7896]
  Output: 2
  Explanation:
  12 contains 2 digits (even number of digits).
  345 contains 3 digits (odd number of digits).
  2 contains 1 digit (odd number of digits).
  6 contains 1 digit (odd number of digits).
  7896 contains 4 digits (even number of digits).
  Therefore only 12 and 7896 contain an even number of digits.
  */
  println!("{:?}", find_numbers(vec![12, 345, 2, 6, 7896]));
}
