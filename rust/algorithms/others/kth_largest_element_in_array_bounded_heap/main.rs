use std::{cmp::Reverse, collections::BinaryHeap};

/// time O(n log k)
/// space O(k)
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    let mut heap = BinaryHeap::new();

    // O(n)
    for num in nums {
        // Wrap elements in Reverse so we can build a min heap.
        // O(log k)
        heap.push(Reverse(num));

        // Heap will never have more than k + 1 elements.
        if heap.len() > k as usize {
            // Remove the smallest element.
            // O(log k)
            let _ = heap.pop();
        }
    }

    // The smallest element is the kth greatest element.
    // O(log k)
    heap.pop().unwrap().0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(5, find_kth_largest(vec![3, 2, 1, 5, 6, 4], 2));

        assert_eq!(4, find_kth_largest(vec![3, 2, 3, 1, 2, 4, 5, 5, 6], 4));
    }
}
