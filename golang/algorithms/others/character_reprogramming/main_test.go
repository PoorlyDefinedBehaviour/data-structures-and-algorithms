package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_getMaxDeletions(t *testing.T) {
	t.Parallel()

	tests := []struct {
		input    string
		expected int32
	}{
		{
			input:    "URDR",
			expected: 2,
		},
		{
			input:    "RRR",
			expected: 0,
		},
		{
			input:    "RUDRL",
			expected: 4,
		},
		{
			input:    "ULUDUULLUD",
			expected: 4,
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, getMaxDeletions(tt.input))
	}
}
