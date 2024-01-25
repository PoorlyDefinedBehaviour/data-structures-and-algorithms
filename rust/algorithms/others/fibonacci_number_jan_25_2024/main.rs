/// The Fibonacci numbers, commonly denoted F(n) form a sequence, called the Fibonacci sequence,
/// such that each number is the sum of the two preceding ones, starting from 0 and 1. That is,
///
/// F(0) = 0, F(1) = 1
/// F(n) = F(n - 1) + F(n - 2), for n > 1.
/// Given n, calculate F(n).
///
/// Example 1:
///
/// Input: n = 2
/// Output: 1
/// Explanation: F(2) = F(1) + F(0) = 1 + 0 = 1.
/// Example 2:
///
/// Input: n = 3
/// Output: 2
/// Explanation: F(3) = F(2) + F(1) = 1 + 1 = 2.
/// Example 3:
///
/// Input: n = 4
/// Output: 3
/// Explanation: F(4) = F(3) + F(2) = 2 + 1 = 3.
///
/// Constraints:
///
/// 0 <= n <= 30
use std::collections::HashMap;

/// time O(2^n)
/// space O(n)
fn fib(n: i32) -> i32 {
    if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

/// time O(n)
/// space O(n)
fn fib_2(n: i32) -> i32 {
    fn go(n: i32, cache: &mut HashMap<i32, i32>) -> i32 {
        if n == 0 {
            0
        } else if n == 1 {
            1
        } else if let Some(value) = cache.get(&n) {
            *value
        } else {
            let value = go(n - 1, cache) + go(n - 2, cache);
            cache.insert(n, value);
            value
        }
    }

    go(n, &mut HashMap::new())
}

/// time O(n)
/// space O(1)
fn fib_3(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;

    let mut result = 0;

    for _ in 2..=n {
        let temp = b;
        result = a + b;
        a = temp;
        b = result;
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib() {
        let cases = vec![(0, 0), (1, 1), (2, 1), (3, 2), (4, 3), (5, 5), (6, 8)];

        for (input, expected) in cases {
            dbg!(input);
            assert_eq!(expected, fib(input));
            assert_eq!(expected, fib_2(input));
            assert_eq!(expected, fib_3(input));
        }
    }
}
