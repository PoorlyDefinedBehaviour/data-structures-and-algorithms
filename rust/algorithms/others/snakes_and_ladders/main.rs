//! 909. Snakes and Ladders
//!
//! You are given an n x n integer matrix board where
//! the cells are labeled from 1 to n2 in a Boustrophedon style starting from the bottom left of the board (i.e. board[n - 1][0])
//! and alternating direction each row.
//!
//! You start on square 1 of the board. In each move, starting from square curr, do the following:
//!
//!     Choose a destination square next with a label in the range [curr + 1, min(curr + 6, n2)].
//!         This choice simulates the result of a standard 6-sided die roll: i.e., there are always at most 6 destinations, regardless of the size of the board.
//!     If next has a snake or ladder, you must move to the destination of that snake or ladder. Otherwise, you move to next.
//!     The game ends when you reach the square n2.
//!
//! A board square on row r and column c has a snake or ladder if board[r][c] != -1.
//! The destination of that snake or ladder is board[r][c]. Squares 1 and n2 do not have a snake or ladder.
//!
//! Note that you only take a snake or ladder at most once per move.
//! If the destination to a snake or ladder is the start of another snake or ladder, you do not follow the subsequent snake or ladder.
//!
//! For example, suppose the board is [[-1,4],[-1,3]],
//! and on the first move, your destination square is 2. You follow the ladder to square 3, but do not follow the subsequent ladder to 4.
//!
//! Return the least number of moves required to reach the square n2. If it is not possible to reach the square, return -1.
//!
//! Example 1:
//!
//! Input: board = [[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,-1,-1,-1,-1,-1],[-1,35,-1,-1,13,-1],[-1,-1,-1,-1,-1,-1],[-1,15,-1,-1,-1,-1]]
//! Output: 4
//! Explanation:
//! In the beginning, you start at square 1 (at row 5, column 0).
//! You decide to move to square 2 and must take the ladder to square 15.
//! You then decide to move to square 17 and must take the snake to square 13.
//! You then decide to move to square 14 and must take the ladder to square 35.
//! You then decide to move to square 36, ending the game.
//! This is the lowest possible number of moves to reach the last square, so return 4.
//!
//! Example 2:
//!
//! Input: board = [[-1,-1],[-1,3]]
//! Output: 1
//!
//! Constraints:
//!
//!     n == board.length == board[i].length
//!     2 <= n <= 20
//!     grid[i][j] is either -1 or in the range [1, n2].
//!     The squares labeled 1 and n2 do not have any ladders or snakes.

use std::collections::{HashSet, VecDeque};

// time O(n^2) because the board size is n^2 and we may visit
// every cell in the board.
// space O(n^2) because every cell would be added to the visited set
// if we visited them all.
pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
    // Flatten the board into a list.
    let board: Vec<i32> = board.into_iter().flatten().collect();

    // Set of position indices that have already been visited.
    let mut visited = HashSet::new();

    // Queue of (position, moves taken to get to position) to visit.
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();

    // We start at the first position which takes 0 moves to get to.
    queue.push_back((1, 1));

    // While there are positions that have not visited.
    while !queue.is_empty() {
        let (position, moves) = queue.pop_front().unwrap();

        visited.insert(position);

        // We can move up to 6 squares from the current position.
        for i in 1..=6 {
            let new_position = position - 1 + i;

            // If board cell contain a ladder or snake, we will follow it.
            let new_square = if board[new_position as usize] != -1 {
                board[new_position as usize]
            } else {
                new_position
            };

            // If we reached the last position in the board  we are done.
            if new_square as usize == board.len() - 1 {
                return moves;
            }

            if !visited.contains(&new_square) {
                queue.push_back((new_square, moves + 1));
            }
        }
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snakes_and_ladders() {
        let tests = vec![
            (
                vec![
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, 35, -1, -1, 13, -1],
                    vec![-1, -1, -1, -1, -1, -1],
                    vec![-1, 15, -1, -1, -1, -1],
                ],
                4,
            ),
            (vec![vec![-1, -1], vec![-1, 3]], 1),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, snakes_and_ladders(input));
        }
    }
}
