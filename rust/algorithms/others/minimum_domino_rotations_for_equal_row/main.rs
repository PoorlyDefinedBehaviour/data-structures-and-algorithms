//! 1007. Minimum Domino Rotations For Equal Row
//!
//! In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the ith domino.
//! (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)
//!
//! We may rotate the ith domino, so that tops[i] and bottoms[i] swap values.
//!
//! Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.
//!
//! If it cannot be done, return -1.
//!
//! Example 1:
//!
//! Input: tops = [2,1,2,4,2,2], bottoms = [5,2,6,2,3,2]
//! Output: 2
//! Explanation:
//! The first figure represents the dominoes as given by tops and bottoms: before we do any rotations.
//! If we rotate the second and fourth dominoes, we can make every value in the top row equal to 2, as indicated by the second figure.
//!
//! Example 2:
//!
//! Input: tops = [3,5,1,2,3], bottoms = [3,6,3,3,4]
//! Output: -1
//! Explanation:
//! In this case, it is not possible to rotate the dominoes to make one row of values equal.
//!
//! Constraints:
//!
//!     2 <= tops.length <= 2 * 104
//!     bottoms.length == tops.length
//!     1 <= tops[i], bottoms[i] <= 6

/// time O(n)
/// space O(1)
pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
    if tops.is_empty() || bottoms.is_empty() {
        return -1;
    }

    let first_top = tops[0];
    let first_bottom = bottoms[0];

    let mut swaps_needed_to_top = 0;
    let mut swaps_to_top = 0;

    let mut swaps_needed_to_bottom = 0;
    let mut swaps_to_bottom = 0;

    for (top, bottom) in tops.into_iter().zip(bottoms.into_iter()) {
        if top != first_top {
            swaps_needed_to_top += 1;
        }

        if top != first_top && bottom == first_top {
            swaps_to_top += 1;
        }

        if bottom != first_bottom {
            swaps_needed_to_bottom += 1;
        }

        if bottom != first_bottom && top == first_bottom {
            swaps_to_bottom += 1;
        }
    }

    if swaps_to_bottom == swaps_needed_to_bottom && swaps_to_top == swaps_needed_to_top {
        return std::cmp::min(swaps_to_bottom, swaps_to_top);
    }

    if swaps_to_bottom == swaps_needed_to_bottom {
        return swaps_to_bottom;
    }

    if swaps_to_top == swaps_needed_to_top {
        return swaps_to_top;
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_domino_rotations() {
        let tests = vec![
            (
                "empty tops and bottoms, should return -1",
                vec![],
                vec![],
                -1,
            ),
            (
                "should move 2 from the bottom to the top",
                vec![2, 1, 2, 4, 2, 2],
                vec![5, 2, 6, 2, 3, 2],
                2,
            ),
            (
                "should move 2 from the top to the bottom",
                vec![5, 2, 6, 2, 3, 2],
                vec![2, 1, 2, 4, 2, 2],
                2,
            ),
            ("no solution", vec![3, 5, 1, 2, 3], vec![3, 6, 3, 3, 4], -1),
        ];

        for (message, tops, bottoms, expected) in tests {
            assert_eq!(expected, min_domino_rotations(tops, bottoms), "{}", message);
        }
    }
}
