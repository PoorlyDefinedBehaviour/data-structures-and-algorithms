package main

import "strings"

/*
iven an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring.
The same letter cell may not be used more than once.

Example 1:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
Output: true

Example 2:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
Output: true

Example 3:

Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
Output: false

Constraints:

    m == board.length
    n = board[i].length
    1 <= m, n <= 6
    1 <= word.length <= 15
    board and word consists of only lowercase and uppercase English letters.

*/
func dfs(board [][]byte, rowIndex int, columnIndex int, currentLetterIndex int, word string) bool {
	/*
	 	ASFBA
	   board =
	   [["A","B","C","E"],
	   ["S","F","C","S"],
	   ["A","D","E","E"]]

	   word = "ABCCED"
	*/
	if currentLetterIndex == len(word) {
		return true
	}

	rows := len(board)
	columns := len(board[0])

	if rowIndex < 0 || rowIndex >= rows || columnIndex < 0 || columnIndex >= columns {
		return false
	}

	letterThatIWant := word[currentLetterIndex]
	currentBoardLetter := board[rowIndex][columnIndex]

	if currentBoardLetter != letterThatIWant {
		return false
	}

	board[rowIndex][columnIndex] = ' '

	wordHasBeenFound := dfs(board, rowIndex-1, columnIndex, currentLetterIndex+1, word) ||
		dfs(board, rowIndex+1, columnIndex, currentLetterIndex+1, word) ||
		dfs(board, rowIndex, columnIndex-1, currentLetterIndex+1, word) ||
		dfs(board, rowIndex, columnIndex+1, currentLetterIndex+1, word)

	board[rowIndex][columnIndex] = currentBoardLetter

	return wordHasBeenFound
}

func exist(board [][]byte, word string) bool {
	for rowIndex, row := range board {
		for columnIndex, letter := range row {
			if strings.HasPrefix(word, string(letter)) && dfs(board, rowIndex, columnIndex, 0, word) {
				return true
			}
		}
	}

	return false
}

func main() {

}
