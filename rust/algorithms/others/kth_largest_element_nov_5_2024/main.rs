use std::collections::BinaryHeap;

/// Given an integer array nums and an integer k, return the kth largest element in the array.
///
/// Note that it is the kth largest element in the sorted order, not the kth distinct element.
///
/// Can you solve it without sorting?
///
/// Example 1:
///
/// Input: nums = [3,2,1,5,6,4], k = 2
/// Output: 5
/// Example 2:
///
/// Input: nums = [3,2,3,1,2,4,5,5,6], k = 4
/// Output: 4
///
/// Constraints:
///
/// 1 <= k <= nums.length <= 105
/// -104 <= nums[i] <= 104

fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap: BinaryHeap<_> = nums.into_iter().collect();

    for _ in 0..k as usize - 1 {
        let _ = heap.pop();
    }

    heap.pop().unwrap()
}

fn find_kth_largest_quick_select(nums: Vec<i32>, k: i32) -> i32 {
    do_find_kth_largest_quick_select(&nums, 0, nums.len() - 1, k as usize - 1)
}

fn do_find_kth_largest_quick_select(nums: &[i32], start: usize, end: usize, k: usize) -> i32 {
    if start == end {
        return nums[start];
    }

    let index = (start + end) / 2;
    if index == k {
        return nums[index];
    }

    if k < index {
        do_find_kth_largest_quick_select(nums, start, index - 1, k)
    } else {
        do_find_kth_largest_quick_select(nums, index + 1, end, k)
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;

    fn check(nums: Vec<i32>, k: usize, f: fn(Vec<i32>, i32) -> i32) -> bool {
        if nums.is_empty() {
            return true;
        }

        let k = 1 + k % nums.len();
        let mut nums = nums;
        nums.sort_unstable();
        nums.reverse();
        let expected = nums[k - 1];
        expected == f(nums, k as i32)
    }

    quickcheck! {
      #[test]
      fn test_find_kth_largest(nums: Vec<i32>, k: usize) -> bool {
        check(nums, k, find_kth_largest)
      }

      fn test_find_kth_largest_quick_select(nums: Vec<i32>, k: usize) -> bool {
        check(nums, k, find_kth_largest_quick_select)
      }
    }
}
