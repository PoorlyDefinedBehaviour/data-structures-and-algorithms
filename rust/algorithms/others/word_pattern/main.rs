//! 290. Word Pattern
//!
//! Given a pattern and a string s, find if s follows the same pattern.
//!
//! Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s.
//!
//! Example 1:
//!
//! Input: pattern = "abba", s = "dog cat cat dog"
//! Output: true
//!
//! Example 2:
//!
//! Input: pattern = "abba", s = "dog cat cat fish"
//! Output: false
//!
//! Example 3:
//!
//! Input: pattern = "aaaa", s = "dog cat cat dog"
//! Output: false
//!
//! Constraints:
//!
//!     1 <= pattern.length <= 300
//!     pattern contains only lower-case English letters.
//!     1 <= s.length <= 3000
//!     s contains only lowercase English letters and spaces ' '.
//!     s does not contain any leading or trailing spaces.
//!     All the words in s are separated by a single space.

use std::collections::{HashMap, HashSet};

/// time O(n + m) where n is pattern.len() and m is the number of words in s.
/// space O(n)
pub fn word_pattern(pattern: String, s: String) -> bool {
    // Set of words we have already seen.
    let mut words_seen = HashSet::new();
    // Maps letters to words.
    let mut patterns_symbols_seen = HashMap::new();

    let words: Vec<&str> = s.split_ascii_whitespace().collect();

    // If we don't have enough or have too many words to match the pattern.
    if words.len() != pattern.len() {
        return false;
    }

    for (pattern_symbol, word) in pattern.chars().zip(words.into_iter()) {
        match patterns_symbols_seen.get(&pattern_symbol) {
            None => {
                // If we have already seen this word, it means that the word
                // is mapepd to another pattern symbol.
                if words_seen.contains(word) {
                    return false;
                }
                // If it is the first time are seeing this pattern symbol,
                // associate it to the word we are looking at.
                patterns_symbols_seen.insert(pattern_symbol, word);
            }
            Some(&expected_word) => {
                // If we have already seen this pattern symbol and it matched
                // another word, it means `s` does not match the pattern.
                if expected_word != word {
                    return false;
                }
            }
        }

        words_seen.insert(word);
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_pattern() {
        let tests = vec![
            ("abba", "dog cat cat dog", true),
            ("abba", "dog cat cat fish", false),
            ("aaaa", "dog cat cat dog", false),
        ];

        for (pattern, s, expected) in tests {
            assert_eq!(
                expected,
                word_pattern(String::from(pattern), String::from(s))
            );
        }
    }
}
