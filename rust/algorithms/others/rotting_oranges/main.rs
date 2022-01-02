use std::collections::{HashSet, VecDeque};

// 994. Rotting Oranges
//
// You are given an m x n grid where each cell can have one of three values:
//
//     0 representing an empty cell,
//     1 representing a fresh orange, or
//     2 representing a rotten orange.
//
// Every minute, any fresh orange that is 4-directionally adjacent to a rotten orange becomes rotten.
//
// Return the minimum number of minutes that must elapse until no cell has a fresh orange. If this is impossible, return -1.
//
// Example 1:
//
// Input: grid = [[2,1,1],[1,1,0],[0,1,1]]
// Output: 4
//
// Example 2:
//
// Input: grid = [[2,1,1],[0,1,1],[1,0,1]]
// Output: -1
// Explanation: The orange in the bottom left corner (row 2, column 0) is never rotten, because rotting only happens 4-directionally.
//
// Example 3:
//
// Input: grid = [[0,2]]
// Output: 0
// Explanation: Since there are already no fresh oranges at minute 0, the answer is just 0.
//
// Constraints:
//
//     m == grid.length
//     n == grid[i].length
//     1 <= m, n <= 10
//     grid[i][j] is 0, 1, or 2.

/// time O(n * m)
/// space O(n * m)
pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
    const FRESH: i32 = 1;
    const ROTTEN: i32 = 2;

    let rows = grid.len();
    let columns = rows;

    let mut fresh_oranges = 0;

    let mut queue = VecDeque::new();

    for i in 0..rows {
        for j in 0..columns {
            if grid[i][j] == ROTTEN {
                queue.push_back((i as i32, j as i32));
            } else if grid[i][j] == FRESH {
                fresh_oranges += 1;
            }
        }
    }

    let mut current_minute = 0;

    let mut visited = HashSet::new();

    let directions: [(i32, i32); 4] = [
        (-1, 0), // to the left
        (0, -1), // up
        (1, 0),  // to the right
        (0, 1),  // down
    ];

    while !queue.is_empty() && fresh_oranges > 0 {
        // Visit every rotten orange that we have at the moment.
        for _ in 0..queue.len() {
            let (i, j) = queue.pop_front().unwrap();

            for (di, dj) in directions {
                let new_i = i + di;
                let new_j = j + dj;

                // If new position is not in the grid
                if new_i < 0
                    || new_i > (rows as i32) - 1
                    || new_j < 0
                    || new_j > (columns as i32) - 1
                {
                    continue;
                }

                // If the position that we want to visit later does not contain
                // a fresh orange, we do not need to visit it.
                if grid[new_i as usize][new_j as usize] != FRESH {
                    continue;
                }

                // If its a positon we have alredy visited, we wont't visit it again.
                if visited.contains(&(new_i, new_j)) {
                    continue;
                }

                // Mark grid position as visited so we can know
                // that we don't need to visit this position again later.

                visited.insert((new_i, new_j));

                fresh_oranges -= 1;
                queue.push_back((new_i, new_j));
            }
        }

        current_minute += 1;
    }

    // If we were able to make all oranges rotten, return how
    // long it took.
    // Return -1 if we were unable to make all oranges rotten.
    if fresh_oranges == 0 {
        current_minute
    } else {
        -1
    }
}

fn main() {
    println!("hello world");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oranges_rotting() {
        let tests = vec![
            (vec![vec![2, 1, 1], vec![1, 1, 0], vec![0, 1, 1]], 4),
            (vec![vec![2, 1, 1], vec![0, 1, 1], vec![1, 0, 1]], -1),
            (vec![vec![0, 2]], 0),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, oranges_rotting(input));
        }
    }
}
