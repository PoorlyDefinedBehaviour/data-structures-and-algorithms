package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_TreeNode_TrimMut(t *testing.T) {
	t.Parallel()

	t.Run("example 1", func(t *testing.T) {
		node := &TreeNode{
			Value: 1,
			Left: &TreeNode{
				Value: 0,
				Left:  nil,
				Right: nil,
			},
			Right: &TreeNode{
				Value: 2,
				Left:  nil,
				Right: nil,
			},
		}

		expected := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: &TreeNode{
				Value: 2,
				Left:  nil,
				Right: nil,
			},
		}

		assert.Equal(t, expected, node.TrimMut(1, 2))
		assert.Equal(t, expected, node.Trim(1, 2))
	})

	t.Run("example 2", func(t *testing.T) {
		node := &TreeNode{
			Value: 3,
			Left: &TreeNode{
				Value: 0,
				Left:  nil,
				Right: &TreeNode{
					Value: 2,
					Left: &TreeNode{
						Value: 1,
						Left:  nil,
						Right: nil,
					},
					Right: nil,
				},
			},
			Right: &TreeNode{
				Value: 4,
				Left:  nil,
				Right: nil,
			},
		}

		expected := &TreeNode{
			Value: 3,
			Left: &TreeNode{
				Value: 2,
				Left: &TreeNode{
					Value: 1,
					Left:  nil,
					Right: nil,
				},
				Right: nil,
			},
			Right: nil,
		}

		assert.Equal(t, expected, node.TrimMut(1, 3))
		assert.Equal(t, expected, node.Trim(1, 3))
	})

	t.Run("example 3", func(t *testing.T) {
		node := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: nil,
		}

		expected := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: nil,
		}

		assert.Equal(t, expected, node.TrimMut(1, 2))
		assert.Equal(t, expected, node.Trim(1, 2))
	})

	t.Run("example 4", func(t *testing.T) {
		node := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: &TreeNode{
				Value: 2,
				Left:  nil,
				Right: nil,
			},
		}

		expected := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: &TreeNode{
				Value: 2,
				Left:  nil,
				Right: nil,
			},
		}

		assert.Equal(t, expected, node.TrimMut(1, 3))
		assert.Equal(t, expected, node.Trim(1, 3))
	})

	t.Run("example 5", func(t *testing.T) {
		node := &TreeNode{
			Value: 1,
			Left:  nil,
			Right: &TreeNode{
				Value: 2,
				Left:  nil,
				Right: nil,
			},
		}

		expected := &TreeNode{
			Value: 2,
			Left:  nil,
			Right: nil,
		}

		assert.Equal(t, expected, node.TrimMut(2, 4))
		assert.Equal(t, expected, node.Trim(2, 4))
	})
}
