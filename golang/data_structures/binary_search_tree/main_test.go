package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBinarySearchTree_IsLeaf(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected bool
	}{
		{
			tree:     nil,
			expected: false,
		},
		{
			tree:     NewBinarySearchTree(3),
			expected: true,
		},
		{
			tree:     NewBinarySearchTree(3).Insert(2).Insert(4),
			expected: false,
		},
	}

	for _, tt := range tests {
		actual := tt.tree.IsLeaf()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Contains(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		target   int
		expected bool
	}{
		{
			tree:     nil,
			target:   10,
			expected: false,
		},
		{
			tree: &BinarySearchTree{
				Data:  10,
				Left:  nil,
				Right: nil,
			},
			target:   48,
			expected: false,
		},
		{
			tree: &BinarySearchTree{
				Data:  10,
				Left:  nil,
				Right: nil,
			},
			target:   10,
			expected: true,
		},
		{
			tree: &BinarySearchTree{
				Data: 0,
				Left: &BinarySearchTree{
					Data:  -1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
			},
			target:   -1,
			expected: true,
		},
	}

	for _, tt := range tests {
		actualRecursive := tt.tree.Contains(tt.target)
		actualIterative := tt.tree.ContainsIterative(tt.target)

		assert.Equal(t, tt.expected, actualRecursive)
		assert.Equal(t, tt.expected, actualIterative)
	}
}

func TestBinarySearchTree_Insert(t *testing.T) {
	t.Parallel()

	tests := []struct {
		recursiveInsertTree *BinarySearchTree
		iterativeInsertTree *BinarySearchTree
		expected            *BinarySearchTree
	}{
		{
			recursiveInsertTree: NewBinarySearchTree(32).Insert(12),
			iterativeInsertTree: NewBinarySearchTree(32).InsertIterative(12),
			expected: &BinarySearchTree{
				Data: 32,
				Left: &BinarySearchTree{
					Data:  12,
					Left:  nil,
					Right: nil,
				},
				Right: nil,
			},
		},
		{
			recursiveInsertTree: NewBinarySearchTree(32).Insert(42),
			iterativeInsertTree: NewBinarySearchTree(32).InsertIterative(42),
			expected: &BinarySearchTree{
				Data: 32,
				Left: nil,
				Right: &BinarySearchTree{
					Data:  42,
					Left:  nil,
					Right: nil,
				},
			},
		},
		{
			recursiveInsertTree: NewBinarySearchTree(0).Insert(1).Insert(2),
			iterativeInsertTree: NewBinarySearchTree(0).InsertIterative(1).InsertIterative(2),
			expected: &BinarySearchTree{
				Data: 0,
				Left: nil,
				Right: &BinarySearchTree{
					Data: 1,
					Left: nil,
					Right: &BinarySearchTree{
						Data:  2,
						Left:  nil,
						Right: nil,
					},
				},
			},
		},
		{
			recursiveInsertTree: NewBinarySearchTree(0).Insert(-10).Insert(10),
			iterativeInsertTree: NewBinarySearchTree(0).InsertIterative(-10).InsertIterative(10),
			expected: &BinarySearchTree{
				Data: 0,
				Left: &BinarySearchTree{
					Data:  -10,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
		},
		{
			recursiveInsertTree: NewBinarySearchTree(0).Insert(-5).Insert(5).Insert(-10).Insert(10).Insert(30),
			iterativeInsertTree: NewBinarySearchTree(0).InsertIterative(-5).InsertIterative(5).InsertIterative(-10).InsertIterative(10).InsertIterative(30),
			expected: &BinarySearchTree{
				Data: 0,
				Left: &BinarySearchTree{
					Data: -5,
					Left: &BinarySearchTree{
						Data:  -10,
						Left:  nil,
						Right: nil,
					},
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data: 5,
					Left: nil,
					Right: &BinarySearchTree{
						Data: 10,
						Left: nil,
						Right: &BinarySearchTree{
							Data:  30,
							Left:  nil,
							Right: nil,
						},
					},
				},
			},
		},
	}

	for _, tt := range tests {
		assert.Equal(t, tt.expected, tt.recursiveInsertTree)
		assert.Equal(t, tt.expected, tt.iterativeInsertTree)
	}
}

func TestBinarySearchTree_CountNodes(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected int
	}{
		{
			tree:     nil,
			expected: 0,
		},
		{
			tree:     NewBinarySearchTree(100),
			expected: 1,
		},
		{
			tree:     NewBinarySearchTree(0).Insert(-1).Insert(2),
			expected: 3,
		},
	}
	for _, tt := range tests {
		actual := tt.tree.CountNodes()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Height(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected int
	}{
		{
			tree:     nil,
			expected: 0,
		},
		{
			tree:     NewBinarySearchTree(100),
			expected: 1,
		},
		{
			tree:     NewBinarySearchTree(0).Insert(-1).Insert(2).Insert(3).Insert(-2),
			expected: 3,
		},
		{
			tree:     NewBinarySearchTree(4).Insert(3).Insert(2).Insert(1).Insert(0),
			expected: 5,
		},
	}
	for _, tt := range tests {
		assert.Equal(t, tt.expected, tt.tree.Height())
		assert.Equal(t, tt.expected, tt.tree.HeightIterative())
	}
}

func TestBinarySearchTree_InOrder(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected []int
	}{
		{
			tree:     nil,
			expected: make([]int, 0),
		},
		{
			tree:     NewBinarySearchTree(4).Insert(2).Insert(5).Insert(1).Insert(3),
			expected: []int{1, 2, 3, 4, 5},
		},
	}
	for _, tt := range tests {
		numbers := make([]int, 0)

		tt.tree.InOrder(func(tree *BinarySearchTree) {
			numbers = append(numbers, tree.Data)
		})

		assert.Equal(t, tt.expected, numbers)
	}
}

func TestBinarySearchTree_PostOrder(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected []int
	}{
		{
			tree:     nil,
			expected: make([]int, 0),
		},
		{
			tree:     NewBinarySearchTree(4).Insert(2).Insert(5).Insert(1).Insert(3),
			expected: []int{1, 3, 2, 5, 4},
		},
	}
	for _, tt := range tests {
		numbers := make([]int, 0)

		tt.tree.PostOrder(func(tree *BinarySearchTree) {
			numbers = append(numbers, tree.Data)
		})

		assert.Equal(t, tt.expected, numbers)
	}
}

func TestBinarySearchTree_HasPathSum(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		target   int
		expected bool
	}{
		{
			tree:     nil,
			target:   27,
			expected: false,
		},
		{
			tree:     NewBinarySearchTree(3),
			target:   0,
			expected: false,
		},
		{
			tree:     NewBinarySearchTree(3),
			target:   3,
			expected: true,
		},
		{
			tree: NewBinarySearchTree(1).Insert(2).Insert(3).
				Insert(-10).Insert(0).Insert(-5),
			target:   6,
			expected: true,
		},
	}

	for _, tt := range tests {
		actual := tt.tree.HasPathSum(tt.target)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_LeafPaths(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected [][]int
	}{
		{
			tree:     nil,
			expected: make([][]int, 0),
		},
		{
			tree: &BinarySearchTree{
				Data:  0,
				Left:  nil,
				Right: nil,
			},
			expected: [][]int{
				{0},
			},
		},
		{
			tree: &BinarySearchTree{
				Data: 0,
				Left: &BinarySearchTree{
					Data: -5,
					Left: &BinarySearchTree{
						Data: -10,
						Left: &BinarySearchTree{
							Data:  -8,
							Left:  nil,
							Right: nil,
						},
						Right: &BinarySearchTree{
							Data:  -15,
							Left:  nil,
							Right: nil,
						},
					},
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data: 5,
					Left: &BinarySearchTree{
						Data:  4,
						Left:  nil,
						Right: nil,
					},
					Right: &BinarySearchTree{
						Data:  10,
						Left:  nil,
						Right: nil,
					},
				},
			},
			expected: [][]int{
				{0, -5, -10, -8},
				{0, -5, -10, -15},
				{0, 5, 4},
				{0, 5, 10},
			},
		},
	}

	for _, tt := range tests {
		actual := tt.tree.LeafPaths()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Mirror(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected *BinarySearchTree
	}{
		{
			tree:     nil,
			expected: nil,
		},
		{
			tree: &BinarySearchTree{
				Data: 4,
				Left: &BinarySearchTree{
					Data: 2,
					Left: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
					Right: &BinarySearchTree{
						Data:  3,
						Left:  nil,
						Right: nil,
					},
				},
				Right: &BinarySearchTree{
					Data:  5,
					Left:  nil,
					Right: nil,
				},
			},
			expected: &BinarySearchTree{
				Data: 4,
				Left: &BinarySearchTree{
					Data:  5,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data: 2,
					Left: &BinarySearchTree{
						Data:  3,
						Left:  nil,
						Right: nil,
					},
					Right: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
				},
			},
		},
	}
	for _, tt := range tests {
		actual := tt.tree.Mirror()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_NodeMap(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		f        func(*BinarySearchTree) *BinarySearchTree
		expected *BinarySearchTree
	}{
		{
			tree: nil,
			f: func(tree *BinarySearchTree) *BinarySearchTree {
				panic("oops")
			},
			expected: nil,
		},
		{
			tree: NewBinarySearchTree(2).Insert(1).Insert(3),
			f: func(tree *BinarySearchTree) *BinarySearchTree {
				return &BinarySearchTree{
					Data:  tree.Data * 2,
					Left:  tree.Left,
					Right: tree.Right,
				}
			},
			expected: NewBinarySearchTree(4).Insert(2).Insert(6),
		},
	}
	for _, tt := range tests {
		actual := tt.tree.NodeMap(tt.f)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Map(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		f        func(int) int
		expected *BinarySearchTree
	}{
		{
			tree: nil,
			f: func(data int) int {
				panic("oops")
			},
			expected: nil,
		},
		{
			tree: NewBinarySearchTree(2).Insert(1).Insert(3),
			f: func(data int) int {
				return data * 2
			},
			expected: NewBinarySearchTree(4).Insert(2).Insert(6),
		},
	}
	for _, tt := range tests {
		actual := tt.tree.Map(tt.f)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_DoubleTree(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected *BinarySearchTree
	}{
		{
			tree:     nil,
			expected: nil,
		},
		{
			tree: &BinarySearchTree{
				Data: 2,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  3,
					Left:  nil,
					Right: nil,
				},
			},
			expected: &BinarySearchTree{
				Data: 2,
				Left: &BinarySearchTree{
					Data: 2,
					Left: &BinarySearchTree{
						Data: 1,
						Left: &BinarySearchTree{
							Data:  1,
							Left:  nil,
							Right: nil,
						},
						Right: nil,
					},
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data: 3,
					Left: &BinarySearchTree{
						Data:  3,
						Left:  nil,
						Right: nil,
					},
					Right: nil,
				},
			},
		},
	}

	for _, tt := range tests {
		actual := tt.tree.DoubleTree()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_IsEqual(t *testing.T) {
	t.Parallel()

	tests := []struct {
		a        *BinarySearchTree
		b        *BinarySearchTree
		expected bool
	}{
		{
			a:        nil,
			b:        nil,
			expected: true,
		},
		{
			a:        nil,
			b:        NewBinarySearchTree(32),
			expected: false,
		},
		{
			a:        NewBinarySearchTree(32),
			b:        nil,
			expected: false,
		},
		{
			a: &BinarySearchTree{
				Data: 1,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: nil,
			},
			b: &BinarySearchTree{
				Data:  1,
				Left:  nil,
				Right: nil,
			},
			expected: false,
		},
		{
			a: &BinarySearchTree{
				Data:  1,
				Left:  nil,
				Right: nil,
			},
			b: &BinarySearchTree{
				Data:  1,
				Left:  nil,
				Right: nil,
			},
			expected: true,
		},
		{
			a: &BinarySearchTree{
				Data: 1,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  3,
					Left:  nil,
					Right: nil,
				},
			},
			b: &BinarySearchTree{
				Data: 1,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  3,
					Left:  nil,
					Right: nil,
				},
			},
			expected: true,
		},
	}
	for _, tt := range tests {
		actual := tt.a.IsEqual(tt.b)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_IsBalanced(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected bool
	}{
		{
			tree:     nil,
			expected: true,
		},
		{
			tree: &BinarySearchTree{
				Data:  1,
				Left:  nil,
				Right: nil,
			},
			expected: true,
		},
		{
			tree: &BinarySearchTree{
				Data: 1,
				Left: &BinarySearchTree{
					Data:  2,
					Left:  nil,
					Right: nil,
				},
			},
			expected: false,
		},
		{
			tree: &BinarySearchTree{
				Data: 1,
				Left: &BinarySearchTree{
					Data:  -1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  0,
					Left:  nil,
					Right: nil,
				},
			},
			expected: false,
		},
	}
	for _, tt := range tests {
		actual := tt.tree.IsBalanced()

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Fold(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree         *BinarySearchTree
		initialValue interface{}
		f            func(interface{}, interface{}, interface{}) interface{}
		expected     int
	}{
		{
			tree:         nil,
			initialValue: 0,
			f: func(interface{}, interface{}, interface{}) interface{} {
				panic("oops")
			},
			expected: 0,
		},
		{
			tree:         NewBinarySearchTree(2).Insert(1).Insert(3),
			initialValue: 0,
			f: func(accum interface{}, left interface{}, right interface{}) interface{} {
				a := accum.(int)
				l := left.(int)
				r := right.(int)

				return a + l + r
			},
			expected: 6,
		},
	}

	for _, tt := range tests {
		actual := tt.tree.Fold(tt.initialValue, tt.f)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_Remove(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		target   int
		expected *BinarySearchTree
	}{
		{
			tree:     nil,
			target:   1,
			expected: nil,
		},
		{
			tree:     NewBinarySearchTree(1),
			target:   1,
			expected: nil,
		},
		{
			tree: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data: 3,
					Left: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
			target: 3,
			expected: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
		},
		{
			tree: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data: 3,
					Left: nil,
					Right: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
			target: 3,
			expected: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data:  1,
					Left:  nil,
					Right: nil,
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
		},
		{
			tree: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data: 3,
					Left: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
					Right: &BinarySearchTree{
						Data: 4,
						Left: &BinarySearchTree{
							Data:  2,
							Left:  nil,
							Right: nil,
						},
						Right: nil,
					},
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
			target: 3,
			expected: &BinarySearchTree{
				Data: 5,
				Left: &BinarySearchTree{
					Data: 2,
					Left: &BinarySearchTree{
						Data:  1,
						Left:  nil,
						Right: nil,
					},
					Right: &BinarySearchTree{
						Data:  4,
						Left:  nil,
						Right: nil,
					},
				},
				Right: &BinarySearchTree{
					Data:  10,
					Left:  nil,
					Right: nil,
				},
			},
		},
	}
	for _, tt := range tests {
		actual := tt.tree.Remove(tt.target)

		assert.Equal(t, tt.expected, actual)
	}
}

func TestBinarySearchTree_MinNode(t *testing.T) {
	t.Parallel()

	tests := []struct {
		tree     *BinarySearchTree
		expected *BinarySearchTree
	}{
		{
			tree:     NewBinarySearchTree(0),
			expected: NewBinarySearchTree(0),
		},
		{
			tree:     NewBinarySearchTree(2).Insert(10).Insert(0).Insert(-3).Insert(-10),
			expected: NewBinarySearchTree(-10),
		},
		{
			tree:     NewBinarySearchTree(0).Insert(1).Insert(2).Insert(3),
			expected: NewBinarySearchTree(0).Insert(1).Insert(2).Insert(3),
		},
		{
			tree:     NewBinarySearchTree(10).Insert(9).Insert(8).Insert(11).Insert(12),
			expected: NewBinarySearchTree(8),
		},
	}

	for _, tt := range tests {
		actual := tt.tree.MinNode()

		assert.Equal(t, tt.expected, actual)
	}
}
