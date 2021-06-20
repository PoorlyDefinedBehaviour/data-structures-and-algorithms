package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_findDuplicateUsingSet(t *testing.T) {
	t.Parallel()

	tests := []struct {
		numbers  []int
		expected int
	}{
		{
			numbers:  []int{1, 3, 4, 2, 2},
			expected: 2,
		},
		{
			numbers:  []int{3, 1, 3, 4, 2},
			expected: 3,
		},
		{
			numbers:  []int{1, 1},
			expected: 1,
		},
		{
			numbers:  []int{1, 1, 2},
			expected: 1,
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, findDuplicateUsingSet(tt.numbers))
		assert.Equal(t, tt.expected, findDuplicateUsingFloydsAlgorithm(tt.numbers))
	}
}
