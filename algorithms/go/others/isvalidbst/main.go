package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
* time O(h)
* space O(h)
**/
func isValidBST(node *TreeNode) bool {
	if node == nil {
		return true
	}

	if node.Left != nil && node.Val <= node.Left.Val {
		return false
	}
	if node.Right != nil && node.Val >= node.Right.Val {
		return false
	}
	return isValidBST(node.Left) && isValidBST(node.Right)
}

func main() {
	/*
	   Given a binary tree, determine if it is a valid binary search tree (BST).

	   Assume a BST is defined as follows:

	       The left subtree of a node contains only nodes with keys less than the node's key.
	       The right subtree of a node contains only nodes with keys greater than the node's key.
	       Both the left and right subtrees must also be binary search trees.

	   Example 1:

	       2
	      / \
	     1   3

	   Input: [2,1,3]
	   Output: true

	   Example 2:

	       5
	      / \
	     1   4
	        / \
	       3   6

	   Input: [5,1,4,null,null,3,6]
	   Output: false
	   Explanation: The root node's value is 5 but its right child's value is 4.
	*/
	var validTree = &TreeNode{Val: 2}
	validTree.Left = &TreeNode{Val: 1}
	validTree.Right = &TreeNode{Val: 3}

	var invalidTree = &TreeNode{Val: 5}
	invalidTree.Left = &TreeNode{Val: 1}
	invalidTree.Right = &TreeNode{Val: 4}
	invalidTree.Right.Left = &TreeNode{Val: 3}
	invalidTree.Right.Right = &TreeNode{Val: 6}

	println(isValidBST(validTree))
	println(isValidBST(invalidTree))
}
