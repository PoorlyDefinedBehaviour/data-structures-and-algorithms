/// time O(n)
/// space O(1)
fn is_permutation(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a_ocurrences = [0; 26];
    let mut b_ocurrences = [0; 26];

    for (a_char, b_char) in a.as_bytes().iter().zip(b.as_bytes()) {
        let index = (*a_char - 'a' as u8) as usize;
        a_ocurrences[index] += 1;

        let index = (*b_char - 'a' as u8) as usize;
        b_ocurrences[index] += 1;
    }

    a_ocurrences == b_ocurrences
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {}
}
