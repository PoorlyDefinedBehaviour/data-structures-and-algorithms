package main

// 695. Max Area of Island - https://leetcode.com/problems/max-area-of-island/
//
// You are given an m x n binary matrix grid.
// An island is a group of 1's (representing land) connected 4-directionally (horizontal or vertical.)
// You may assume all four edges of the grid are surrounded by water.
//
// The area of an island is the number of cells with a value 1 in the island.
//
// Return the maximum area of an island in grid. If there is no island, return 0.
const (
	WATER = 0
	LAND  = 1
)

func countIslandArea(visited VisitedSet, grid [][]int, i int, j int) int {
	// Mark position as visited because we want to visit each position only once.
	visited.Visit(i, j)

	if grid[i][j] == WATER {
		return 0
	}

	// [grid[i][j] == LAND] so we have an island of an area that's atleast 1.
	area := 1

	// If we can go to the left of the current position.
	if i > 0 && !visited.IsVisited(i-1, j) {
		// Check if there's land to the left of the current position.
		area += countIslandArea(visited, grid, i-1, j)
	}

	// If we can go above the current position.
	if j > 0 && !visited.IsVisited(i, j-1) {
		// Check if there's land above the current position.
		area += countIslandArea(visited, grid, i, j-1)
	}

	// If we can go below current position.
	if i < len(grid)-1 && !visited.IsVisited(i+1, j) {
		// Check if there's land below the current position.
		area += countIslandArea(visited, grid, i+1, j)
	}

	// If we can go to the right of current position.
	if j < len(grid[0])-1 && !visited.IsVisited(i, j+1) {
		// Check if there's land to the right of the current position
		area += countIslandArea(visited, grid, i, j+1)
	}

	return area
}

type Position struct {
	i int
	j int
}

type VisitedSet map[Position]struct{}

func (visited VisitedSet) IsVisited(i int, j int) bool {
	_, ok := visited[Position{i: i, j: j}]
	return ok
}

func (visited VisitedSet) Visit(i int, j int) {
	visited[Position{i: i, j: j}] = struct{}{}
}

// time O(n * m) because we visit each cell in [grid] once and
// the [grid] size is n * m.
// space O(n * m) because we add every position [grid] to [visited].
func maxAreaOfIsland(grid [][]int) int {
	visited := make(VisitedSet)

	maxArea := 0

	for i := range grid {
		for j := range grid[i] {
			islandArea := countIslandArea(visited, grid, i, j)
			if islandArea > maxArea {
				maxArea = islandArea
			}
		}
	}

	if len(visited) != len(grid)*len(grid[0]) {
		panic("expected len(visited) to equal the number of position in the grid")
	}

	return maxArea
}

func main() {
}
