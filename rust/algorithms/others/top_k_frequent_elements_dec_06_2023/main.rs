use std::collections::{BinaryHeap, HashMap};

#[derive(PartialEq, Eq)]
struct Entry {
    num: i32,
    count: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.count.cmp(&other.count)
    }
}

/// time O(n + k log n)
/// space O(n)
fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut count = HashMap::new();

    // time O(n)
    // space O(n)
    for num in nums {
        let entry = count.entry(num).or_insert(0);
        *entry += 1;
    }

    // time O(n)
    // space O(n)
    let mut heap = BinaryHeap::from_iter(count.into_iter().map(|(num, n)| Entry { num, count: n }));

    let mut out = Vec::with_capacity(k as usize);

    // time O(k log n)
    // space O(k)
    for _ in 0..k {
        out.push(heap.pop().expect("k is guaranteed to be <= nums.len()").num);
    }

    out
}

/// time O(n)
/// space O(n)
fn top_k_frequent_2(nums: Vec<i32>, k: i32) -> Vec<i32> {
    // space O(n)
    let mut occurrences = vec![vec![]; nums.len()];

    let mut count = HashMap::new();

    // time O(n)
    // space O(n)
    for num in nums {
        let entry = count.entry(num).or_insert(0);
        *entry += 1;
    }

    // time O(n)
    for (num, count) in count {
        occurrences[count as usize - 1].push(num);
    }

    // space O(k)
    let mut out = Vec::with_capacity(k as usize);

    // time O(n)
    for nums in occurrences
        .into_iter()
        // Start from the numbers with the highest frequency.
        .rev()
    {
        // Take the k most frequent numbers.
        // time O(k)
        for num in nums {
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
    fn test_top_k_frequent() {
        let cases = [
            (vec![1, 1, 1, 2, 2, 3], 2, vec![1, 2]),
            (vec![1], 1, vec![1]),
        ];

        for (nums, k, expected) in cases {
            assert_eq!(expected, top_k_frequent(nums.clone(), k));
            assert_eq!(expected, top_k_frequent_2(nums, k));
        }
    }
}
