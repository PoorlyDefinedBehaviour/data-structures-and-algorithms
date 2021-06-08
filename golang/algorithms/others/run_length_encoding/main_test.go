package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_runLengthEncoding(t *testing.T) {
	t.Parallel()

	tests := []struct {
		input    string
		expected string
	}{
		{
			input:    "",
			expected: "",
		},
		{
			input:    "aaab",
			expected: "3a1b",
		},
		{
			input:    "ab",
			expected: "1a1b",
		},
		{
			input:    "abbbbccccc",
			expected: "1a4b5c",
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, runLengthEncoding(tt.input))
		assert.Equal(t, tt.expected, runLengthEncoding2(tt.input))
	}
}
