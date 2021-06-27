package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_searchMatrix(t *testing.T) {
	t.Parallel()

	tests := []struct {
		matrix   [][]int
		target   int
		expected bool
	}{
		{
			matrix:   [][]int{},
			target:   3,
			expected: false,
		},
		{
			matrix:   [][]int{{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}},
			target:   3,
			expected: true,
		},
		{
			matrix:   [][]int{{1, 3, 5, 7}, {10, 11, 16, 20}, {23, 30, 34, 60}},
			target:   13,
			expected: false,
		},
	}

	for _, tt := range tests {
		actual := searchMatrix(tt.matrix, tt.target)

		assert.Equal(t, tt.expected, actual)
	}
}
