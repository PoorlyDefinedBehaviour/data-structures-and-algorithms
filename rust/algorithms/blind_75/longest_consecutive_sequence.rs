use std::collections::HashSet;

/// Solution: find a number that is the first number in a sequence(the number without a previous number)
/// and calculate the sequence length starting from it.
///
/// time O(n)
/// space O(n)
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let nums = nums.into_iter().collect::<HashSet<_>>();

    let mut max_sequence_len = 0;

    for num in nums.iter() {
        // If num is not the first number in the sequence.
        // Example: 2 in the 1, 2, 3 sequence.
        if nums.contains(&(num - 1)) {
            continue;
        }

        // We are at the first number in a sequence, find out the sequence length.
        let mut current_num = num + 1;

        // Sequence len is at least 1  to take the first number in the sequence into account.
        let mut sequence_len = 1;

        while nums.contains(&current_num) {
            current_num += 1;
            sequence_len += 1;
        }

        max_sequence_len = std::cmp::max(max_sequence_len, sequence_len);
    }

    max_sequence_len
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::longest_consecutive;

    #[test]
    fn test() {
        assert_eq!(4, longest_consecutive(vec![100, 4, 200, 1, 3, 2]));
        assert_eq!(9, longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]));
    }
}
