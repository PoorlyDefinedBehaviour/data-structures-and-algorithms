//! 187. Repeated DNA Sequences
//!
//! The DNA sequence is composed of a series of nucleotides abbreviated as 'A', 'C', 'G', and 'T'.
//!
//!     For example, "ACGAATTCCG" is a DNA sequence.
//!
//! When studying DNA, it is useful to identify repeated sequences within the DNA.
//!
//! Given a string s that represents a DNA sequence, return all the 10-letter-long sequences (substrings) that occur more than once in a DNA molecule.
//! You may return the answer in any order.
//!
//! Example 1:
//!
//! Input: s = "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT"
//! Output: ["AAAAACCCCC","CCCCCAAAAA"]
//!
//! Example 2:
//!
//! Input: s = "AAAAAAAAAAAAA"
//! Output: ["AAAAAAAAAA"]
//!
//! Constraints:
//!
//!     1 <= s.length <= 105
//!     s[i] is either 'A', 'C', 'G', or 'T'.

use std::collections::HashMap;

/// time O(n) where n is s.len()
/// space O(n) where n is s.len()
pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
    let mut seen = HashMap::new();

    for i in 0..s.len() {
        if i + 10 >= s.len() {
            break;
        }

        let sequence = &s[i..i + 10];

        let count = seen.entry(sequence).or_insert(0);
        *count += 1;
    }

    // NOTE: we are iterating over every key in the HashMap,
    // this could be avoided if we just added the sequence to the output vector
    // after seeing it for the second time.
    seen.into_iter()
        .filter_map(|(sequence, count)| {
            if count > 1 {
                Some(sequence.to_string())
            } else {
                None
            }
        })
        .collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_find_repeated_dna_sequences() {
        let tests = vec![
            ("", vec![]),
            (
                "AAAAACCCCCAAAAACCCCCCAAAAAGGGTTT",
                vec!["AAAAACCCCC", "CCCCCAAAAA"],
            ),
            ("AAAAAAAAAAAAA", vec!["AAAAAAAAAA"]),
        ];

        for (input, expected) in tests {
            let expected: HashSet<String> =
                HashSet::from_iter(expected.into_iter().map(String::from));

            let actual: HashSet<String> =
                HashSet::from_iter(find_repeated_dna_sequences(String::from(input)).into_iter());

            assert_eq!(expected, actual, "input: {:?}", input);
        }
    }
}
