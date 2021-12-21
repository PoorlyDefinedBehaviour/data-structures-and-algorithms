// 329. Longest Increasing Path in a Matrix
//
// Given an m x n integers matrix, return the length of the longest increasing path in matrix.
//
// From each cell, you can either move in four directions:
// left, right, up, or down. You may not move diagonally or move outside the boundary (i.e., wrap-around is not allowed).
//
// Example 1
//
// Input: matrix = [[9,9,4],[6,6,8],[2,1,1]]
// Output: 4
// Explanation: The longest increasing path is [1, 2, 6, 9].
//
// Example 2
//
// Input: matrix = [[3,4,5],[3,2,6],[2,2,1]]
// Output: 4
// Explanation: The longest increasing path is [3, 4, 5, 6]. Moving diagonally is not allowed.
//
// Example 3
//
// Input: matrix = [[1]]
// Output: 1
//
// Constraints:
// m == matrix.length
// n == matrix[i].length
// 1 <= m, n <= 200
// 0 <= matrix[i][j] <= 231 - 1
use std::collections::HashMap;

// time O(n * m)  -- time is O(n * m) because results are getting cached
//  where n is the number of rows
//  and m is the number of columns
// space O(n * m)
pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
    if matrix.is_empty() {
        return 0;
    }

    fn dfs(
        matrix: &Vec<Vec<i32>>,
        i: usize,
        j: usize,
        cache: &mut HashMap<(usize, usize), i32>,
    ) -> i32 {
        if let Some(longest_path) = cache.get(&(i, j)) {
            return *longest_path;
        }

        let rows = matrix.len();
        let columns = matrix[0].len();

        let current_cell_value = matrix[i][j];

        // Current node counts, so the longest path is starting from the current
        // node is at least 1.
        let mut path = 1;

        if i > 0 && matrix[i - 1][j] > current_cell_value {
            path = std::cmp::max(path, 1 + dfs(matrix, i - 1, j, cache));
        }

        if j > 0 && matrix[i][j - 1] > current_cell_value {
            path = std::cmp::max(path, 1 + dfs(matrix, i, j - 1, cache));
        }

        if i < rows - 1 && matrix[i + 1][j] > current_cell_value {
            path = std::cmp::max(path, 1 + dfs(matrix, i + 1, j, cache));
        }

        if j < columns - 1 && matrix[i][j + 1] > current_cell_value {
            path = std::cmp::max(path, 1 + dfs(matrix, i, j + 1, cache));
        }

        cache.insert((i, j), path);

        path
    }

    let mut cache = HashMap::new();

    let mut max = 0;

    let rows = matrix.len();
    let columns = matrix[0].len();

    for i in 0..rows {
        for j in 0..columns {
            max = std::cmp::max(max, dfs(&matrix, i, j, &mut cache));
        }
    }

    max
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_increasing_path() {
        let tests = vec![
            (vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]], 4),
            (vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]], 4),
            (vec![vec![1]], 1),
            (vec![], 0),
        ];

        for (input, expected) in tests {
            assert_eq!(expected, longest_increasing_path(input));
        }
    }
}
