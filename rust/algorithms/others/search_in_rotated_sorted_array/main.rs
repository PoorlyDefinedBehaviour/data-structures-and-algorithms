// There is an integer array nums sorted in ascending order (with distinct values).
//
// Prior to being passed to your function, nums is possibly left rotated at an unknown index k (1 <= k < nums.length)
// such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be left rotated by 3 indices and become [4,5,6,7,0,1,2].
//
// Given the array nums after the possible rotation and an integer target,
// return the index of target if it is in nums, or -1 if it is not in nums.
//
// You must write an algorithm with O(log n) runtime complexity.
//
// Example 1:
//
// Input: nums = [4,5,6,7,0,1,2], target = 0
// Output: 4
// Example 2:
//
// Input: nums = [4,5,6,7,0,1,2], target = 3
// Output: -1
// Example 3:
//
// Input: nums = [1], target = 0
// Output: -1
//
// Constraints:
//
// 1 <= nums.length <= 5000
// -104 <= nums[i] <= 104
// All values of nums are unique.
// nums is an ascending array that is possibly rotated.
// -104 <= target <= 104

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    go(&nums, 0, nums.len() as isize - 1, target)
}

fn go(nums: &[i32], start: isize, end: isize, target: i32) -> i32 {
    if start as usize > nums.len() - 1 || end < start {
        return -1;
    }
    let mid = (start + end) / 2;
    if nums[mid as usize] == target {
        return mid as i32;
    }
    let left = go(nums, start, mid - 1, target);
    if left != -1 {
        return left;
    }
    go(nums, mid + 1, end, target)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;
    use proptest::prelude::*;

    #[test]
    fn debug() {
        let xs = vec![
            -17, 20, -19, -4, -20, 17, 9, 18, 13, 24, -27, -28, -23, 25, -26, 5, -21, 6, 7, 11, -6,
            -13, 12, 10, -10, -11, -8, -15, 14, 4, -3, -5, 2, -7, 3, -18, 0, 15, 16, 21, 22, -22,
            -16, -12, -24, 23, -1, 8, -29, -25, -30, -2, 1, 19, -14, -9,
        ];
        // [1,2,3,4,5,6]
        // 0 [1,2,3]     3 [4,5,6]
        // 0 [1]    2 [3]   3 [4]   5 [6]
        let target = xs[(-14379_i32).abs() as usize % xs.len()];
        dbg!(&target);
        for (j, v) in xs.iter().enumerate() {
            if *v == target {
                dbg!(j);
                break;
            }
        }

        dbg!(search(xs, target));
    }

    proptest! {
        #[test]
        fn model(xs: HashSet<i32>, k: usize, use_target: bool, target: i32) {
            prop_assume!(!xs.is_empty());

            let k = k % xs.len();
            let mut xs: Vec<i32> = xs.into_iter().collect();
            xs.sort();
            xs.rotate_left(k);

            let target = if use_target {
                target
            } else {
               xs[target.abs()as usize % xs.len()]
            };

            let mut i = -1;
            for (j, v) in xs.iter().enumerate() {
                if *v == target {
                    i = j as i32;
                    break;
                }
            }

            assert_eq!(i, search(xs, target));
        }
    }

    #[test]
    fn examples() {
        let cases = [
            // (vec![4, 5, 6, 7, 0, 1, 2], 0, 4),
            // (vec![4, 5, 6, 7, 0, 1, 2], 3, -1),
            // (vec![1], 0, -1),
            // (vec![0], 0, 0),
            (
                {
                    let mut xs = vec![
                        -10,
                        -12,
                        -9,
                        5,
                        -14,
                        -942117095,
                        -1472220650,
                        0,
                        2086166308,
                        7,
                        -2,
                        8,
                        35906401,
                        -8,
                        -4,
                        -108519800,
                        -5,
                        -6,
                        6,
                        1774723144,
                        -1618883436,
                        -716662869,
                        13,
                        -7,
                        728688937,
                        -3,
                        1,
                        3,
                        4,
                        11,
                        -30,
                        203547261,
                        59324065,
                        -1,
                        -15,
                        1152269855,
                        -11,
                        1978415384,
                        -87459519,
                        -463830807,
                        -13,
                        -384816374,
                        1453610896,
                        -1254071403,
                        -1698416569,
                        -184770824,
                        10,
                        395845247,
                        12,
                        1660777790,
                        -584788371,
                        2,
                        9,
                        -1757770309,
                        867307673,
                        -1883169390,
                    ];
                    xs.sort();
                    let mid = 4319276902770599842 % xs.len();
                    xs.rotate_left(mid);
                    xs
                },
                -1843035760,
                -1,
            ),
            (
                vec![
                    -17, 20, -19, -4, -20, 17, 9, 18, 13, 24, -27, -28, -23, 25, -26, 5, -21, 6, 7,
                    11, -6, -13, 12, 10, -10, -11, -8, -15, 14, 4, -3, -5, 2, -7, 3, -18, 0, 15,
                    16, 21, 22, -22, -16, -12, -24, 23, -1, 8, -29, -25, -30, -2, 1, 19, -14, -9,
                ],
                -12,
                43,
            ),
        ];

        for (nums, target, expected) in cases {
            assert_eq!(expected, search(nums, target));
        }
    }
}
