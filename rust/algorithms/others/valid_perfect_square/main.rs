//! 367. Valid Perfect Square
//!
//! Given a positive integer num, write a function which returns True if num is a perfect square else False.
//!
//! Follow up: Do not use any built-in library function such as sqrt.
//!
//! Example 1:
//!
//! Input: num = 16
//! Output: true
//!
//! Example 2:
//!
//! Input: num = 14
//! Output: false
//!
//! Constraints:
//!
//!  1 <= num <= 2^31 - 1

/// Returns true if `num` is a perfect square.
///
/// A perfect square is a number that is equal to some number squared.
///
/// 4 is a perfect square because it is equal to 2^2.
/// 9 is a perfect square because it is equal to 3^2.
///
/// We just check every number up to `sqrt(num)` until we find
/// a square thats equal to `num` or a square that's greater than it
/// in which case we know that `num` is not a perfect square.
///
/// time O(sqrt(num))
/// space O(1)
pub fn is_perfect_square_1(num: i32) -> bool {
    let num = num as usize;

    for i in 1..=num {
        let square = i * i;

        if square == num {
            return true;
        }

        if square > num {
            return false;
        }
    }

    return false;
}

/// Returns true if `num` is a perfect square.
///
/// A perfect square is a number that is equal to some number squared.
///
/// 4 is a perfect square because it is equal to 2^2.
/// 9 is a perfect square because it is equal to 3^2.
///
/// Since we know num is always positive,
/// we can binary search the perfect square.
///
/// That's better than the sqrt(num) solution because
/// log num is grows slower than sqrt(num).
///
/// time O(log n)
/// space O(1)
pub fn is_perfect_square_2(num: i32) -> bool {
    let num = num as usize;

    let mut left = 1;
    let mut right = num;

    while left <= right {
        let mid = (left + right) / 2;

        let square = mid * mid;

        if square == num {
            return true;
        }

        if square < num {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_perfect_square() {
        let tests = vec![(1, true), (16, true), (14, false), (2147483647, false)];

        for (input, expected) in tests {
            assert_eq!(expected, is_perfect_square_1(input));
            assert_eq!(expected, is_perfect_square_2(input));
        }
    }
}
