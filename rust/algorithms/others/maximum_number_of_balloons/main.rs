//! 1189. Maximum Number of Balloons
//!
//! Given a string text, you want to use the characters of text to form as many instances of the word "balloon" as possible.
//!
//! You can use each character in text at most once. Return the maximum number of instances that can be formed.
//!
//! Example 1:
//!
//! Input: text = "nlaebolko"
//! Output: 1
//!
//! Example 2:
//!
//! Input: text = "loonbalxballpoon"
//! Output: 2
//!
//! Example 3:
//!
//! Input: text = "leetcode"
//! Output: 0
//!
//! Constraints:
//!
//!     1 <= text.length <= 104
//!     text consists of lower case English letters only.

use std::collections::HashMap;

/// time O(n) where n is text.len() because we visit every character in `text`.
/// space O(1) because even though we have a `HashMap`, we only store at max 5 characters in it.
pub fn max_number_of_balloons(text: String) -> i32 {
    // Maps a character to how many times it appears in `text`.
    let mut letter_count = HashMap::new();

    // Count how many times the characters in balloon appear.
    for character in text.chars() {
        if matches!(character, 'b' | 'a' | 'l' | 'o' | 'n') {
            let count = letter_count.entry(character).or_insert(0);
            *count += 1;
        }
    }

    let unique_letters_in_balloon = 5;

    if letter_count.len() != unique_letters_in_balloon {
        return 0;
    }

    // The max number of times we can build the word balloon is the amount
    // of times that the character that appears the least appears.
    let (_, num_of_ballons) = letter_count
        .into_iter()
        .map(|(character, count)| {
            // The characters 'l' and 'o' need to appear twice to build the word balloon.
            if matches!(character, 'l' | 'o') {
                (character, (count as f32 / 2.0) as i32)
            } else {
                (character, count)
            }
        })
        .min()
        .unwrap();

    num_of_ballons
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_number_of_balloons() {
        let tests = vec![("nlaebolko", 1), ("loonbalxballpoon", 2), ("leetcode", 0)];

        for (input, expected) in tests {
            assert_eq!(expected, max_number_of_balloons(String::from(input)));
        }
    }
}
