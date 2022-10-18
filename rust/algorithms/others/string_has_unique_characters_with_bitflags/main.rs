/// Assuming the string only has ascii characters.
///
/// time O(n)
/// space O(1)
fn string_has_unique_characters(s: &str) -> bool {
    let mut seen = 0_u32;

    for character in s.as_bytes() {
        let index = (character - 'a' as u8) as u32;

        if seen & 1 << index > 0 {
            return false;
        }

        seen |= 1_u32 << index;
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert!(string_has_unique_characters("abc"));
        assert!(string_has_unique_characters("a"));
        assert!(string_has_unique_characters(""));
        assert!(!string_has_unique_characters("aabc"));
        assert!(!string_has_unique_characters("abcc"));
        assert!(!string_has_unique_characters("abbc"));
    }
}
