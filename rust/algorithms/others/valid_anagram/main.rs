//! 242. Valid Anagram
//!
//! Given two strings s and t, return true if t is an anagram of s, and false otherwise.
//!
//! An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
//! typically using all the original letters exactly once.
//!
//! Example 1:
//!
//! Input: s = "anagram", t = "nagaram"
//! Output: true
//!
//! Example 2:
//!
//! Input: s = "rat", t = "car"
//! Output: false
//!
//! Constraints:
//!
//!     1 <= s.length, t.length <= 5 * 104
//!     s and t consist of lowercase English letters.
//!
//! Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

use std::collections::HashMap;

/// time O(n) where n = s.len() or t.len() since they are equal.
/// space O(n) where n = s.len() or t.len() since they are equal.
pub fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut ocurrences = HashMap::new();

    for (s_char, t_char) in s.chars().into_iter().zip(t.chars().into_iter()) {
        let count = ocurrences.entry(s_char).or_insert(0);

        *count += 1;

        let count = ocurrences.entry(t_char).or_insert(0);

        *count -= 1;
    }

    ocurrences.into_iter().all(|(_char, count)| count == 0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let tests = vec![
            ("", "a", false),
            ("anagram", "nagaram", true),
            ("rat", "car", false),
        ];

        for (s, t, expected) in tests {
            assert_eq!(expected, is_anagram(s, t));
        }
    }
}
