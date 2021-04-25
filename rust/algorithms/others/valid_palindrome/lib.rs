/// Given a string s, determine if it is a palindrome, considering only alphanumeric characters and ignoring cases.
///
/// Example 1:
/// Input: s = "A man, a plan, a canal: Panama"
/// Output: true
/// Explanation: "amanaplanacanalpanama" is a palindrome.
///
/// Example 2:
/// Input: s = "race a car"
/// Output: false
/// Explanation: "raceacar" is not a palindrome.
///
/// Constraints:
/// 1 <= s.length <= 2 * 105
/// s consists only of printable ASCII characters.

/// time O(n)
/// space O(1)
fn is_palindrome_3(string: &String) -> bool {
  let mut start = 0;
  let mut end = string.len() - 1;

  let string = string.as_bytes();

  while start < end {
    if !string[start].is_ascii_alphanumeric() {
      start += 1;
      continue;
    }

    if !string[end].is_ascii_alphanumeric() {
      end -= 1;
      continue;
    }

    if !string[start].eq_ignore_ascii_case(&string[end]) {
      return false;
    }

    start += 1;
    end -= 1;
  }

  true
}

/// time O(n)
/// space O(n)
fn is_palindrome_2(string: &String) -> bool {
  let string = string
    .chars()
    .filter_map(|character| {
      if character.is_alphanumeric() {
        Some(character.to_ascii_lowercase())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  let mut start = 0;
  let mut end = string.len() - 1;

  while start < end {
    if string[start] != string[end] {
      return false;
    }

    start += 1;
    end -= 1;
  }

  true
}

/// time O(n)
/// space O(n)
fn is_palindrome(string: &String) -> bool {
  let string = string
    .chars()
    .filter_map(|character| {
      if character.is_alphanumeric() {
        Some(character.to_ascii_lowercase())
      } else {
        None
      }
    })
    .collect::<Vec<_>>();

  string.iter().zip(string.iter().rev()).all(|(a, b)| a == b)
}

fn main() {}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_palindrome_tests() {
    let test_cases = vec![
      ("A man, a plan, a canal: Panama", true),
      ("race a car", false),
    ];

    for (input, expected) in test_cases {
      let actual = is_palindrome(&input.to_owned());
      assert_eq!(actual, expected)
    }
  }

  #[test]
  fn is_palindrome_2_tests() {
    let test_cases = vec![
      ("A man, a plan, a canal: Panama", true),
      ("race a car", false),
    ];

    for (input, expected) in test_cases {
      let actual = is_palindrome_2(&input.to_owned());
      assert_eq!(actual, expected)
    }
  }

  #[test]
  fn is_palindrome_3_tests() {
    let test_cases = vec![
      ("A man, a plan, a canal: Panama", true),
      ("race a car", false),
    ];

    for (input, expected) in test_cases {
      let actual = is_palindrome_3(&input.to_owned());
      assert_eq!(actual, expected)
    }
  }
}
