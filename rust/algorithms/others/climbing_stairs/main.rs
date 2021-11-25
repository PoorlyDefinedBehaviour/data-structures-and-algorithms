use std::collections::HashMap;

// 70. Climbing Stairs
//
// You are climbing a staircase. It takes n steps to reach the top.
//
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?
//
// Example 1:
//
// Input: n = 2
// Output: 2
// Explanation: There are two ways to climb to the top.
// 1. 1 step + 1 step
// 2. 2 steps
//
// Example 2:
//
// Input: n = 3
// Output: 3
// Explanation: There are three ways to climb to the top.
// 1. 1 step + 1 step + 1 step
// 2. 1 step + 2 steps
// 3. 2 steps + 1 step
//
// Constraints:
//
//     1 <= n <= 45

// time O(2^n)
// space O(n)
pub fn climb_stairs(n: i32) -> i32 {
  fn go(n: i32, target: i32) -> i32 {
    if n > target {
      return 0;
    }

    if n == target {
      return 1;
    }

    go(n + 1, target) + go(n + 2, target)
  }

  go(0, n)
}

// time O(n)
// space O(n)
pub fn climb_stairs_cache(n: i32) -> i32 {
  fn go(cache: &mut HashMap<i32, i32>, n: i32, target: i32) -> i32 {
    if n > target {
      return 0;
    }

    if n == target {
      return 1;
    }

    match cache.get(&n) {
      None => {
        let value = go(cache, n + 1, target) + go(cache, n + 2, target);
        cache.insert(n, value);
        value
      }
      Some(value) => *value,
    }
  }

  let mut cache = HashMap::new();

  go(&mut cache, 0, n)
}

// time O(n)
// space O(1)
pub fn climb_stairs_dp_bottom_up(n: i32) -> i32 {
  if n == 1 {
    return 1;
  }

  if n == 2 {
    return 2;
  }

  let mut one = 1;
  let mut two = 1;

  for _ in 0..n - 1 {
    let value = one + two;
    two = one;
    one = value;
  }

  one
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_climb_stairs() {
    let tests = vec![(2, 2), (3, 3), (5, 8)];

    for (input, expected) in tests {
      assert_eq!(expected, climb_stairs(input));
      assert_eq!(expected, climb_stairs_cache(input));
      assert_eq!(expected, climb_stairs_dp_bottom_up(input));
    }
  }
}
