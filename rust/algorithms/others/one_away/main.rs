//! There are three types of edits that can be performed on strings: insert a character,
//! remove a character, or replace a character. Given two strings, write a function to check if they are
//! one edit (or zero edits) away.

/// time O(n)
/// space O(1)
fn one_away(a: &str, b: &str) -> bool {
    let len_difference = a.len().abs_diff(b.len());

    if len_difference > 1 {
        // Having strings with a length difference greater than 1
        // would quire more than one edit.
        return false;
    }

    let a_as_bytes = a.as_bytes();
    let b_as_bytes = b.as_bytes();

    let mut edits = 0;

    let mut i = 0;
    let mut j = 0;
    loop {
        if i >= a_as_bytes.len() || j >= b_as_bytes.len() {
            break;
        }

        if a_as_bytes[i] == b_as_bytes[j] {
            i += 1;
            j += 1;
            continue;
        }

        while i < a_as_bytes.len() && j < b.as_bytes().len() && a_as_bytes[i] != b_as_bytes[j] {
            if a_as_bytes.len() == b_as_bytes.len() {
                i += 1;
                j += 1;
            } else {
                i += 1;
            }

            edits += 1;
        }

        if edits > 1 {
            return false;
        }
    }

    edits <= 1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert!(one_away("pale", "ple"));
        assert!(one_away("pales", "pale"));
        assert!(one_away("pale", "bale"));
        assert!(!one_away("pale", "bae"));
        assert!(!one_away("bae", "pale"));
    }
}
