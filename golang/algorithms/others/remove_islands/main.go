package main

/*
	Given a 2D matrix of zeros and ones that represent an image,
	replace all ones that are not connected to a one that's in a
	border of the image with zero.

	1 represents black.
	0 represents white.

	Example:

	Input:
	[
		[1, 0, 0, 0, 0, 0],
		[0, 1, 0, 1, 1, 1],
		[0, 0, 1, 0, 1, 0],
		[1, 1, 0, 0, 1, 0],
		[1, 0, 1, 1, 0, 0],
		[1, 0, 0, 0, 0, 1],
	]

	Output:
	[
		[1, 0, 0, 0, 0, 0],
		[0, 0, 0, 1, 1, 1],
		[0, 0, 0, 0, 1, 0],
		[1, 1, 0, 0, 1, 0],
		[1, 0, 0, 0, 0, 0],
		[1, 0, 0, 0, 0, 1],
	]
*/

/*
	Solution:
	For each value that's in the border of the matrix,
	if the value is a 1, mark it as value that should not be replaced with 0,
	repeat the process for each neighbor of the current value.

	After all ones that are connected to a one in a border are marked,
	go through each element in the matrix and replace it with 0 if it is not marked.
*/

func rows(graph [][]int) int {
	return len(graph)
}

func columns(graph [][]int) int {
	return len(graph[0])
}

func isOutOfBounds(graph [][]int, rowIndex, columnIndex int) bool {
	rows := rows(graph)
	columns := columns(graph)

	return rowIndex < 0 || rowIndex > rows-1 || columnIndex < 0 || columnIndex > columns-1
}

func dfs(graph [][]int, rowIndex int, columnIndex int, visited map[pair]bool, connectedToBorder map[pair]bool) {
	if isOutOfBounds(graph, rowIndex, columnIndex) {
		return
	}

	value := graph[rowIndex][columnIndex]

	position := newPair(rowIndex, columnIndex)

	if visited[position] || value == 0 {
		return
	}

	// the first call to dfs() is always from a 1 that's in a border,
	// so any time we find a 1, we know it is connected to another 1 that's
	// connected to a 1 that's in a border.
	visited[position] = true

	connectedToBorder[position] = value == 1

	dfs(graph, rowIndex-1, columnIndex, visited, connectedToBorder)
	dfs(graph, rowIndex+1, columnIndex, visited, connectedToBorder)
	dfs(graph, rowIndex, columnIndex-1, visited, connectedToBorder)
	dfs(graph, rowIndex, columnIndex+1, visited, connectedToBorder)
}

type pair struct {
	first  int
	second int
}

func newPair(first, second int) pair {
	return pair{
		first:  first,
		second: second,
	}
}

// time O(n * m + n * m)
// space O(n * m)
func RemoveIslands(graph [][]int) {
	rows := rows(graph)
	columns := columns(graph)

	visited := make(map[pair]bool)
	connectedToBorder := make(map[pair]bool)

	// go through first and last rows and search for neighboring ones whenever a one is found
	// [
	// 	[1, 0, 0, 0, 0, 0], <-- visiting columns in this row
	// 	[0, 1, 0, 1, 1, 1],
	// 	[0, 0, 1, 0, 1, 0],
	// 	[1, 1, 0, 0, 1, 0],
	// 	[1, 0, 1, 1, 0, 0],
	// 	[1, 0, 0, 0, 0, 1], <-- and this row
	// ]
	for columnIndex := 0; columnIndex < columns; columnIndex++ {
		if graph[0][columnIndex] == 1 {
			dfs(graph, 0, columnIndex, visited, connectedToBorder)
		}
		if graph[rows-1][columnIndex] == 1 {
			dfs(graph, rows-1, columnIndex, visited, connectedToBorder)
		}
	}

	// go through first and last columns and search for neighboring ones whenever a one is found
	// [
	//  visiting rows in this column
	//   |							and this column
	//   |							|
	// 	[1, 0, 0, 0, 0, 0],
	// 	[0, 1, 0, 1, 1, 1],
	// 	[0, 0, 1, 0, 1, 0],
	// 	[1, 1, 0, 0, 1, 0],
	// 	[1, 0, 1, 1, 0, 0],
	// 	[1, 0, 0, 0, 0, 1],
	// ]
	for rowIndex := 0; rowIndex < rows; rowIndex++ {
		if graph[rowIndex][0] == 1 {
			dfs(graph, rowIndex, 0, visited, connectedToBorder)
		}
		if graph[rowIndex][columns-1] == 1 {
			dfs(graph, rowIndex, columns-1, visited, connectedToBorder)
		}
	}

	for rowIndex, row := range graph {
		for columnIndex := range row {
			if connectedToBorder[newPair(rowIndex, columnIndex)] {
				continue
			}

			graph[rowIndex][columnIndex] = 0
		}
	}
}

func main() {
}
