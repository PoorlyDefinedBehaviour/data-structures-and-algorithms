package main

import (
	"math"
)

// A binary tree is made of nodes, where each node contains
// a left pointer, a right pointer, and a data element.
// The root pointer points to the topmost node in the tree.
// The left and right pointers recursively point to smaller subtrees
// on either side. A null pointer represents a binary tree
// with no elements -- the empty tree.
// The formal recursive definition is: a binary tree is either
// empty(represented by a null pointer), or is miade of a single node,
// where the left and right pointers each point to a binary tree.
//
// type BinaryTree struct {
// 	Data  int
// 	Left  *BinaryTree
// 	Right *BinaryTree
// }
//
// A binary search tree is a type of binary tree where the nodes are arranged in order:
// for each node, all elements in its left subtree ar less-or-equal to the node,
// and all elements in its right subtree are greater than the node.
// The nodes at the bottom edge of the tree have empty subtrees and are called
// leaf nodes while the others are internal nodes.
//
// Binary search trees are fast at insert and lookup.
// On average, a binary search tree algorithm can locate a node in
// a N node tree in order log(n) time. Therefore, binary search trees
// are good for dictionary problems where the code inserts and looks up
// information indexed by some key.
// The log(n) behaviour is the average case, it's possible for a particular
// tree to be much slower if it is unbalanced.

type BinarySearchTree struct {
	Data  int
	Left  *BinarySearchTree
	Right *BinarySearchTree
}

func NewBinarySearchTree(data int) *BinarySearchTree {
	return &BinarySearchTree{
		Data:  data,
		Left:  nil,
		Right: nil,
	}
}

func (tree *BinarySearchTree) IsLeaf() bool {
	if tree == nil {
		return false
	}

	return tree.Left == nil && tree.Right == nil
}

// time O(log n)
// space O(log n) recursive calls without tail call optimization.
func (tree *BinarySearchTree) Contains(target int) bool {
	if tree == nil {
		return false
	}

	if tree.Data == target {
		return true
	}

	if target < tree.Data {
		return tree.Left.Contains(target)
	}

	return tree.Right.Contains(target)
}

// time O(log n)
// space O(1).
func (tree *BinarySearchTree) ContainsIterative(target int) bool {
	currentNode := tree

	for currentNode != nil {
		if currentNode.Data == target {
			return true
		}

		if target < currentNode.Data {
			currentNode = currentNode.Left
		} else {
			currentNode = currentNode.Right
		}
	}

	return false
}

// time O(log n)
// space O(log n).
func (tree *BinarySearchTree) Insert(data int) *BinarySearchTree {
	if tree == nil {
		return NewBinarySearchTree(data)
	}

	if data <= tree.Data {
		tree.Left = tree.Left.Insert(data)
	} else {
		tree.Right = tree.Right.Insert(data)
	}

	return tree
}

// time O(log n)
// space O(1).
func (tree *BinarySearchTree) InsertIterative(data int) *BinarySearchTree {
	if tree == nil {
		return NewBinarySearchTree(data)
	}

	currentNode := tree

	for {
		if data <= currentNode.Data {
			if currentNode.Left == nil {
				currentNode.Left = NewBinarySearchTree(data)
				break
			} else {
				currentNode = currentNode.Left
			}
		} else {
			if currentNode.Right == nil {
				currentNode.Right = NewBinarySearchTree(data)
				break
			} else {
				currentNode = currentNode.Right
			}
		}
	}

	return tree
}

// time O(n)
// space O(n).
func (tree *BinarySearchTree) CountNodes() int {
	if tree == nil {
		return 0
	}

	return 1 + tree.Left.CountNodes() + tree.Right.CountNodes()
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) Height() int {
	if tree == nil {
		return 0
	}

	return 1 + int(math.Max(float64(tree.Left.Height()), float64(tree.Right.Height())))
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) HeightIterative() int {
	if tree == nil {
		return 0
	}

	queue := []*BinarySearchTree{tree}

	height := 0

	for len(queue) > 0 {
		height++

		nodeCount := len(queue)

		for nodeCount > 0 {
			node := queue[0]
			queue = queue[1:]

			if node.Left != nil {
				queue = append(queue, node.Left)
			}

			if node.Right != nil {
				queue = append(queue, node.Right)
			}

			nodeCount--
		}
	}

	return height
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) InOrder(visitor func(tree *BinarySearchTree)) {
	if tree == nil {
		return
	}

	tree.Left.InOrder(visitor)

	visitor(tree)

	tree.Right.InOrder(visitor)
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) PostOrder(visitor func(tree *BinarySearchTree)) {
	if tree == nil {
		return
	}

	tree.Left.PostOrder(visitor)
	tree.Right.PostOrder(visitor)

	visitor(tree)
}

func (tree *BinarySearchTree) hasPathSum(target, currentSum int) bool {
	if tree == nil {
		return false
	}

	sum := currentSum + tree.Data

	if tree.IsLeaf() {
		return sum == target
	}

	return tree.Left.hasPathSum(target, sum) || tree.Right.hasPathSum(target, sum)
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) HasPathSum(target int) bool {
	return tree.hasPathSum(target, 0)
}

func (tree *BinarySearchTree) leafPaths(paths *[][]int, currentPath []int) {
	if tree == nil {
		return
	}

	currentPath = append(currentPath, tree.Data)

	if tree.IsLeaf() {
		currentPathCopy := make([]int, len(currentPath))
		copy(currentPathCopy, currentPath)

		*paths = append(*paths, currentPathCopy)
		return
	}

	tree.Left.leafPaths(paths, currentPath)
	tree.Right.leafPaths(paths, currentPath)
}

// time O(n)
// space O(n * leafs)
func (tree *BinarySearchTree) LeafPaths() [][]int {
	paths := make([][]int, 0)

	currentPath := make([]int, 0)

	tree.leafPaths(&paths, currentPath)

	return paths
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) NodeMap(f func(*BinarySearchTree) *BinarySearchTree) *BinarySearchTree {
	if tree == nil {
		return nil
	}

	newTree := f(tree)
	newTree.Left = newTree.Left.NodeMap(f)
	newTree.Right = newTree.Right.NodeMap(f)

	return newTree
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) Map(f func(int) int) *BinarySearchTree {
	return tree.NodeMap(func(tree *BinarySearchTree) *BinarySearchTree {
		return &BinarySearchTree{
			Data:  f(tree.Data),
			Left:  tree.Left,
			Right: tree.Right,
		}
	})
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) Fold(initialValue interface{}, f func(interface{}, interface{}, interface{}) interface{}) interface{} {
	if tree == nil {
		return initialValue
	}

	return f(tree.Data, tree.Left.Fold(initialValue, f), tree.Right.Fold(initialValue, f))
}

// time O(n)
// space O(1)
func (tree *BinarySearchTree) MinNode() *BinarySearchTree {
	current := tree

	for current.Left != nil {
		current = current.Left
	}

	return current
}

func (tree *BinarySearchTree) Remove(value int) *BinarySearchTree {
	if tree == nil {
		return nil
	}

	if value < tree.Data {
		tree.Left = tree.Left.Remove(value)
		return tree
	}

	if value > tree.Data {
		tree.Right = tree.Right.Remove(value)
		return tree
	}

	if tree.IsLeaf() {
		return nil
	}

	if tree.Right == nil {
		return tree.Left
	}

	if tree.Left == nil {
		return tree.Right
	}

	minNode := tree.Right.MinNode()

	tree.Data = minNode.Data
	tree.Right = tree.Right.Remove(minNode.Data)

	return tree
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) Mirror() *BinarySearchTree {
	return tree.NodeMap(func(tree *BinarySearchTree) *BinarySearchTree {
		return &BinarySearchTree{
			Data:  tree.Data,
			Left:  tree.Right,
			Right: tree.Left,
		}
	})
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) DoubleTree() *BinarySearchTree {
	if tree == nil {
		return nil
	}

	return &BinarySearchTree{
		Data: tree.Data,
		Left: &BinarySearchTree{
			Data:  tree.Data,
			Left:  tree.Left.DoubleTree(),
			Right: nil,
		},
		Right: tree.Right.DoubleTree(),
	}
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) IsEqual(other *BinarySearchTree) bool {
	if tree == nil || other == nil {
		return tree == other
	}

	if tree.Data != other.Data {
		return false
	}

	return tree.Left.IsEqual(other.Left) && tree.Right.IsEqual(other.Right)
}

// time O(n)
// space O(n)
func (tree *BinarySearchTree) IsBalanced() bool {
	if tree == nil {
		return true
	}

	if tree.Left != nil && tree.Left.Data > tree.Data {
		return false
	}

	if tree.Right != nil && tree.Right.Data < tree.Data {
		return false
	}

	return tree.Left.IsBalanced() && tree.Right.IsBalanced()
}

func main() {
}
