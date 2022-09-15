use std::collections::HashMap;

/// time O(n)
/// space O(n)
pub fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }
    let mut counts = HashMap::new();

    for (s_char, t_char) in s.chars().zip(t.chars()) {
        let entry = counts.entry(s_char).or_insert(0);
        *entry += 1;

        let entry = counts.entry(t_char).or_insert(0);
        *entry -= 1;
    }

    for count in counts.values() {
        if *count != 0 {
            return false;
        }
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let tests = vec![("anagram", "nagaram", true), ("rat", "car", false)];

        for (s, t, expected) in tests {
            assert_eq!(expected, is_anagram(s.to_owned(), t.to_owned()));
        }
    }
}
