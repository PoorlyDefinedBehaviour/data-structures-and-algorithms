use std::collections::{BinaryHeap, HashMap};

// 767. Reorganize String
//
// Given a string s, rearrange the characters of s so
// that any two adjacent characters are not the same.
//
// Return any possible rearrangement of s or return "" if not possible.
//
// Example 1:
//
// Input: s = "aab"
// Output: "aba"
//
// Example 2:
//
// Input: s = "aaab"
// Output: ""
//
// Constraints:
//
//     1 <= s.length <= 500
//     s consists of lowercase English letters.

#[derive(Debug, PartialEq, Eq)]
struct Entry {
    character: char,
    occurrences: usize,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.occurrences.cmp(&other.occurrences)
    }
}

// time O(n log n)
// space O(n)
pub fn reorganize_string(s: String) -> String {
    let mut out = String::with_capacity(s.len());

    let mut occurrences = HashMap::new();

    // Count how many times each character shows up.
    // Given the input "aab" occurrences will look like this after
    // we count how many times each character shows up:
    // 'a' => 2
    // 'b' => 1
    for character in s.chars() {
        *occurrences.entry(character).or_insert(0 as usize) += 1;
    }

    // Build a priority queue where the character that shows up the most
    // is always the first.
    let mut priority_queue: BinaryHeap<Entry> = occurrences
        .iter()
        .map(|(&character, &count)| Entry {
            character,
            occurrences: count,
        })
        .collect();

    while !priority_queue.is_empty() {
        // entry will be Some(Entry) when there's a character that we can use
        // and the character is not the same as the last character used.
        // Otherwise, it will be None.
        let entry = {
            let entry_with_most_occurrences = priority_queue.pop().unwrap();

            // TODO: is this O(1)?
            // If the character that shows up the most is the same character we used in the previous
            // loop iteration, we want to take the next character that shows up the most
            // because we cannot have adjacent repeated characters in the output String.
            if out.chars().rev().nth(0) == Some(entry_with_most_occurrences.character) {
                let second_entry_with_most_occurrences = priority_queue.pop();
                priority_queue.push(entry_with_most_occurrences);
                second_entry_with_most_occurrences
            } else {
                Some(entry_with_most_occurrences)
            }
        };

        match entry {
            // entry will be None when there isn't a character that is not
            // the previously used character that we can add to the output String.
            None => return String::from(""),
            Some(entry) => {
                out.push(entry.character);

                // Since a character can appear more than once in the output String
                // as long as it is not adjacent to itself, we add it to the priority queue again
                // if we have not used it more times than it shows in the input String.
                if entry.occurrences - 1 > 0 {
                    priority_queue.push(Entry {
                        occurrences: entry.occurrences - 1,
                        ..entry
                    });
                }
            }
        }
    }

    out
}

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reorganize_string() {
        let tests = vec![("aab", "aba"), ("aaab", "")];

        for (input, expected) in tests {
            assert_eq!(
                reorganize_string(String::from(input)),
                String::from(expected)
            );
        }
    }
}
