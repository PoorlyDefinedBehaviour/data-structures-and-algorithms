package main

/*
Given head, the head of a linked list, determine if the linked list has a cycle in it.

There is a cycle in a linked list if there is some node in the list that can be reached again by continuously following the next pointer. Internally, pos is used to denote the index of the node that tail's next pointer is connected to. Note that pos is not passed as a parameter.

Return true if there is a cycle in the linked list. Otherwise, return false.

Example 1:

Input: head = [3,2,0,-4], pos = 1
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 1st node (0-indexed).

Example 2:

Input: head = [1,2], pos = 0
Output: true
Explanation: There is a cycle in the linked list, where the tail connects to the 0th node.

Example 3:

Input: head = [1], pos = -1
Output: false
Explanation: There is no cycle in the linked list.

Constraints:

    The number of the nodes in the list is in the range [0, 104].
    -105 <= Node.val <= 105
    pos is -1 or a valid index in the linked-list.
*/

type Node struct {
	Value int
	Next  *Node
}

// time O(n)
// space O(1)
//
// Solution:
// There are two pointers:
// the pointer called tortoise advances by going to the next node in the list,
// the pointer called hare advances by skipping the next node in the list and
// going directly to the node after that. Since one pointer is faster than the other,
// the faster node will reach the slower one if there's a cycle.
func hasCycleUsingFloydsTortoiseAndHare(head *Node) bool {
	tortoise := head
	hare := tortoise

	for hare != nil && hare.Next != nil {
		tortoise = tortoise.Next
		hare = hare.Next.Next

		if tortoise == hare {
			return true
		}
	}

	return false
}

// time O(n)
// space O(n)
//
// Solution:
// Visit every node and mark it as visited,
// if the same node is visited more than once,
// there is a cycle.
func hasCycleUsingSet(head *Node) bool {
	visited := make(map[*Node]bool)

	currentNode := head

	for currentNode != nil {
		if visited[currentNode] {
			return true
		}

		visited[currentNode] = true

		currentNode = currentNode.Next
	}

	return false
}

func main() {
}
