package minstack

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMinStack_Push(t *testing.T) {
	t.Parallel()

	minStack := New()

	minStack.Push(1)

	assert.Equal(t, 1, minStack.Top())
	assert.Equal(t, 1, minStack.Min())

	minStack.Push(2)

	assert.Equal(t, 2, minStack.Top())
	assert.Equal(t, 1, minStack.Min())

	minStack.Pop()

	assert.Equal(t, 1, minStack.Top())
	assert.Equal(t, 1, minStack.Min())

	minStack.Push(2)
	minStack.Push(-1)
	minStack.Push(3)

	assert.Equal(t, 3, minStack.Top())
	assert.Equal(t, -1, minStack.Min())

	minStack.Pop()

	assert.Equal(t, -1, minStack.Top())
	assert.Equal(t, -1, minStack.Min())

	minStack.Pop()

	assert.Equal(t, 2, minStack.Top())
	assert.Equal(t, 1, minStack.Min())

	minStack.Pop()

	assert.Equal(t, 1, minStack.Top())
	assert.Equal(t, 1, minStack.Min())

	minStack.Pop()

	minStack.Push(10)

	assert.Equal(t, 10, minStack.Top())
	assert.Equal(t, 10, minStack.Min())

	minStack.Push(5)

	assert.Equal(t, 5, minStack.Top())
	assert.Equal(t, 5, minStack.Min())

	minStack.Push(15)

	assert.Equal(t, 15, minStack.Top())
	assert.Equal(t, 5, minStack.Min())
}
