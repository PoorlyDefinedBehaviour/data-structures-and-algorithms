package main

// 513. Find Bottom Left Tree Value
//
// Given the root of a binary tree, return the leftmost value in the last row of the tree.
//
// Examples
//
// 1 should be returned for the following tree:
//
//         ┌───┐
//   ┌─────┤ 2 ├─────┐
//   │     └───┘     │
// ┌─▼─┐           ┌─▼─┐
// │ 1 │           │ 3 │
// └───┘           └───┘
//
//
// ---
//
// 7 should be returned because it is deeper in the tree.
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

type TreeNode struct {
	Val   int
	Left  *TreeNode
	Right *TreeNode
}

// time O(n) where n is the number of nodes in the tree
// space O(n) where n is the height of the tree
func findLeftmostNode(previous *TreeNode, node *TreeNode, level int) (*TreeNode, int) {
	if node == nil {
		return previous, level
	}

	left, leftLevel := findLeftmostNode(node, node.Left, level+1)
	right, rightLevel := findLeftmostNode(node, node.Right, level+1)

	if left == nil {
		return right, rightLevel
	}

	if right == nil {
		return left, leftLevel
	}

	if leftLevel >= rightLevel {
		return left, leftLevel
	}

	return right, rightLevel
}

func findBottomLeftValueUsingLevels(root *TreeNode) int {
	node, _ := findLeftmostNode(root, root, 0)
	return node.Val
}

// time O(n) where n is the number of nodes in the tree
// time O(n) where n is the number of nodes in the tree
func findBottomLeftValueUsingBfs(root *TreeNode) int {
	if root == nil {
		panic("tree root cannot be nil")
	}

	queue := []*TreeNode{root}

	for {
		// [node] is the first element in queue.
		node := queue[0]
		// removing [first] from the queue.
		queue = queue[1:]

		if node.Right != nil {
			queue = append(queue, node.Right)
		}

		if node.Left != nil {
			queue = append(queue, node.Left)
		}

		// If there's only one node in [queue], it means we reached the deepest node in the tree
		// since we are visiting the tree level by level.
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
		if len(queue) == 1 {
			return queue[0].Val
		}
	}
}

func main() {
}
