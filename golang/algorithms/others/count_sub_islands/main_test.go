package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_countSubIslands(t *testing.T) {
	t.Parallel()

	tests := []struct {
		grid1    [][]int
		grid2    [][]int
		expected int
	}{
		{
			grid1:    [][]int{{1, 1, 1, 0, 0}, {0, 1, 1, 1, 1}, {0, 0, 0, 0, 0}, {1, 0, 0, 0, 0}, {1, 1, 0, 1, 1}},
			grid2:    [][]int{{1, 1, 1, 0, 0}, {0, 0, 1, 1, 1}, {0, 1, 0, 0, 0}, {1, 0, 1, 1, 0}, {0, 1, 0, 1, 0}},
			expected: 3,
		},
		{
			grid1:    [][]int{{1, 0, 1, 0, 1}, {1, 1, 1, 1, 1}, {0, 0, 0, 0, 0}, {1, 1, 1, 1, 1}, {1, 0, 1, 0, 1}},
			grid2:    [][]int{{0, 0, 0, 0, 0}, {1, 1, 1, 1, 1}, {0, 1, 0, 1, 0}, {0, 1, 0, 1, 0}, {1, 0, 0, 0, 1}},
			expected: 2,
		},
	}

	for _, tt := range tests {
		actual := countSubIslands(tt.grid1, tt.grid2)

		assert.Equal(t, tt.expected, actual)
	}
}
