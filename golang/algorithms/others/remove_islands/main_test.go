package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestRemoveIslands(t *testing.T) {
	t.Parallel()

	tests := []struct {
		graph    [][]int
		expected [][]int
	}{
		{
			graph: [][]int{
				{1, 0, 0, 0, 0, 0},
				{0, 1, 0, 1, 1, 1},
				{0, 0, 1, 0, 1, 0},
				{1, 1, 0, 0, 1, 0},
				{1, 0, 1, 1, 0, 0},
				{1, 0, 0, 0, 0, 1},
			},
			expected: [][]int{
				{1, 0, 0, 0, 0, 0},
				{0, 0, 0, 1, 1, 1},
				{0, 0, 0, 0, 1, 0},
				{1, 1, 0, 0, 1, 0},
				{1, 0, 0, 0, 0, 0},
				{1, 0, 0, 0, 0, 1},
			},
		},
	}

	for _, tt := range tests {
		RemoveIslands(tt.graph)

		assert.Equal(t, tt.expected, tt.graph)
	}
}
