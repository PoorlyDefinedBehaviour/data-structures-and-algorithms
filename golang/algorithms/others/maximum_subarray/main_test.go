package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_maxSubArray(t *testing.T) {
	tests := []struct {
		input    []int
		expected int
	}{
		{
			input:    []int{-2, 1, -3, 4, -1, 2, 1, -5, 4},
			expected: 6,
		},
		{
			input:    []int{1},
			expected: 1,
		},
		{
			input:    []int{5, 4, -1, 7, 8},
			expected: 23,
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, maxSubArray(tt.input))
	}
}
