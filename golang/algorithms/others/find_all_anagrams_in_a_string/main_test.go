package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_findAnagrams(t *testing.T) {
	tests := []struct {
		s        string
		p        string
		expected []int
	}{
		{
			s:        "cbaebabacd",
			p:        "abc",
			expected: []int{0, 6},
		},
		{
			s:        "abab",
			p:        "ab",
			expected: []int{0, 1, 2},
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, findAnagrams(tt.s, tt.p))
		assert.Equal(t, tt.expected, findAnagrams2(tt.s, tt.p))
	}
}
