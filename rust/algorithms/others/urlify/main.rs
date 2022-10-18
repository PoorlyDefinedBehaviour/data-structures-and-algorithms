/// time O(n)
/// space O(n)
fn urlify(s: &str) -> String {
    let s = s.trim();
    let mut buffer = String::new();

    for character in s.chars() {
        // Pushing into a String is O(1)~ since it uses a Vec underneath.
        if character.is_ascii_whitespace() {
            buffer.push_str("%20");
        } else {
            buffer.push(character);
        }
    }

    buffer
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!("Mr%20John%20Smith", urlify("Mr John Smith "));
    }
}
