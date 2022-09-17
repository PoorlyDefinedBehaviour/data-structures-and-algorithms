use anyhow::Result;
use std::{fmt::Write, num::ParseIntError};
use thiserror::Error;

/// time O(n)
/// space O(n) where n = sum(s.len() for each s in strs)
pub fn encode(strs: &[String]) -> Result<String, std::fmt::Error> {
    // Pre allocate string to avoid having many allocations.
    // Note that buffer will end up having a length greater than xs.len()
    // so allocations will still happen.
    let mut buffer = String::with_capacity(strs.len());

    for str in strs.iter() {
        // Note that str.len() is the number of bytes not the number of chars.
        write!(&mut buffer, "{}\r\n", str.len() as u64)?;

        buffer.push_str(str);
    }

    Ok(buffer)
}

#[derive(Debug, Error)]
enum ReadLengthError {
    #[error("expected length crlf after {0}")]
    ExpectedCRLF(String),
    #[error("unable to parse length. error={0:?}")]
    Parsing(ParseIntError),
}

#[derive(Debug)]
struct Parser<'a> {
    str: &'a str,
    position: usize,
}

impl<'a> Parser<'a> {
    fn new(str: &'a str) -> Self {
        Self { str, position: 0 }
    }

    fn is_done(&self) -> bool {
        self.position >= self.str.len()
    }

    fn peek(&self) -> Option<u8> {
        self.str.as_bytes().get(self.position).cloned()
    }

    fn next(&mut self) -> Option<u8> {
        let byte = self.str.as_bytes().get(self.position).cloned();
        self.position += 1;
        byte
    }

    fn parsed_bytes_as_string(&self) -> String {
        self.str[0..self.position].to_string()
    }

    fn read_len(&mut self) -> Result<u64, ReadLengthError> {
        let len_starts_at = self.position;

        while self.peek() != None && self.peek() != Some(b'\r') {
            let _ = self.next();
        }

        let buffer = self.str[len_starts_at..self.position].to_string();

        let next_1 = self.next();
        let next_2 = self.next();

        if next_1 != Some(b'\r') || next_2 != Some(b'\n') {
            return Err(ReadLengthError::ExpectedCRLF(self.parsed_bytes_as_string()));
        }

        let len = buffer.parse::<u64>().map_err(ReadLengthError::Parsing)?;

        Ok(len)
    }

    fn read_string(&mut self) -> Result<String, ReadLengthError> {
        let len = self.read_len()? as usize;

        let s = self.str[self.position..self.position + len].to_string();

        self.position += len;

        Ok(s)
    }
}

/// time O(n)
/// space O(n)
pub fn decode(str: &str) -> Result<Vec<String>> {
    let mut buffer = Vec::new();

    let mut parser = Parser::new(str);

    while !parser.is_done() {
        buffer.push(parser.read_string()?);
    }

    Ok(buffer)
}

fn main() {}

#[cfg(test)]
mod tests {
    use proptest::proptest;

    use super::*;

    proptest! {
      #[test]
      fn encode_decode(xs: Vec<String>) {
        let encoded = encode(&xs).expect("error encoding");
        let decoded =decode(&encoded).expect("error decoding");
        assert_eq!(xs, decoded);
      }
    }
}
