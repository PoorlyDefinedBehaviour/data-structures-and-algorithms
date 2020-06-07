package main

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

func deepestLeavesSum(root *TreeNode) int {
	var queue = append(make([]*TreeNode, 0), root)

	var sum int

	for len(queue) > 0 {
		var queueLength = len(queue)
		sum = 0

		for i := 0; i < queueLength; i++ {
			var currentNode = queue[0]
			queue = queue[1:]
			sum += currentNode.Val

			if currentNode.Left != nil {
				queue = append(queue, currentNode.Left)
			}
			if currentNode.Right != nil {
				queue = append(queue, currentNode.Right)
			}
		}
	}

	return sum
}

func main() {
	/*
	   Given a binary tree, return the sum of values of its deepest leaves.

	   					1
	   				2		3
	   			4		5		6
	   		7						8

	   Example 1:

	   Input: root = [1,2,3,4,5,null,6,7,null,null,null,null,8]
	   Output: 7 + 8 -> 15

	*/

	var tree = &TreeNode{Val: 1}
	tree.Left = &TreeNode{Val: 2}
	tree.Left.Left = &TreeNode{Val: 4}
	tree.Left.Right = &TreeNode{Val: 5}
	tree.Left.Left.Left = &TreeNode{Val: 7}

	tree.Right = &TreeNode{Val: 3}
	tree.Right.Left = &TreeNode{Val: 5}
	tree.Right.Right = &TreeNode{Val: 6}
	tree.Right.Right.Right = &TreeNode{Val: 8}

	println(deepestLeavesSum(tree))
}
