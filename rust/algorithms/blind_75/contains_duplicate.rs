use std::collections::{hash_map::Entry, HashMap};

/// time O(n)
/// space O(n)
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashMap::new();

    for num in nums.into_iter() {
        match seen.entry(num) {
            Entry::Occupied(_) => return true,
            Entry::Vacant(entry) => {
                entry.insert(());
            }
        }
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_duplicate() {
        let tests = vec![
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, contains_duplicate(input));
        }
    }
}
