package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_maximumRemovals(t *testing.T) {
	t.Parallel()

	tests := []struct {
		s         string
		p         string
		removable []int
		expected  int
	}{
		{
			s:         "abcacb",
			p:         "ab",
			removable: []int{3, 1, 0},
			expected:  2,
		},
		{
			s:         "abcbddddd",
			p:         "abcd",
			removable: []int{3, 2, 1, 4, 5, 6},
			expected:  1,
		},
		{
			s:         "abcab",
			p:         "abc",
			removable: []int{0, 1, 2, 3, 4},
			expected:  0,
		},
	}

	for _, tt := range tests {
		actual := maximumRemovals(tt.s, tt.p, tt.removable)

		assert.Equal(t, tt.expected, actual)
	}
}
