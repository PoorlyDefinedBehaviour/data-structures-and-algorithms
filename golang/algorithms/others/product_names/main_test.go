package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_filter(t *testing.T) {
	t.Parallel()

	tests := []struct {
		products []string
		letter   rune
		expected []string
	}{
		{
			products: []string{},
			letter:   'A',
			expected: []string{},
		},
		{
			products: []string{"Product A", "Product B", "Product C"},
			letter:   'D',
			expected: []string{},
		},
		{
			products: []string{"Product A", "Product B", "Product C"},
			letter:   'B',
			expected: []string{"Product B"},
		},
		{
			products: []string{"Product A", "Product B", "Product C"},
			letter:   'P',
			expected: []string{"Product A", "Product B", "Product C"},
		},
	}
	for _, tt := range tests {
		actual := filter(tt.products, tt.letter)

		assert.Equal(t, tt.expected, actual)
	}
}
