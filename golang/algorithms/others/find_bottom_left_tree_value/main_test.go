package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func Test_findBottomLeftValue(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *TreeNode
		expected int
	}{
		{
			//         ┌───┐
			//   ┌─────┤ 2 ├─────┐
			//   │     └───┘     │
			// ┌─▼─┐           ┌─▼─┐
			// │ 1 │           │ 3 │
			// └───┘           └───┘
			tree: &TreeNode{
				Val: 2,
				Left: &TreeNode{
					Val:   1,
					Left:  nil,
					Right: nil,
				},
				Right: &TreeNode{
					Val:   3,
					Left:  nil,
					Right: nil,
				},
			},
			expected: 1,
		},
		{
			//             ┌───┐
			//        ┌────┤ 1 ├────┐
			//        │    └───┘    │
			//      ┌─▼─┐         ┌─▼─┐
			//   ┌──┤ 2 │      ┌──┤ 3 ├───┐
			//   │  └───┘      │  └───┘   │
			// ┌─▼─┐           │          │
			// │ 4 │         ┌─▼─┐      ┌─▼─┐
			// └───┘       ┌─┤ 5 │      │ 6 │
			//             │ └───┘      └───┘
			//             │
			//           ┌─▼─┐
			//           │ 7 │
			//           └───┘
			tree: &TreeNode{
				Val: 1,
				Left: &TreeNode{
					Val: 2,
					Left: &TreeNode{
						Val:   4,
						Left:  nil,
						Right: nil,
					},
					Right: nil,
				},
				Right: &TreeNode{
					Val: 3,
					Left: &TreeNode{
						Val: 5,
						Left: &TreeNode{
							Val:   7,
							Left:  nil,
							Right: nil,
						},
						Right: nil,
					},
					Right: &TreeNode{
						Val:   6,
						Left:  nil,
						Right: nil,
					},
				},
			},
			expected: 7,
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, findBottomLeftValueUsingLevels(tt.tree))
		assert.Equal(t, tt.expected, findBottomLeftValueUsingBfs(tt.tree))
	}
}
