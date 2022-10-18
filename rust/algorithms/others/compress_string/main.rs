use std::fmt::Write;

/// time O(n)
/// space O(n)
fn compress(s: &str) -> String {
    if s.is_empty() {
        return s.to_owned();
    }

    let mut buffer = String::new();

    let mut chars = s.chars();
    let mut current_char = chars.next().unwrap();
    let mut count = 1;

    for character in chars {
        if character == current_char {
            count += 1;
        } else {
            write!(&mut buffer, "{current_char}{count}").unwrap();
            current_char = character;
            count = 1;
        }
    }

    write!(&mut buffer, "{current_char}{count}").unwrap();

    buffer
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!("a2b1c5a3", compress("aabcccccaaa"));
    }
}
