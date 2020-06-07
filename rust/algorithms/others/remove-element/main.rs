/*
* time O(n)
* space O(1)
**/
fn remove_element(numbers: &mut Vec<i32>, value: i32) -> i32 {
  let mut i: usize = 0;

  for j in 0..numbers.len() {
    if numbers[j] != value {
      numbers[i] = numbers[j];
      i += 1;
    }
  }

  return i as i32;
}

fn main() {
  /*
  Given an array nums and a value val, remove all instances of that value in-place and return the new length.

  Do not allocate extra space for another array, you must do this by modifying the input array in-place with O(1) extra memory.

  The order of elements can be changed. It doesn't matter what you leave beyond the new length.

  Example 1:

  Given nums = [3,2,2,3], val = 3,

  Your function should return length = 2, with the first two elements of nums being 2.

  It doesn't matter what you leave beyond the returned length.
  */

  println!("{:?}", remove_element(&mut vec![3, 2, 2, 3], 3));
}
