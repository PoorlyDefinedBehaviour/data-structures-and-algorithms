/// Given an integer array nums, return the length of the longest strictly increasing
/// subsequence
///
/// Example 1:
///
/// Input: nums = [10,9,2,5,3,7,101,18]
/// Output: 4
/// Explanation: The longest increasing subsequence is [2,3,7,101], therefore the length is 4.
/// Example 2:
///
/// Input: nums = [0,1,0,3,2,3]
/// Output: 4
/// Example 3:
///
/// Input: nums = [7,7,7,7,7,7,7]
/// Output: 1
///
/// Constraints:
///
/// 1 <= nums.length <= 2500
/// -104 <= nums[i] <= 104

/// time O(n²)
/// space O(n)
fn length_of_lis(nums: Vec<i32>) -> i32 {
    let mut max_sub_sequence_at_index = vec![1; nums.len()];

    for (i, num) in nums.iter().enumerate().rev() {
        let max_length = nums
            .iter()
            .enumerate()
            .skip(i)
            .filter_map(|(j, other_num)| {
                if other_num > num {
                    Some(max_sub_sequence_at_index[j])
                } else {
                    None
                }
            })
            .max()
            .unwrap_or(0);

        max_sub_sequence_at_index[i] = 1 + max_length;
    }

    max_sub_sequence_at_index.into_iter().max().unwrap_or(0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_length_of_lis() {
        let cases = [
            (vec![1, 2, 4, 3], 3),
            (vec![10, 9, 2, 5, 3, 7, 101, 18], 4),
            (vec![7, 7, 7, 7, 7, 7, 7], 1),
            (vec![0, 1, 0, 3, 2, 3], 4),
        ];

        for (input, expected) in cases {
            assert_eq!(expected, length_of_lis(input));
        }
    }
}
