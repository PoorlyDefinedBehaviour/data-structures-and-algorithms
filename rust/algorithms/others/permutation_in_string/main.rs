// 567. Permutation in String
//
// Given two strings s1 and s2, return true if s2 contains a permutation of s1, or false otherwise.
//
// In other words, return true if one of s1's permutations is the substring of s2.
//
// Example 1:
//
// Input: s1 = "ab", s2 = "eidbaooo"
// Output: true
// Explanation: s2 contains one permutation of s1 ("ba").
//
// Example 2:
//
// Input: s1 = "ab", s2 = "eidboaoo"
// Output: false
//
// Constraints:
//
//     1 <= s1.length, s2.length <= 104
//     s1 and s2 consist of lowercase English letters.

// time O(n * m)
//  where n is s1.len()
//  and m is s2.len()
// space O(1) -- 26 lowercase english letters
pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let mut left = 0;
    let mut right = s1.len();

    let s1_count = {
        let mut count = vec![0; 26];

        for &character in s1.as_bytes() {
            count[character as usize - 97] += 1;
        }

        count
    };

    while right < s2.len() {
        // Using vectors instead of hashmaps because indexing is faster.
        let mut count = vec![0; 26];

        for i in left..right {
            count[s2.as_bytes()[i] as usize - 97] += 1;
        }

        if s1_count == count {
            return true;
        }

        left += 1;
        right += 1;
    }

    false
}

// time O(n)
// space O(1) -- 26 lowercase english letters
//
// Use a sliding window to count how many chars from s2 are in s1.
pub fn check_inclusion_2(s1: String, s2: String) -> bool {
    if s1.len() > s2.len() {
        return false;
    }

    let s1 = s1.as_bytes();
    let s2 = s2.as_bytes();

    let s1_count = {
        let mut count = vec![0; 26];

        for &character in s1 {
            count[character as usize - 97] += 1;
        }

        count
    };

    let target_matches = s1.len() as i32;
    let mut matches: i32 = 0;

    for i in 0..s1.len() {
        if s1_count[s2[i] as usize - 97] > 0 {
            matches += 1;
        }
    }

    if matches == target_matches {
        return true;
    }

    for i in s1.len()..s2.len() {
        // If s1 contains the character from s2 that we are currently looking at
        // we increase the number of matches, otherwise we decrease it.
        if s1_count[s2[i] as usize - 97] > 0 {
            matches += 1;
        } else {
            matches = 0;
        }

        if matches == target_matches {
            return true;
        }
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_inclusion() {
        let tests = vec![("ab", "eidbaooo", true), ("ab", "eidboaoo", false)];

        for (s1, s2, expected) in tests {
            assert_eq!(
                expected,
                check_inclusion(String::from(s1), String::from(s2))
            );

            assert_eq!(
                expected,
                check_inclusion_2(String::from(s1), String::from(s2))
            );
        }
    }
}
