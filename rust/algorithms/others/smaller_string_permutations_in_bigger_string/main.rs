//! Given a smaller strings and a bigger string b, design an algorithm to find all
//! permutions of the shorter string within the longer one. Print the location of each permutation.

use std::collections::HashSet;

fn to_index(character: u8) -> usize {
    // Subtract the character 'a' byte value from the character so 'a' maps to 0, 'b' to 1 and so on.
    (character - 'a' as u8) as usize
}

/// time O(n) where n = b.len() and s.len() < b.len()
/// space O(n) where n = b.len()
fn permutations<'a, 'b>(s: &'a str, b: &'b str) -> Vec<&'b str> {
    const LETTERS_IN_ALPHABET: usize = 26;
    let mut s_count = [0; LETTERS_IN_ALPHABET];

    // Count how many times each letter shows up.
    // O(s.len())
    for character in s.as_bytes() {
        let index = to_index(*character);
        s_count[index] += 1;
    }

    let b_as_bytes = b.as_bytes();

    let mut result = HashSet::new();

    let mut permutation_count = [0; LETTERS_IN_ALPHABET];

    // O(s.len())
    for i in 0..s.len() {
        let index = to_index(b_as_bytes[i]);
        permutation_count[index] += 1;
    }

    let mut i = 0;
    let mut j = s.len();

    // O(b.len())
    loop {
        // O(26)
        if s_count == permutation_count {
            // O(1)
            result.insert(&b[i..j]);
        }

        if j >= b.len() {
            break;
        }

        let index = to_index(b_as_bytes[i]);
        permutation_count[index] -= 1;

        let index = to_index(b_as_bytes[j]);
        permutation_count[index] += 1;

        i += 1;
        j += 1;
    }

    // O(n)
    result.into_iter().collect()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let mut expected = vec!["cbab", "cbba", "abbc", "bcba", "babc"];
        expected.sort_unstable();

        let mut actual = permutations("abbc", "cbabadcbbabbcbabaabccbabc");
        actual.sort_unstable();

        assert_eq!(expected, actual);
    }
}
