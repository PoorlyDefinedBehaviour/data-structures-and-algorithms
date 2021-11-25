use std::collections::HashMap;

pub fn fib_no_cache(n: u32) -> u32 {
  if n == 0 {
    return 0;
  }

  if n == 1 {
    return 1;
  }

  fib_no_cache(n - 1) + fib_no_cache(n - 2)
}

pub fn fib_with_cache(n: u32) -> u32 {
  fn go(n: u32, cache: &mut HashMap<u32, u32>) -> u32 {
    if n == 0 {
      return 0;
    }

    if n == 1 {
      return 1;
    }

    match cache.get(&n) {
      None => {
        let value = go(n - 1, cache) + go(n - 2, cache);
        cache.insert(n, value);
        value
      }
      Some(value) => *value,
    }
  }

  let mut cache = HashMap::new();

  go(n, &mut cache)
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_fib() {
    let tests = vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3), (5, 5), (6, 8)];

    for (input, expected) in tests {
      assert_eq!(expected, fib_no_cache(input));
      assert_eq!(expected, fib_with_cache(input));
    }
  }
}
