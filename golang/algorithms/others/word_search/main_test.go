package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_exist(t *testing.T) {
	t.Parallel()

	tests := []struct {
		board    [][]byte
		word     string
		expected bool
	}{
		{
			board:    [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:     "ABCCED",
			expected: true,
		},
		{
			board:    [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:     "SEE",
			expected: true,
		},
		{
			board:    [][]byte{{'A', 'B', 'C', 'E'}, {'S', 'F', 'C', 'S'}, {'A', 'D', 'E', 'E'}},
			word:     "ABCB",
			expected: false,
		},
	}

	for _, tt := range tests {
		actual := exist(tt.board, tt.word)

		assert.Equal(t, tt.expected, actual)
	}
}
