/// time O(n)
/// space O(1)
fn is_anagram(s: &str, t: &str) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut count = [0_isize; 26];

    let a_ascii_number = 'a' as u32;

    for (s_letter, t_letter) in s.chars().zip(t.chars()) {
        count[(s_letter as u32 - a_ascii_number) as usize] += 1;
        count[(t_letter as u32 - a_ascii_number) as usize] -= 1;
    }

    count.into_iter().all(|n| n == 0)
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::is_anagram;

    #[test]
    fn test_is_anagram() {
        let cases = [
            ("", "", true),
            ("anagram", "nagaram", true),
            ("rat", "car", false),
        ];

        for (s, t, expected) in cases {
            assert_eq!(expected, is_anagram(s, t));
        }
    }
}
