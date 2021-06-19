package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_hasCycleUsingSet(t *testing.T) {
	t.Parallel()

	t.Run("cycle in position 1", func(t *testing.T) {
		t.Parallel()

		tail := &Node{
			Value: -4,
			Next:  nil,
		}

		list := &Node{
			Value: 3,
			Next: &Node{
				Value: 2,
				Next: &Node{
					Value: 0,
					Next:  tail,
				},
			},
		}

		tail.Next = list.Next

		assert.Equal(t, true, hasCycleUsingSet(list))
		assert.Equal(t, true, hasCycleUsingFloydsTortoiseAndHare(list))
	})

	t.Run("cycle in position 0", func(t *testing.T) {
		t.Parallel()

		tail := &Node{
			Value: 2,
			Next:  nil,
		}

		list := &Node{
			Value: 1,
			Next:  tail,
		}

		tail.Next = list

		assert.Equal(t, true, hasCycleUsingSet(list))
		assert.Equal(t, true, hasCycleUsingFloydsTortoiseAndHare(list))
	})

	t.Run("no cycle", func(t *testing.T) {
		t.Parallel()

		list := &Node{
			Value: 1,
			Next:  nil,
		}

		assert.Equal(t, false, hasCycleUsingSet(list))
		assert.Equal(t, false, hasCycleUsingFloydsTortoiseAndHare(list))
	})
}
