use std::collections::HashMap;

/// time O(n)
/// space O(n)
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache = HashMap::new();

    for (i, num) in nums.into_iter().enumerate() {
        let n = (target - num).abs();

        if let Some(j) = cache.get(&n) {
            return vec![*j, i as i32];
        }

        cache.insert(num, i as i32);
    }

    unreachable!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::two_sum;

    #[test]
    fn test_two_sum() {
        let cases = [
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
        ];

        for (nums, target, expected) in cases {
            assert_eq!(expected, two_sum(nums, target));
        }
    }
}
