package main

// 669. Trim a Binary Search Tree
//
// Given the root of a binary search tree and
// the lowest and highest boundaries as low and high,
// trim the tree so that all its elements lies in [low, high].
// Trimming the tree should not change the relative structure of
// the elements that will remain in the tree
// (i.e., any node's descendant should remain a descendant).
// It can be proven that there is a unique answer.
//
// Return the root of the trimmed binary search tree.
// Note that the root may change depending on the given bounds.
//
// Examples
//
// Input: root = [1,0,2], low = 1, high = 2
// Output: [1,null,2]
//
// Input: root = [3,0,4,null,2,null,null,1], low = 1, high = 3
// Output: [3,2,null,1]
//
// Input: root = [1], low = 1, high = 2
// Output: [1]
//
// Input: root = [1,null,2], low = 1, high = 3
// Output: [1,null,2]
//
// Input: root = [1,null,2], low = 2, high = 4
// Output: [2]
type TreeNode struct {
	Value int
	Left  *TreeNode
	Right *TreeNode
}

// time O(n) where n is the number of nodes in the tree
// space O(h) where h is the height of the tree
func (node *TreeNode) TrimMut(low int, high int) *TreeNode {
	if node == nil {
		return nil
	}

	if node.Value < low {
		return node.Right.TrimMut(low, high)
	}

	if node.Value > high {
		return node.Left.TrimMut(low, high)
	}

	node.Left = node.Left.TrimMut(low, high)
	node.Right = node.Right.TrimMut(low, high)

	return node
}

func (node *TreeNode) Trim(low int, high int) *TreeNode {
	if node == nil {
		return node
	}

	if node.Value < low {
		return node.Right.Trim(low, high)
	}

	if node.Value > high {
		return node.Left.Trim(low, high)
	}

	return &TreeNode{
		Value: node.Value,
		Left:  node.Left.Trim(low, high),
		Right: node.Right.Trim(low, high),
	}
}

func main() {
}
