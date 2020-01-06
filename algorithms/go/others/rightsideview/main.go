package main

import "fmt"

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
* time O(n)
* space O(n)
**/
func rightSideViewImpl(node *TreeNode, visibleNodes *[]int, depth int) {
	if node == nil {
		return
	}

	if len(*visibleNodes) <= depth {
		*visibleNodes = append(*visibleNodes, node.Val)
	}
	rightSideViewImpl(node.Right, visibleNodes, depth+1)
	rightSideViewImpl(node.Left, visibleNodes, depth+1)
}

func rightSideView(root *TreeNode) []int {
	var visibleNodes = make([]int, 0)

	if root == nil {
		return visibleNodes
	}

	rightSideViewImpl(root, &visibleNodes, 0)
	return visibleNodes
}

func main() {
	/*
	   Given a binary tree, imagine yourself standing on the right side of it, return the values of the nodes you can see ordered from top to bottom.

	   Example:

	   Input: [1,2,3,null,5,null,4]
	   Output: [1, 3, 4]
	   Explanation:

	      1            <---
	    /   \
	   2     3         <---
	    \     \
	     5     4       <---
	*/
	var tree = &TreeNode{Val: 1}
	tree.Left = &TreeNode{Val: 2}
	tree.Left.Right = &TreeNode{Val: 5}
	tree.Right = &TreeNode{Val: 3}
	tree.Right.Right = &TreeNode{Val: 4}

	var otherTree = &TreeNode{Val: 1}
	otherTree.Left = &TreeNode{Val: 2}
	otherTree.Right = &TreeNode{Val: 3}
	otherTree.Left.Left = &TreeNode{Val: 4}

	fmt.Printf("%v", rightSideView(tree))
	fmt.Printf("%v", rightSideView(otherTree))
}
