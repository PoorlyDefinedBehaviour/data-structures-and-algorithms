// 1025. Divisor Game
// Alice and Bob take turns playing a game, with Alice starting first.
//
// Initially, there is a number n on the chalkboard. On each player's turn, that player makes a move consisting of:
//
//     Choosing any x with 0 < x < n and n % x == 0.
//     Replacing the number n on the chalkboard with n - x.
//
// Also, if a player cannot make a move, they lose the game.
//
// Return true if and only if Alice wins the game, assuming both players play optimally.
//
// Example 1:
//
// Input: n = 2
// Output: true
// Explanation: Alice chooses 1, and Bob has no more moves.
//
// Example 2:
//
// Input: n = 3
// Output: false
// Explanation: Alice chooses 1, Bob chooses 1, and Alice has no more moves.

// time O(1)
// space O(1)
//
// Alice will always win if number is even because she starts the game.
pub fn divisor_game(n: i32) -> bool {
    n % 2 == 0
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor_game() {
        let tests = vec![(2, true), (3, false)];

        for (input, expected) in tests {
            assert_eq!(expected, divisor_game(input));
        }
    }
}
