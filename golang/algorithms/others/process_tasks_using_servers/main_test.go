package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_assignTasks(t *testing.T) {
	t.Parallel()

	tests := []struct {
		servers  []int
		tasks    []int
		expected []int
	}{
		{
			servers:  []int{3, 3, 2},
			tasks:    []int{1, 2, 3, 2, 1, 2},
			expected: []int{2, 2, 0, 2, 1, 2},
		},
		{
			servers:  []int{5, 1, 4, 3, 2},
			tasks:    []int{2, 1, 2, 4, 5, 2, 1},
			expected: []int{1, 4, 1, 4, 1, 3, 2},
		},
	}

	for _, tt := range tests {
		actual := assignTasks(tt.servers, tt.tasks)

		assert.Equal(t, tt.expected, actual)
	}
}
