// Given an integer array nums and an integer k, return the kth largest element in the array.
//
// Note that it is the kth largest element in the sorted order, not the kth distinct element.
//
// Can you solve it without sorting?
//
//
// Example 1:
//
// Input: nums = [3,2,1,5,6,4], k = 2
// Output: 5
//
// Example 2:
//
// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
// Output: 4
//
// Constraints:
//
// 1 <= k <= nums.length <= 105
// -104 <= nums[i] <= 104

use std::{cmp::Reverse, collections::BinaryHeap};

// time O(n log n)
// space O(n)
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::new();
    for num in nums {
        if heap.len() < k as usize {
            heap.push(Reverse(num));
        } else {
            let Reverse(value) = heap.pop().unwrap();
            heap.push(Reverse(std::cmp::max(value, num)));
        }
    }

    // 0-indexed but k is 1-indexed so add a - 1
    let Reverse(v) = heap.pop().unwrap();
    v
}

// time O(n + k log(n))
// space O(n)
pub fn find_kth_largest_2(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::from(nums);
    for _ in 0..k - 1 {
        let _ = heap.pop();
    }
    heap.pop().unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        let cases = [
            (vec![3, 2, 1, 5, 6, 4], 2, 5),
            (vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4, 4),
        ];

        for (nums, k, expected) in cases {
            assert_eq!(expected, find_kth_largest(nums.clone(), k));
            assert_eq!(expected, find_kth_largest_2(nums, k));
        }
    }
}
