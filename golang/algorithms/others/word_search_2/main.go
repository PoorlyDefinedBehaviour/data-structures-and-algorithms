package main

import "fmt"

/*
Given an m x n board of characters and a list of strings words, return all words on the board.

Each word must be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once in a word.

Example 1:

Input: board = [["o","a","a","n"],["e","t","a","e"],["i","h","k","r"],["i","f","l","v"]], words = ["oath","pea","eat","rain"]
Output: ["eat","oath"]

Example 2:

Input: board = [["a","b"],["c","d"]], words = ["abcb"]
Output: []

Constraints:

    m == board.length
    n == board[i].length
    1 <= m, n <= 12
    board[i][j] is a lowercase English letter.
    1 <= words.length <= 3 * 104
    1 <= words[i].length <= 10
    words[i] consists of lowercase English letters.
    All the strings of words are unique.
*/

type TrieNode struct {
	isEndOfWord bool
	children    map[rune]TrieNode
}

func DefaultTrieNode() TrieNode {
	return TrieNode{
		isEndOfWord: false,
		children:    make(map[rune]TrieNode),
	}
}

func DefaultTrie() *TrieNode {
	return &TrieNode{isEndOfWord: false, children: make(map[rune]TrieNode)}
}

func (trie *TrieNode) Insert(word string) *TrieNode {
	currentNode := *trie

	for index, letter := range word {
		if _, ok := currentNode.children[letter]; !ok {
			node := DefaultTrieNode()
			node.isEndOfWord = index == len(word)-1

			currentNode.children[letter] = node
		}

		currentNode = currentNode.children[letter]
	}

	return trie
}

func (trie *TrieNode) Contains(word string) bool {
	currentNode := *trie

	for _, letter := range word {
		if _, ok := currentNode.children[letter]; !ok {
			return false
		}

		currentNode = currentNode.children[letter]
	}

	return currentNode.isEndOfWord
}

func (trie *TrieNode) NodeContainsCharacter(character rune) bool {
	_, ok := trie.children[character]

	return ok
}

func keys(m map[string]bool) []string {
	out := make([]string, 0, len(m))

	for key := range m {
		out = append(out, key)
	}

	return out
}

// time O(rows * columns * 4 ^ rows * colums)
// 4 because there are 4 directions we can to go in theboard
// 4 ^ rows * columns is how long a dfs can take
// rows * columns because we are doing it starting from each row and column
func findWords(board [][]rune, words []string) []string {
	type position struct {
		i int
		j int
	}

	trie := DefaultTrie()

	for _, word := range words {
		trie.Insert(word)
	}

	wordsThatCanBeBuiltFromBoard := make(map[string]bool)

	visitedPositions := make(map[position]bool)

	var dfs func(i int, j int, currentWord string, currentNode TrieNode)

	rows := len(board)
	columns := len(board[0])

	dfs = func(i int, j int, currentWord string, currentNode TrieNode) {
		if i < 0 || i >= rows || j < 0 || j >= columns {
			return
		}

		currentCharacter := board[i][j]

		if !currentNode.NodeContainsCharacter(currentCharacter) {
			return
		}

		currentNode = currentNode.children[currentCharacter]

		currentWord += string(currentCharacter)

		currentPosition := position{i, j}

		visitedPositions[currentPosition] = true

		if currentNode.isEndOfWord {
			wordsThatCanBeBuiltFromBoard[currentWord] = true
		}

		dfs(i-1, j, currentWord, currentNode)
		dfs(i+1, j, currentWord, currentNode)
		dfs(i, j-1, currentWord, currentNode)
		dfs(i, j+1, currentWord, currentNode)

		visitedPositions[currentPosition] = false
	}

	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			dfs(i, j, "", *trie)
		}
	}

	return keys(wordsThatCanBeBuiltFromBoard)
}

func main() {
	trie := DefaultTrie().
		Insert("hello").
		Insert("world").
		Insert("one").
		Insert("two").
		Insert("three")

	fmt.Println(trie.Contains("hello"))
	fmt.Println(trie.Contains("hel"))
	fmt.Println(trie.Contains("world"))
	fmt.Println(trie.Contains("hello"))
}
