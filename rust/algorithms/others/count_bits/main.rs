// 338. Counting Bits
//
// Given an integer n, return an array ans of length n + 1 such that
// for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.
//
// Example 1:
//
// Input: n = 2
// Output: [0,1,1]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
//
// Example 2:
//
// Input: n = 5
// Output: [0,1,1,2,1,2]
// Explanation:
// 0 --> 0
// 1 --> 1
// 2 --> 10
// 3 --> 11
// 4 --> 100
// 5 --> 101
//
// Constraints:
//
//     0 <= n <= 105
//
// Follow up:
//
//     It is very easy to come up with a solution with a runtime of O(n log n). Can you do it in linear time O(n) and possibly in a single pass?
//     Can you do it without using any built-in function (i.e., like __builtin_popcount in C++)?

// time O(n)
// space O(n)
pub fn count_bits(n: i32) -> Vec<i32> {
    let mut dp = vec![0; (n + 1) as usize];

    // Input: n = 2
    // Output: [0,1,1]
    // Explanation:
    // 0 --> 0
    // 1 --> 1
    // 2 --> 10
    //
    // dp[0] =
    //  (0 >> 1) + (0 & 1)
    //  (00 >> 1) + (00 & 01)
    //  00 + (00 & 01)
    //  00 + 00
    //  00
    //  0
    // dp[1] =
    //  (i >> 1) + (1 & 1)
    //  (01 >> 1) + (01 & 01)
    //  00 + (01 & 01)
    //  00 + 01
    //  01
    //  1
    // dp[2] =
    //  (2 >> 1) + (2 & 1)
    //  (10 >> 1) + (10 & 01)
    //  01 + (10 & 01)
    //  01 + 00
    //  01
    //  1
    for i in 0..n + 1 {
        dp[i as usize] = dp[(i >> 1) as usize] + (i & 1);
    }

    dp
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_bits() {
        let tests = vec![(2, vec![0, 1, 1]), (5, vec![0, 1, 1, 2, 1, 2])];

        for (input, expected) in tests {
            assert_eq!(expected, count_bits(input));
        }
    }
}
