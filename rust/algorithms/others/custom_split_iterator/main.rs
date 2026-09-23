use std::{
    iter::Peekable,
    str::{CharIndices, Chars},
};

fn main() {
    let s = String::from("a1a2a3a");
    for x in s.split("a") {
        dbg!(x);
    }
}

struct SplitIterator<'a> {
    start: usize,
    pat: char,
    input: &'a str,
    it: Peekable<Chars<'a>>,
}

impl<'a> Iterator for SplitIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.input.len() {
            return None;
        }
        let mut current = self.start;
        while let Some(c) = self.it.next() {
            if c == self.pat {
                break;
            }
            current += 1;
        }
        let s = &self.input[self.start..current];
        self.start = current + 1;
        Some(s)
    }
}

fn split(input: &str, pat: char) -> SplitIterator<'_> {
    SplitIterator {
        start: 0,
        pat,
        input,
        it: input.chars().peekable(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let cases = [
            ("", 'a', vec![""]),
            ("a", 'a', vec!["", ""]),
            ("1a2a3a", 'a', vec!["1", "2", "3", ""]),
            ("a1a2a3a", 'a', vec!["", "1", "2", "3", ""]),
        ];

        for (s, pat, expected) in cases {
            assert_eq!(expected, split(s, pat).collect::<Vec<_>>());
        }
    }
}
