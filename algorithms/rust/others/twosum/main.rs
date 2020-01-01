use std::collections::HashMap;

#[derive(Debug)]
struct Pair<T, U> {
  first: T,
  second: U,
}

/**
 * time O(n)
 * space O(n)
 */
fn two_sum(numbers: &Vec<i32>, target: i32) -> Pair<i32, i32> {
  let mut cache = HashMap::new();

  for (index, number) in numbers.iter().enumerate() {
    cache.insert(number, index);

    let difference = target - number;
    match cache.get(&difference) {
      Some(value) => {
        if *value != index {
          return Pair {
            first: *value as i32,
            second: index as i32,
          };
        }
      }
      None => {}
    }
  }

  return Pair {
    first: -1,
    second: -1,
  };
}

fn main() {
  /*Given an array of integers, return indices of the two numbers such that they add up to a specific target.
  You may assume that each input would have exactly one solution, and you may not use the same element twice.

  Example:
  Given nums = [2, 7, 11, 15], target = 9,
  Because nums[0] + nums[1] = 2 + 7 = 9,
  return [0, 1].
  */
  println!("{:?}", two_sum(&vec![2, 7, 11, 15], 9));
}
