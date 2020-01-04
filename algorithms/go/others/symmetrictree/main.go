package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

/**
*	time O(n)
* space O(n)
**/
func isSimmetricRecursiveImpl(left *TreeNode, right *TreeNode) bool {
	if left == nil || right == nil {
		return left == right
	}

	return left.Val == right.Val && isSimmetricRecursiveImpl(left.Right, right.Left) && isSimmetricRecursiveImpl(left.Left, right.Right)
}

func isSymmetricRecursive(root *TreeNode) bool {
	if root == nil {
		return true
	}

	return isSimmetricRecursiveImpl(root.Left, root.Right)
}

/**
* time O(n)
* space O(n)
**/
func isSymmetricIterative(root *TreeNode) bool {
	var stack = make([]*TreeNode, 2)
	stack = append(stack, root)
	stack = append(stack, root)

	for len(stack) > 0 {
		var first = stack[0]
		var second = stack[1]
		stack = stack[2:]

		if first == nil && second == nil {
			continue
		}
		if first == nil || second == nil {
			return false
		}
		if first.Val != second.Val {
			return false
		}

		stack = append(stack, first.Left)
		stack = append(stack, second.Right)
		stack = append(stack, first.Right)
		stack = append(stack, second.Left)
	}

	return true
}

func main() {
	/*
	   Given a binary tree, check whether it is a mirror of itself (ie, symmetric around its center).

	   For example, this binary tree [1,2,2,3,4,4,3] is symmetric:

	       1
	      / \
	     2   2
	    / \ / \
	   3  4 4  3

	   But the following [1,2,2,null,3,null,3] is not:

	       1
	      / \
	     2   2
	      \   \
	      3    3
	*/

	var tree = &TreeNode{}
	tree.Left = &TreeNode{Val: 2}
	tree.Left.Left = &TreeNode{Val: 3}
	tree.Left.Right = &TreeNode{Val: 4}

	tree.Right = &TreeNode{Val: 2}
	tree.Right.Left = &TreeNode{Val: 4}
	tree.Right.Right = &TreeNode{Val: 3}

	println(isSymmetricRecursive(tree))
	println(isSymmetricIterative(tree))
}
