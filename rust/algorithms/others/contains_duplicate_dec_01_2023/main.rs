use std::collections::HashSet;

/// time O(n)
/// space O(n)
fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut seen = HashSet::new();

    for num in nums {
        // insert returns false when the element is already in the set.
        if !seen.insert(num) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use crate::contains_duplicate;

    #[test]
    fn test_contains_duplicate() {
        let cases = [
            (vec![1, 2, 3, 1], true),
            (vec![1, 2, 3, 4], false),
            (vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2], true),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, contains_duplicate(input));
        }
    }
}
