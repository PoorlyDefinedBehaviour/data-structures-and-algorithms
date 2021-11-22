// 473. Matchsticks to Square
//
// You are given an integer array matchsticks where matchsticks[i] is the length of the ith matchstick.
// You want to use all the matchsticks to make one square.
// You should not break any stick, but you can link them up, and each matchstick must be used exactly one time.
//
// Return true if you can make this square and false otherwise.
//
// Example 1
//
// Input: matchsticks = [1,1,2,2,2]
// Output: true
// Explanation: You can form a square with length 2, one side of the square came two sticks with length 1.
//
//      2
//  ┌──────┐
// 1│      │
//  ▼      │2
// 1│      │
//  ▼◄─────┘
//      2
//
// Example 2
//
// Input: matchsticks = [3,3,3,3,4]
// Output: false
// Explanation: You cannot find a way to form a square with all the matchsticks.

// time O(4^n) because the square has 4 sides and we
// build a recursive tree for each side n times where n is the length of [matchsticks].
// space O(n) where n is the length of [matchsticks]
// because we make a recursive function call for each stick in [matchsticks].
fn makesquare(matchsticks: Vec<i32>) -> bool {
    let matchsticks_sum = matchsticks.iter().sum::<i32>();

    let square_size = matchsticks_sum / 4;

    // If we can't create a square where each side is of equal length
    // using the sticks in [matchsticks] because the size of each side is a floating point number.
    if matchsticks_sum as f32 / 4.0 != square_size as f32 {
        return false;
    }

    fn go(
        stick_index: usize,
        matchsticks: &Vec<i32>,
        square_size: i32,
        square_sides: &mut [i32; 4],
    ) -> bool {
        for i in 0..square_sides.len() {
            // This side of the square is already of the size we need it to be.
            if square_sides[i] == square_size {
                continue;
            }

            // Add stick to the square side we are currently looking at.
            square_sides[i] += matchsticks[stick_index];

            // If we still have sticks to use.
            if stick_index < matchsticks.len() - 1 {
                // [go] will return true if we were able to build a square using every stick [matchsticks].
                if go(stick_index + 1, matchsticks, square_size, square_sides) {
                    return true;
                }
            }

            // If we have used every stick in [matchsticks] and
            // we have built a square where each side is equal to [square_size].
            {
                if stick_index == matchsticks.len() - 1
                    && square_sides.iter().all(|&size| size == square_size)
                {
                    return true;
                }
            }

            // Remove stick to the square side we are currently looking at
            // because this path does not lead to a solution and we will need to backtrack.
            square_sides[i] -= matchsticks[stick_index];
        }

        false
    }

    // Each square side starts with the length 0 and we have four sides.
    let mut square_sides = [0; 4];

    for i in 0..matchsticks.len() {
        if go(i, &matchsticks, square_size, &mut square_sides) {
            return true;
        }
    }

    false
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_makesquare() {
        let tests = vec![(vec![1, 1, 2, 2, 2], true), (vec![3, 3, 3, 3, 4], false)];

        for (input, expected) in tests {
            assert_eq!(expected, makesquare(input));
        }
    }
}
