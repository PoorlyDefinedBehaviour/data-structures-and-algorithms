//! Given an array of distinct integer values, count the number of pairs of integers that
//! have difference k. For example, given the array { 1, 7, 5, 9, 2, 12, 3} and the difference
//! k = 2, there are four pairs with difference 2: (1, 3), (3, 5), (5, 7), (7, 9).

use std::collections::HashSet;

/// time O(n)
/// space O(n)
fn count_pairs(nums: Vec<i32>, k: i32) -> Vec<(i32, i32)> {
    // O(n)
    let nums: HashSet<i32> = nums.into_iter().collect();

    let mut result = vec![];

    // O(n)
    for num in nums.iter() {
        let difference = (k - *num).abs();
        // O(1)
        if difference != *num && nums.contains(&difference) {
            // O(1)~
            result.push((difference, *num));
        }
    }

    result
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut expected = vec![(1, 3), (3, 5), (5, 7), (7, 9)];
        expected.sort_unstable();

        let mut actual = count_pairs(vec![1, 7, 5, 9, 2, 12, 3], 2);

        actual.sort_unstable();

        assert_eq!(expected, actual);
    }
}
