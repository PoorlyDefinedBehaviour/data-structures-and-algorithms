package main

// 83. Remove Duplicates from Sorted List
//
// Given the head of a sorted linked list, delete all duplicates such that each element appears only once.
// Return the linked list sorted as well.
//
// Example
//
// ┌───┐    ┌───┐   ┌───┐    ┌───┐
// │ 1 ├────► 1 ├───► 2 ├────► 3 │
// └───┘    └───┘   └───┘    └───┘
//
//	     			becomes
//
//    ┌───┐     ┌───┐    ┌───┐
//    │ 1 ├─────► 2 ├────► 3 │
//    └───┘     └───┘    └───┘
//
// Constraints:
//
//     The number of nodes in the list is in the range [0, 300].
//     -100 <= Node.val <= 100
//     The list is guaranteed to be sorted in ascending order.
type ListNode struct {
	Val  int
	Next *ListNode
}

// time O(n)
// space O(1)
func deleteDuplicates(head *ListNode) *ListNode {
	if head == nil {
		return nil
	}

	current := head
	next := head.Next

	for next != nil {
		if current.Val == next.Val {
			next = next.Next
			current.Next = next
		} else {
			current = next
			next = current.Next
		}
	}

	return head
}

func main() {
}
