package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_dailyTemperaturesSlow(t *testing.T) {
	t.Parallel()

	tests := []struct {
		temperatures []int
		expected     []int
	}{
		{
			temperatures: []int{73, 74, 75, 71, 69, 72, 76, 73},
			expected:     []int{1, 1, 4, 2, 1, 1, 0, 0},
		},
		{
			temperatures: []int{30, 40, 50, 60},
			expected:     []int{1, 1, 1, 0},
		},
		{
			temperatures: []int{30, 60, 90},
			expected:     []int{1, 1, 0},
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, dailyTemperaturesSlow(tt.temperatures))
		assert.Equal(t, tt.expected, dailyTemperatures(tt.temperatures))
	}
}
