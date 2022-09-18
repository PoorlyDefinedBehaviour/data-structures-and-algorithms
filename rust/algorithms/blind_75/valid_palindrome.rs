/// Constraints:
///
/// 1 <= s.length <= 2 * 105
/// s consists only of printable ASCII characters.
///
/// time O(n)
/// space O(1)
pub fn is_palindrome(s: String) -> bool {
    let mut front = 0;
    let mut back = s.len() - 1;

    let s = s.as_bytes();

    while front < back {
        while front < s.len() && !s[front].is_ascii_alphanumeric() {
            front += 1;
        }

        while !s[back].is_ascii_alphanumeric() {
            back = back.saturating_sub(1);
            if back == 0 {
                break;
            }
        }

        if front > back {
            return true;
        }

        if !s[front].eq_ignore_ascii_case(&s[back]) {
            return false;
        }

        front += 1;
        back = back.saturating_sub(1)
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        // assert!(is_palindrome("A man, a plan, a canal: Panama".to_owned()));
        // assert!(is_palindrome(" ".to_owned()));
        assert!(is_palindrome(".,".to_owned()));
    }
}
