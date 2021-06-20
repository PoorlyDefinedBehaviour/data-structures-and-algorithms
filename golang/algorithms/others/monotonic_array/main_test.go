package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_isMonotonic(t *testing.T) {
	t.Parallel()

	tests := []struct {
		numbers  []int
		expected bool
	}{
		{
			numbers:  []int{1, 2, 2, 3},
			expected: true,
		},
		{
			numbers:  []int{6, 5, 4, 4},
			expected: true,
		},
		{
			numbers:  []int{1, 3, 2},
			expected: false,
		},
		{
			numbers:  []int{1, 2, 4, 5},
			expected: true,
		},
		{
			numbers:  []int{1, 1, 1},
			expected: true,
		},
	}

	for _, tt := range tests {
		actual := isMonotonic(tt.numbers)

		assert.Equal(t, tt.expected, actual)
	}
}
