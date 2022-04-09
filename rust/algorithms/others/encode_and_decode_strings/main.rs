//! 659 · Encode and Decode Strings
//!
//! Description
//!
//! Design an algorithm to encode a list of strings to a string. The encoded string is then sent over the network and is decoded back to the original list of strings.
//!
//! Please implement encode and decode
//! Example
//!
//! Example1
//!
//! Input: ["lint","code","love","you"]
//!
//! Output: ["lint","code","love","you"]
//!
//! Explanation:
//!
//! One possible encode method is: "lint:;code:;love:;you"
//!
//! Example2
//!
//! Input: ["we", "say", ":", "yes"]
//!
//! Output: ["we", "say", ":", "yes"]
//!
//! Explanation:
//!
//! One possible encode method is: "we:;say:;:::;yes"

use std::{fmt::Write, num::ParseIntError};

fn encode(strings: &[impl AsRef<str>]) -> Result<String, Box<dyn std::error::Error>> {
    let mut buffer = String::new();

    for string in strings.iter().map(|s| s.as_ref()) {
        write!(&mut buffer, "{}\r\n", string.len())?;
        write!(&mut buffer, "{}", string)?;
        write!(&mut buffer, "\r\n")?
    }

    Ok(buffer)
}

fn read_len(s: &str) -> Result<(usize, usize), ParseIntError> {
    let start = 0;
    let mut end = 0;

    for character in s.chars() {
        if !character.is_digit(10) {
            break;
        }

        end += 1;
    }

    let len = s[start..end].parse::<usize>()?;

    Ok((len, end))
}

fn decode(encoded: impl AsRef<str>) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let encoded = encoded.as_ref();

    let mut out = Vec::new();

    let mut i = 0;

    while i < encoded.len() {
        let (len, characters_consumed) = read_len(&encoded[i..])?;

        // Skip the number we just consumed.
        i += characters_consumed;

        // Skip \r\n.
        i += 2;

        let value = encoded[i..i + len].to_string();

        out.push(value);

        // Skip the value we just consumed.
        i += len;

        // Skip the \r\n that comes after the value.
        i += 2;
    }

    Ok(out)
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn simple_1() -> Result<(), Box<dyn std::error::Error>> {
        let tests = vec![
            (
                vec!["lint", "code", "love", "you"],
                "4\r\nlint\r\n4\r\ncode\r\n4\r\nlove\r\n3\r\nyou\r\n",
            ),
            (vec![], ""),
            (vec!["hello"], "5\r\nhello\r\n"),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, encode(&input)?);
        }

        Ok(())
    }

    proptest! {
      #[test]
      fn simple(input: Vec<String>) {
        assert_eq!(input, decode(encode(&input).expect("error encoding")).expect("error decoding"));
      }
    }
}
