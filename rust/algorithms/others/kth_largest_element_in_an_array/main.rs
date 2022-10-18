use std::collections::BinaryHeap;

/// time O(n + k log n)
/// space O(n)
pub fn find_kth_largest(nums: Vec<i32>, k: i32) -> i32 {
    // BinaryHeap is a max heap by default.
    let mut heap: BinaryHeap<i32> = nums.into_iter().collect();

    for _ in 0..k - 1 {
        // Pop k-1 elements.
        let _ = heap.pop();
    }

    // Pop the kth element.
    heap.pop().unwrap()
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
