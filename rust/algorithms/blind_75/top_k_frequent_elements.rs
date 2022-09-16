use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    num: i32,
    count: i32,
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// time
///   O(n) + O(n) + (k log n)
///   O(2n) + (k log n)
///   O(n) + (k log n)
///
/// space O(n)
pub fn top_k_frequent_heap(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut counts = HashMap::new();

    /// O(n)
    for num in nums.into_iter() {
        let entry = counts.entry(num).or_insert(0);
        *entry += 1;
    }

    let mut heap = BinaryHeap::with_capacity(counts.len());

    // O(n) (push is O(1)~)
    for (num, count) in counts.into_iter() {
        heap.push(Entry { num, count });
    }

    let mut out = Vec::with_capacity(k as usize);

    // O(k log n)
    for _ in 0..k {
        out.push(heap.pop().unwrap().num);
    }

    out
}

/// time O(n)
/// space O(n)
pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut occurrences = vec![vec![]; nums.len()];

    let mut counts = HashMap::new();

    for num in nums.into_iter() {
        let entry = counts.entry(num).or_insert(0);
        *entry += 1;
    }

    for (num, count) in counts.into_iter() {
        occurrences[count as usize - 1].push(num);
    }

    let mut out = Vec::with_capacity(k as usize);

    for nums in occurrences
        .into_iter()
        .rev()
        .filter(|nums| !nums.is_empty())
    {
        for num in nums.into_iter() {
            if out.len() >= k as usize {
                break;
            }

            out.push(num);
        }
    }

    out
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tests = vec![(vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2])];

        for (input, k, expected) in tests {
            assert_eq!(expected, top_k_frequent(input, k));
        }
    }
}
