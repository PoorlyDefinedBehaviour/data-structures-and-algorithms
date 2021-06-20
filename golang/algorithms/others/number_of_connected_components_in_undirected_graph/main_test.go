package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_countComponents(t *testing.T) {
	t.Parallel()

	tests := []struct {
		n        int
		graph    [][]int
		expected int
	}{
		{
			graph:    [][]int{{0, 1}, {1, 2}, {3, 4}},
			expected: 2,
		},
	}

	for _, tt := range tests {
		actual := countComponentsUsingDfs(tt.graph)

		assert.Equal(t, tt.expected, actual)
	}
}
