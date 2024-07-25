fn string_to_int(s: &str) -> u64 {
    let mut n = 0;

    for (i, character) in (0..s.len()).rev().zip(s.chars()) {
        n += char_to_int(character) * 10_u64.pow(i as u32);
    }

    n
}

fn char_to_int(c: char) -> u64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        _ => unreachable!(),
    }
}

fn main() {
    let n = string_to_int("526");
    dbg!(n);
}

#[cfg(test)]
mod tests {
    use super::*;

    use proptest::prelude::*;

    proptest! {
      #[test]
      fn test_string_to_int(n: u64) {
        let result = string_to_int(n.to_string().as_ref());
        assert_eq!(result, n);
      }
    }
}
