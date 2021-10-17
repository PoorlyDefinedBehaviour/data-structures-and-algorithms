package hammingweight

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_Of(t *testing.T) {
	t.Parallel()

	tests := []struct {
		input    uint32
		expected int
	}{
		{input: 11, expected: 3},
		{input: 128, expected: 1},
		{input: 4294967293, expected: 31},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, Of(tt.input))
	}
}
