/// 150. Evaluate Reverse Polish Notation
///
/// Evaluate the value of an arithmetic expression in Reverse Polish Notation.
///
/// Valid operators are +, -, *, and /. Each operand may be an integer or another expression.
///
/// Note that division between two integers should truncate toward zero.
///
/// It is guaranteed that the given RPN expression is always valid.
/// That means the expression would always evaluate to a result, and there will not be any division by zero operation.
///
/// Example 1:
///
/// Input: tokens = ["2","1","+","3","*"]
/// Output: 9
/// Explanation: ((2 + 1) * 3) = 9
///
/// Example 2:
///
/// Input: tokens = ["4","13","5","/","+"]
/// Output: 6
/// Explanation: (4 + (13 / 5)) = 6
///
/// Example 3:
///
/// Input: tokens = ["10","6","9","3","+","-11","*","/","*","17","+","5","+"]
/// Output: 22
/// Explanation: ((10 * (6 / ((9 + 3) * -11))) + 17) + 5
/// = ((10 * (6 / (12 * -11))) + 17) + 5
/// = ((10 * (6 / -132)) + 17) + 5
/// = ((10 * 0) + 17) + 5
/// = (0 + 17) + 5
/// = 17 + 5
/// = 22

/// time O(n)
/// space O(n)
pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = Vec::new();

    for token in tokens.into_iter() {
        if matches!(token.as_str(), "+" | "-" | "*" | "/") {
            let b = stack.pop().unwrap();
            let a = stack.pop().unwrap();

            let result = match token.as_str() {
                "+" => a + b,
                "-" => a - b,
                "*" => a * b,
                "/" => a / b,
                _ => unreachable!(),
            };

            stack.push(result);
        } else {
            stack.push(token.parse::<i32>().unwrap());
        }
    }

    stack.pop().unwrap()
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_eval_rpn() {
        let tests = vec![
            (vec!["2", "1", "+", "3", "*"], 9),
            (vec!["4", "13", "5", "/", "+"], 6),
            (
                vec![
                    "10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+",
                ],
                22,
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(
                expected,
                eval_rpn(input.into_iter().map(|rpn| rpn.to_string()).collect())
            );
        }
    }
}
