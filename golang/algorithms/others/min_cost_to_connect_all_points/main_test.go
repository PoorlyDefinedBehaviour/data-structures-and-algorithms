package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_minCostConnectPoints(t *testing.T) {
	t.Parallel()

	tests := []struct {
		points   [][]int
		expected int
	}{
		{
			points:   make([][]int, 0),
			expected: 0,
		},
		{
			points:   [][]int{{0, 0}, {2, 2}, {3, 10}, {5, 2}, {7, 0}},
			expected: 20,
		},
		{
			points:   [][]int{{3, 12}, {-2, 5}, {-4, 1}},
			expected: 18,
		},
		{
			points:   [][]int{{0, 0}, {1, 1}, {1, 0}, {-1, 1}},
			expected: 4,
		},
		{
			points:   [][]int{{-1000000, -1000000}, {1000000, 1000000}},
			expected: 4000000,
		},
		{
			points:   [][]int{{0, 0}},
			expected: 0,
		},
	}

	for _, tt := range tests {
		actual := minCostConnectPoints(tt.points)

		assert.Equal(t, tt.expected, actual)
	}
}
