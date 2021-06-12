package main

import "math"

/*
Given the root of a binary tree, return its maximum depth.

A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.

Example 1:

Input: root = [3,9,20,null,null,15,7]
Output: 3

Example 2:

Input: root = [1,null,2]
Output: 2

Example 3:

Input: root = []
Output: 0

Example 4:

Input: root = [0]
Output: 1

Constraints:

    The number of nodes in the tree is in the range [0, 104].
    -100 <= Node.val <= 100
*/
type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// time O(n)
// space O(n)
func (node *TreeNode) MaxDepth() int {
	if node == nil {
		return 0
	}

	return 1 + int(math.Max(float64(node.Left.MaxDepth()), float64(node.Right.MaxDepth())))
}

// time O(n)
// space O(n)
func (node *TreeNode) MaxDepthIterativeDfs() int {
	if node == nil {
		return 0
	}

	type Item struct {
		node  *TreeNode
		depth int
	}

	stack := []Item{{node: node, depth: 1}}

	depth := 0

	for len(stack) > 0 {
		item := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		if item.depth > depth {
			depth = item.depth
		}

		if item.node.Left != nil {
			stack = append(stack, Item{node: item.node.Left, depth: depth + 1})
		}

		if item.node.Right != nil {
			stack = append(stack, Item{node: item.node.Right, depth: depth + 1})
		}
	}

	return depth
}

// time O(n)
// space O(n)
func (node *TreeNode) MaxDepthIterativeBfs() int {
	if node == nil {
		return 0
	}

	type Item struct {
		node  *TreeNode
		depth int
	}

	queue := []Item{{node: node, depth: 1}}

	depth := 0

	for len(queue) > 0 {
		item := queue[0]
		queue = queue[1:]

		if item.depth > depth {
			depth = item.depth
		}

		if item.node.Left != nil {
			queue = append(queue, Item{node: item.node.Left, depth: depth + 1})
		}

		if item.node.Right != nil {
			queue = append(queue, Item{node: item.node.Right, depth: depth + 1})
		}
	}

	return depth
}

func main() {

}
