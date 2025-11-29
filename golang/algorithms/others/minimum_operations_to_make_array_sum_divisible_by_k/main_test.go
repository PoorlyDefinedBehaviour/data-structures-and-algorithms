package main

import (
	"testing"

	"github.com/stretchr/testify/require"
)

func TestMinOperations(t *testing.T) {
	t.Parallel()

	require.Equal(t, 4, minOperations([]int{3, 9, 7}, 5))
	require.Equal(t, 0, minOperations([]int{4, 1, 3}, 4))
	require.Equal(t, 5, minOperations([]int{3, 2}, 6))
}
