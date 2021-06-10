package main

import "fmt"

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

type Trie struct {
	root TrieNode
}

func DefaultTrie() *Trie {
	return &Trie{root: DefaultTrieNode()}
}

func (trie *Trie) Insert(word string) *Trie {
	currentNode := trie.root

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

func (trie *Trie) Contains(word string) bool {
	currentNode := trie.root

	for _, letter := range word {
		if _, ok := currentNode.children[letter]; !ok {
			return false
		}

		currentNode = currentNode.children[letter]
	}

	return currentNode.isEndOfWord
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
