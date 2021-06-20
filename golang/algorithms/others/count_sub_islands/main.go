package main

/*
https://leetcode.com/problems/count-sub-islands/

You are given two m x n binary matrices grid1 and grid2 containing only 0's (representing water)
and 1's (representing land). An island is a group of 1's connected 4-directionally (horizontal or vertical).
 Any cells outside of the grid are considered water cells.

An island in grid2 is considered a sub-island if there is an island in grid1
that contains all the cells that make up this island in grid2.

Return the number of islands in grid2 that are considered sub-islands.

Example 1:

Input: grid1 = [[1,1,1,0,0],[0,1,1,1,1],[0,0,0,0,0],[1,0,0,0,0],[1,1,0,1,1]],
 grid2 = [[1,1,1,0,0],[0,0,1,1,1],[0,1,0,0,0],[1,0,1,1,0],[0,1,0,1,0]]
Output: 3
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are three sub-islands.

Example 2:

Input: grid1 = [[1,0,1,0,1],[1,1,1,1,1],[0,0,0,0,0],[1,1,1,1,1],[1,0,1,0,1]],
 grid2 = [[0,0,0,0,0],[1,1,1,1,1],[0,1,0,1,0],[0,1,0,1,0],[1,0,0,0,1]]
Output: 2
Explanation: In the picture above, the grid on the left is grid1 and the grid on the right is grid2.
The 1s colored red in grid2 are those considered to be part of a sub-island. There are two sub-islands.

Constraints:

    m == grid1.length == grid2.length
    n == grid1[i].length == grid2[i].length
    1 <= m, n <= 500
    grid1[i][j] and grid2[i][j] are either 0 or 1.
*/

func isOutOfBounds(grid [][]int, currentRow int, currentColumn int) bool {
	rows := len(grid)
	columns := len(grid[0])

	return currentRow < 0 || currentRow > rows-1 || currentColumn < 0 || currentColumn > columns-1
}

type position struct {
	row    int
	column int
}

const (
	water = 0
	land  = 1
)

func dfs(grid1 [][]int, grid2 [][]int, currentRow int, currentColumn int, visited map[position]bool) bool {
	currentPosition := position{row: currentRow, column: currentColumn}

	if isOutOfBounds(grid2, currentRow, currentColumn) ||
		grid2[currentRow][currentColumn] == water ||
		visited[currentPosition] {
		return true
	}

	visited[currentPosition] = true

	// we always visit neighbors to mark them as visited
	ok := grid1[currentRow][currentColumn] == land
	ok = dfs(grid1, grid2, currentRow-1, currentColumn, visited) && ok
	ok = dfs(grid1, grid2, currentRow+1, currentColumn, visited) && ok
	ok = dfs(grid1, grid2, currentRow, currentColumn-1, visited) && ok
	ok = dfs(grid1, grid2, currentRow, currentColumn+1, visited) && ok

	return ok
}

// Solution:
// Dfs both grids at the same time and compare grid2 position with
// grid1 position whenver land is found. If a position in grid2 contains land
// but grid1 contains water, it is not a sub-island.
//
// time O(n * m)
// space O(n * m)
func countSubIslands(grid1 [][]int, grid2 [][]int) int {
	visited := make(map[position]bool)

	subIslands := 0

	for rowIndex, row := range grid2 {
		for columnIndex := range row {
			if grid2[rowIndex][columnIndex] == land &&
				!visited[position{row: rowIndex, column: columnIndex}] &&
				dfs(grid1, grid2, rowIndex, columnIndex, visited) {
				subIslands++
			}
		}
	}

	return subIslands
}

func main() {
}
