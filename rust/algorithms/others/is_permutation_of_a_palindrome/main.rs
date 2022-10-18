//! Given a string, write a function to check if it is a permutation of
//! a palindrome. A palindrome is a word or phrase that is the same forwards and backwards. A
//! permutation is a rearrangement of letters. The palindrome does not need to be limited to just
//! dictionary words.

/// time O(n)
/// space O(1)
fn is_palindrome_permutation(s: &str) -> bool {
    const ASCII_LETTERS_IN_ALPHABET: usize = 26;

    let mut occurrences = [0; ASCII_LETTERS_IN_ALPHABET];

    for character in s.as_bytes() {
        if !character.is_ascii_alphabetic() {
            continue;
        }

        let index = (character.to_ascii_lowercase() - 'a' as u8) as usize;
        occurrences[index] += 1;
    }

    let mut odd_characters_count = 0;

    for count in occurrences.into_iter() {
        // If it is odd.
        if count & 1 != 0 {
            if odd_characters_count > 0 {
                return false;
            }

            odd_characters_count += 1;
        }
    }

    true
}

/// time O(n)
/// space O(1)
fn is_palindrome_permutation_bit_vector(s: &str) -> bool {
    let mut occurrences = 0;

    for character in s.as_bytes() {
        if !character.is_ascii_alphabetic() {
            continue;
        }

        let index = (character.to_ascii_lowercase() - 'a' as u8) as usize;
        // Toggle the bit.
        occurrences ^= 1 << index;
    }

    occurrences & (occurrences - 1) == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert!(is_palindrome_permutation("Tact Coa"));
        assert!(!is_palindrome_permutation("Tactt Coa"));

        assert!(is_palindrome_permutation_bit_vector("Tact Coa"));
        assert!(!is_palindrome_permutation_bit_vector("Tactt Coa"));
    }
}
