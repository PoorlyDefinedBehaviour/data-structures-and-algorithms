package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_containsDuplicate(t *testing.T) {
	t.Parallel()

	assert.True(t, containsDuplicate1([]int{1, 2, 3, 1}))
	assert.False(t, containsDuplicate1([]int{1, 2, 3, 4}))
	assert.True(t, containsDuplicate1([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))

	assert.True(t, containsDuplicate2([]int{1, 2, 3, 1}))
	assert.False(t, containsDuplicate2([]int{1, 2, 3, 4}))
	assert.True(t, containsDuplicate2([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))

	assert.True(t, containsDuplicate3([]int{1, 2, 3, 1}))
	assert.False(t, containsDuplicate3([]int{1, 2, 3, 4}))
	assert.True(t, containsDuplicate3([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))
}
