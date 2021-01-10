package main

/*
Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.


Example 1:

Input: head = [1,2,3,3,4,4,5]
Output: [1,2,5]

Example 2:

Input: head = [1,1,1,2,3]
Output: [2,3]



Constraints:

    The number of nodes in the list is in the range [0, 300].
    -100 <= Node.val <= 100
    The list is guaranteed to be sorted in ascending order.
*/

type node struct {
	val  int
	next *node
}

/*
	time: O(n)
	space: O(1)
*/
func DeleteDuplicates(head *node) *node {
	previousNode := head
	currentNode := head.next

	for currentNode != nil && currentNode.next != nil {
		isHead := previousNode == head

		if currentNode.val == currentNode.next.val {
			currentValue := currentNode.val

			for currentValue == currentNode.val {
				previousNode.next = currentNode.next
				currentNode = currentNode.next
			}

			if isHead {
				head = currentNode
			}
		} else {
			previousNode = currentNode
			currentNode = currentNode.next
		}
	}

	return head
}

func main() {

}
