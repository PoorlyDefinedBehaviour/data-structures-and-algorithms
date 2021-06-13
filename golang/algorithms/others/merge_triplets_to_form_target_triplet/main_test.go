package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_mergeTriplets(t *testing.T) {
	t.Parallel()

	tests := []struct {
		triplets [][]int
		target   []int
		expected bool
	}{
		{
			triplets: [][]int{},
			target:   []int{3, 2, 5},
			expected: false,
		},
		{

			triplets: [][]int{{2, 5, 3}, {1, 8, 4}, {1, 7, 5}},
			target:   []int{2, 7, 5},
			expected: true,
		},
		{
			triplets: [][]int{{1, 3, 4}, {2, 5, 8}},
			target:   []int{2, 5, 8},
			expected: true,
		},
		{
			triplets: [][]int{{2, 5, 3}, {2, 3, 4}, {1, 2, 5}, {5, 2, 3}},
			target:   []int{5, 5, 5},
			expected: true,
		},
		{
			triplets: [][]int{{3, 4, 5}, {4, 5, 6}},
			target:   []int{3, 2, 5},
			expected: false,
		},
	}

	for _, tt := range tests {
		actual := mergeTriplets(tt.triplets, tt.target)

		assert.Equal(t, tt.expected, actual)
	}
}
