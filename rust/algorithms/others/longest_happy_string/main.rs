use std::collections::BinaryHeap;

/// 1405. Longest Happy String
///
/// s only contains the letters 'a', 'b', and 'c'.
/// s does not contain any of "aaa", "bbb", or "ccc" as a substring.
/// s contains at most a occurrences of the letter 'a'.
/// s contains at most b occurrences of the letter 'b'.
/// s contains at most c occurrences of the letter 'c'.
///
/// Given three integers a, b, and c, return the longest possible happy string.
/// If there are multiple longest happy strings, return any of them. If there is no such string, return the empty string "".
///
/// A substring is a contiguous sequence of characters within a string.
///
/// Example 1:
///
/// Input: a = 1, b = 1, c = 7
/// Output: "ccaccbcc"
/// Explanation: "ccbccacc" would also be a correct answer.
///
/// Example 2:
///
/// Input: a = 7, b = 1, c = 0
/// Output: "aabaa"
/// Explanation: It is the only correct answer in this case.
///
/// Constraints:
///
/// 0 <= a, b, c <= 100
/// a + b + c > 0

/// time O(n) where n is a + b + c
/// space O(1) if we don't consider the string we will return.
pub fn longest_diverse_string(a: i32, b: i32, c: i32) -> String {
    let mut buffer = String::new();

    let mut priority_queue = BinaryHeap::new();

    priority_queue.push((a, 'a'));
    priority_queue.push((b, 'b'));
    priority_queue.push((c, 'c'));

    let mut previous: Option<char> = None;
    let mut previous_previous: Option<char> = None;

    while !priority_queue.is_empty() {
        let (times_we_can_use_this_character, character) = {
            let (count, character) = priority_queue.pop().unwrap();

            match (previous_previous, previous) {
                (Some(pp_character), Some(p_character))
                    if pp_character == p_character && p_character == character =>
                {
                    match priority_queue.pop() {
                        None => return buffer,
                        Some(entry) => {
                            priority_queue.push((count, character));
                            entry
                        }
                    }
                }
                _ => (count, character),
            }
        };

        if times_we_can_use_this_character <= 0 {
            continue;
        }

        buffer.push(character);

        priority_queue.push((times_we_can_use_this_character - 1, character));

        previous_previous = previous;
        previous = Some(character);
    }

    buffer
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_diverse_string() {
        let tests = vec![(1, 1, 7, "ccbccacc"), (7, 1, 0, "aabaa")];

        for (a, b, c, expected) in tests {
            assert_eq!(String::from(expected), longest_diverse_string(a, b, c));
        }
    }
}
