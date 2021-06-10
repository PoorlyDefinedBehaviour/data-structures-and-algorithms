package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTrie_Contains(t *testing.T) {
	t.Parallel()

	tests := []struct {
		trie     *Trie
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
