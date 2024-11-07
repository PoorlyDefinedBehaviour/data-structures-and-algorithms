/// Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
///
/// Example 1:
///
/// Input: n = 3
/// Output: ["((()))","(()())","(())()","()(())","()()()"]
/// Example 2:
///
/// Input: n = 1
/// Output: ["()"]
///
/// Constraints:
///
/// 1 <= n <= 8

fn generate_parenthesis(n: i32) -> Vec<String> {
    let mut results = Vec::new();
    go(n, 0, &mut results, String::new());
    results
}

fn go(to_open: i32, to_close: i32, results: &mut Vec<String>, s: String) {
    if to_open > 0 {
        go(to_open - 1, to_close + 1, results, s.clone() + "(");
    }

    if to_close > 0 {
        go(to_open, to_close - 1, results, s.clone() + ")")
    }

    if to_open == 0 && to_close == 0 {
        results.push(s);
    }
}

fn main() {
    let result = generate_parenthesis(1);
    dbg!(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_parenthesis() {
        let cases = vec![
            (1, vec!["()"]),
            (2, vec!["(())", "()()"]),
            (3, vec!["((()))", "(()())", "(())()", "()(())", "()()()"]),
        ];

        for (input, expected) in cases {
            assert_eq!(
                expected.into_iter().map(String::from).collect::<Vec<_>>(),
                generate_parenthesis(input)
            );
        }
    }
}
