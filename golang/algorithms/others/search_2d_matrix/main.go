package main

// Write an efficient algorithm that searches for a value in an m x n matrix.
// This matrix has the following properties:
//     Integers in each row are sorted from left to right.
//     The first integer of each row is greater than the last integer of the previous row.
//
// Example 1:
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 3
// Output: true
//
// Example 2:
//
// Input: matrix = [[1,3,5,7],[10,11,16,20],[23,30,34,60]], target = 13
// Output: false
//
// Constraints:
//
//     m == matrix.length
//     n == matrix[i].length
//     1 <= m, n <= 100
//     -104 <= matrix[i][j], target <= 104

// time O(log n)
// space O(1)
//
// Solution:
// Since matrix is sorted, binary search rows and columns at the same time.
func searchMatrix(matrix [][]int, target int) bool {
	columns := len(matrix)

	if columns == 0 {
		return false
	}

	rows := len(matrix[0])

	left := 0
	right := columns - 1

	top := 0
	bottom := rows - 1

	for left <= right && top <= bottom {
		row := (top + bottom) / 2
		middle := (left + right) / 2

		value := matrix[row][middle]

		if value == target {
			return true
		}

		if target < matrix[row][0] {
			bottom = row - 1
		} else if target > matrix[row][columns-1] {
			top = row + 1
		} else if target < value {
			right = middle - 1
		} else if target > value {
			left = middle + 1
		}
	}

	return false
}

func main() {
}
