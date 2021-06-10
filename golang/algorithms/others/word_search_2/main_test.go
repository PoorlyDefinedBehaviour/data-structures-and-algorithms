package main

import (
	"sort"
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTrie_Contains(t *testing.T) {
	t.Parallel()

	tests := []struct {
		trie     *TrieNode
		word     string
		expected bool
	}{
		{
			trie:     DefaultTrie(),
			word:     "any word",
			expected: false,
		},
		{
			trie:     DefaultTrie().Insert("hello").Insert("world").Insert("SOME").Insert("WORD"),
			word:     "notinthetrie",
			expected: false,
		},
		{
			trie:     DefaultTrie().Insert("hello").Insert("world").Insert("SOME").Insert("WORD"),
			word:     "hel",
			expected: false,
		},
		{
			trie:     DefaultTrie().Insert("hello").Insert("world").Insert("SOME").Insert("WORD"),
			word:     "hello",
			expected: true,
		},
	}

	for _, tt := range tests {
		actual := tt.trie.Contains(tt.word)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestTrieNode_NodeContainsCharacter(t *testing.T) {
	t.Parallel()

	tests := []struct {
		trie      *TrieNode
		character rune
		expected  bool
	}{
		{
			trie:      DefaultTrie(),
			character: 'e',
			expected:  false,
		},
		{
			trie:      DefaultTrie().Insert("hello"),
			character: 'e',
			expected:  false,
		},
		{
			trie:      DefaultTrie().Insert("hello"),
			character: 'h',
			expected:  true,
		},
		{
			trie:      DefaultTrie().Insert("hello"),
			character: 'e',
			expected:  false,
		},
	}

	for _, tt := range tests {
		actual := tt.trie.NodeContainsCharacter(tt.character)

		assert.Equal(t, tt.expected, actual)
	}
}

func Test_findWords(t *testing.T) {
	t.Parallel()

	tests := []struct {
		board    [][]rune
		words    []string
		expected []string
	}{
		{
			board:    [][]rune{{'o', 'a', 'a', 'n'}, {'e', 't', 'a', 'e'}, {'i', 'h', 'k', 'r'}, {'i', 'f', 'l', 'v'}},
			words:    []string{"oath", "pea", "eat", "rain"},
			expected: []string{"eat", "oath"},
		},
		{
			board:    [][]rune{{'a', 'b'}, {'c', 'd'}},
			words:    []string{"abcb"},
			expected: []string{},
		},
	}
	for _, tt := range tests {
		actual := findWords(tt.board, tt.words)

		sort.Strings(tt.expected)
		sort.Strings(actual)

		assert.Equal(t, tt.expected, actual)
	}
}
