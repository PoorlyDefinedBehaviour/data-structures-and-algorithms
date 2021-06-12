package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestTreeNode_MaxDepth(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *TreeNode
		expected int
	}{
		{
			tree:     nil,
			expected: 0,
		},
		{
			tree: &TreeNode{
				Val:   1,
				Left:  nil,
				Right: nil,
			},
			expected: 1,
		},
		{
			tree: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val:   0,
					Left:  nil,
					Right: nil,
				},
				Right: &TreeNode{
					Val:   2,
					Left:  nil,
					Right: nil,
				},
			},
			expected: 2,
		},
		{
			tree: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val:   0,
					Left:  nil,
					Right: nil,
				},
				Right: &TreeNode{
					Val:  2,
					Left: nil,
					Right: &TreeNode{
						Val:   3,
						Left:  nil,
						Right: nil,
					},
				},
			},
			expected: 3,
		},
		{
			tree: &TreeNode{
				Val:  1,
				Left: nil,
				Right: &TreeNode{
					Val:  2,
					Left: nil,
					Right: &TreeNode{
						Val:  3,
						Left: nil,
						Right: &TreeNode{
							Val:  4,
							Left: nil,
							Right: &TreeNode{
								Val:   5,
								Left:  nil,
								Right: nil,
							},
						},
					},
				},
			},
			expected: 5,
		},
	}
	for _, tt := range tests {
		assert.Equal(t, tt.expected, tt.tree.MaxDepth())
		assert.Equal(t, tt.expected, tt.tree.MaxDepthIterativeDfs())
		assert.Equal(t, tt.expected, tt.tree.MaxDepthIterativeBfs())
	}
}
