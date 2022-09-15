use std::collections::HashMap;

/// time O(n)
/// space (n)
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut cache = HashMap::new();

    for (i, num) in nums.into_iter().enumerate() {
        let x = target - num;

        if let Some(j) = cache.get(&x) {
            return vec![*j, i as i32];
        }

        cache.insert(num, i as i32);
    }

    unreachable!()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tests = vec![
            (vec![2, 7, 11, 15], 9, vec![0, 1]),
            (vec![3, 2, 4], 6, vec![1, 2]),
            (vec![3, 3], 6, vec![0, 1]),
            (vec![-3, 4, 3, 90], 0, vec![0, 2]),
        ];

        for (input, target, expected) in tests {
            assert_eq!(expected, two_sum(input, target));
        }
    }
}
