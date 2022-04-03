//! 49. Group Anagrams
//!
//! Given an array of strings strs, group the anagrams together. You can return the answer in any order.
//!
//! An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
//! typically using all the original letters exactly once.
//!
//! Example 1:
//!
//! Input: strs = ["eat","tea","tan","ate","nat","bat"]
//! Output: [["bat"],["nat","tan"],["ate","eat","tea"]]
//!
//! Example 2:
//!
//! Input: strs = [""]
//! Output: [[""]]
//!
//! Example 3:
//!
//! Input: strs = ["a"]
//! Output: [["a"]]
//!
//! Constraints:
//!
//!     1 <= strs.length <= 104
//!     0 <= strs[i].length <= 100
//!     strs[i] consists of lowercase English letters.

use std::collections::HashMap;

/// time O(n * m) where n = strs.len() and m = max(strs[i].len())
/// space O(n * m) where n = strs.len() and m = max(strs[i].len())
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups = HashMap::new();

    for s in strs.into_iter() {
        let s_ascii = s.as_bytes();

        let mut occurrences = [0_u8; 26];

        for ascii_character in s_ascii {
            occurrences[(ascii_character - 97) as usize] += 1;
        }

        let anagrams = groups.entry(occurrences).or_insert_with(Vec::new);

        anagrams.push(s);
    }

    groups.into_values().collect()
}
