//! 36. Valid Sudoku
//!
//! Determine if a 9 x 9 Sudoku board is valid. Only the filled cells need to be validated according to the following rules:
//!
//!     Each row must contain the digits 1-9 without repetition.
//!     Each column must contain the digits 1-9 without repetition.
//!     Each of the nine 3 x 3 sub-boxes of the grid must contain the digits 1-9 without repetition.
//!
//! Note:
//!
//!     A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//!     Only the filled cells need to be validated according to the mentioned rules.
//!
//! Example 1:
//!
//! Input: board =
//! [["5","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//!
//! Output: true
//!
//! Example 2:
//!
//! Input: board =
//! [["8","3",".",".","7",".",".",".","."]
//! ,["6",".",".","1","9","5",".",".","."]
//! ,[".","9","8",".",".",".",".","6","."]
//! ,["8",".",".",".","6",".",".",".","3"]
//! ,["4",".",".","8",".","3",".",".","1"]
//! ,["7",".",".",".","2",".",".",".","6"]
//! ,[".","6",".",".",".",".","2","8","."]
//! ,[".",".",".","4","1","9",".",".","5"]
//! ,[".",".",".",".","8",".",".","7","9"]]
//!
//! Output: false
//!
//! Explanation: Same as Example 1, except with the 5 in the top left corner being modified to 8.
//! Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//!
//! Constraints:
//!
//!     board.length == 9
//!     board[i].length == 9
//!     board[i][j] is a digit 1-9 or '.'.

use std::collections::{HashMap, HashSet};

pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
    let mut rows = HashMap::new();
    let mut columns = HashMap::new();
    let mut square = HashMap::new();

    for i in 0..board.len() {
        for j in 0..board.len() {
            // Skip empty positions.
            if board[i][j] == '.' {
                continue;
            }

            // Check if row contains duplicate.
            let entry = rows.entry(i).or_insert_with(HashSet::new);

            if entry.contains(&board[i][j]) {
                return false;
            }

            entry.insert(board[i][j]);

            // Check if column contains duplicate
            let entry = columns.entry(i).or_insert_with(HashSet::new);

            if entry.contains(&board[i][j]) {
                return false;
            }

            entry.insert(board[i][j]);

            // Check 3x3 square.
            let entry = square.entry((i / 3, j / 3)).or_insert_with(HashSet::new);

            if entry.contains(&board[i][j]) {
                return false;
            }

            entry.insert(board[i][j]);
        }
    }

    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_sudoku() {
        let tests = vec![
            (
                vec![
                    vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                true,
            ),
            (
                vec![
                    vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
                    vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
                    vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
                    vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
                    vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
                    vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
                    vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
                    vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
                    vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
                ],
                false,
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, is_valid_sudoku(input));
        }
    }
}
