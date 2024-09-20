use std::io::Read;

/// In this problem we will be dealing with character sequences, often called strings. A sequence is non-trivial if it contains at least two elements.
///
/// Given a sequence s, we say that a chunk si, ... ,sj is monotone if all its characters are equal, and we say that it is maximal if this chunk cannot be extended to left or right without losing the monotonicity.
///
/// Given a sequence composed only of characters “a” and “b”, determine how many characters “a” occur in non-trivial maximal monotone chunks.
///
/// Input
/// The input consists of two lines. The first line contains a single integer N, where 1≤  N ≤105. The second line contains a string with exactly N characters, composed only of the characters “a” and “b”.
///
/// Output
/// Print a single line containing an integer representing the total number of times the character “a” occurs in non-trivial maximal monotone chunks.
///
/// Examples
/// Input:
/// 7
/// abababa
///
/// Output:
/// 0
///
/// Input:
/// 7
/// bababab
///
/// Output:
/// 0
///
/// Input:
/// 10
/// aababaaabb
///
/// Output:
/// 5

fn solve(s: &str) -> usize {
    let s = s.as_bytes();

    let mut i = 0;
    let mut count = 0;

    for c in s {
        if *c != b'a' {
            if i > 1 {
                count += i;
            }
            i = 0;
            continue;
        }

        i += 1;
    }

    count
}

fn main() {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut String::new()).unwrap();
    std::io::stdin().read_line(&mut buffer).unwrap();
    println!("{}", solve(&buffer));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(0, solve("abababa"));
        assert_eq!(0, solve("bababab"));
        assert_eq!(5, solve("aababaaabb"));
    }
}
