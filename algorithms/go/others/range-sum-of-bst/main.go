package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
* time O(n)
* space O(h)
**/
func rangeSumBSTRecursive(node *TreeNode, L int, R int) int {
	if node == nil {
		return 0
	}

	if L <= node.Val && node.Val <= R {
		return node.Val + rangeSumBSTRecursive(node.Left, L, R) + rangeSumBSTRecursive(node.Right, L, R)
	}

	return rangeSumBSTRecursive(node.Left, L, R) + rangeSumBSTRecursive(node.Right, L, R)
}

/**
* time O(n)
* space O(h)
**/
func rangeSumBSTIterative(node *TreeNode, L int, R int) int {
	if node == nil {
		return 0
	}

	var queue = make([]*TreeNode, 0)
	queue = append(queue, node)
	queue = append(queue, nil)

	var result = 0

	for len(queue) > 0 {
		var first = queue[0]
		var second = queue[1]
		queue = queue[2:]

		if first != nil {
			if L <= first.Val && first.Val <= R {
				result = result + first.Val
			}

			queue = append(queue, first.Left)
			queue = append(queue, first.Right)
		}

		if second != nil {
			if L <= second.Val && second.Val <= R {
				result = result + second.Val
			}

			queue = append(queue, second.Left)
			queue = append(queue, second.Right)
		}
	}

	return result
}

func main() {
	/*
	   Given the root node of a binary search tree, return the sum of values of all nodes with value between L and R (inclusive).

	   The binary search tree is guaranteed to have unique values.

	   Example 1:

	   Input: root = [10,5,15,3,7,null,18], L = 7, R = 15
	   Output: 32

	   Example 2:

	   Input: root = [10,5,15,3,7,13,18,1,null,6], L = 6, R = 10
	   Output: 23
	*/
	var tree = &TreeNode{Val: 10}
	tree.Left = &TreeNode{Val: 5}
	tree.Right = &TreeNode{Val: 15}

	tree.Right.Right = &TreeNode{Val: 18}

	tree.Left.Left = &TreeNode{Val: 3}
	tree.Left.Right = &TreeNode{Val: 7}

	println(rangeSumBSTRecursive(tree, 7, 15))
	println(rangeSumBSTIterative(tree, 7, 15))
}
