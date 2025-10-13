use std::{iter::Peekable, str::Chars};

struct Split<'input> {
    input: &'input str,
    delimiter: char,
    start: usize,
    iter: Chars<'input>,
    at_delimiter: bool,
}

impl<'input> Split<'input> {
    fn new(input: &'input str, delimiter: char) -> Self {
        Self {
            input,
            delimiter,
            start: 0,
            iter: input.chars(),
            at_delimiter: false,
        }
    }
}

impl<'input> Iterator for Split<'input> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.input.is_empty() && self.start <= self.input.len() {
            self.start = self.input.len() + 1;
            return Some("");
        }
        if !self.at_delimiter && self.start >= self.input.len() {
            return None;
        }

        if self.at_delimiter {
            self.at_delimiter = false;
            self.start += 1;
        }

        let start = self.start;

        while let Some(c) = self.iter.next() {
            if c == self.delimiter {
                self.at_delimiter = true;
                break;
            }
            self.start += 1;
        }

        return Some(&self.input[start..self.start]);
    }
}

fn main() {
    let cases = ["a,b,c", "", ",a", "a,", ","];

    for input in cases {
        let expected = input.split(",").collect::<Vec<_>>();
        let actual = Split::new(input, ',').collect::<Vec<_>>();
        assert_eq!(expected, actual);
    }
}
