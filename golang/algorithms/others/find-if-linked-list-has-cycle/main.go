package main

type Node struct {
	Value int
	Next  *Node
}

/**
* time O(n)
* space O(1)
**/
func isThereACycle(node *Node) bool {
	var fast = node
	var slow = node

	for fast != nil && fast.Next != nil {
		fast = fast.Next.Next
		slow = slow.Next

		if fast == slow {
			return true
		}
	}

	return false
}

func main() {
	/*
	    Find if a linked list has a cycle

	   	Nodes:  1 -> 2 -> 3 -> 4 -> 3 # has a cycle
	   	Nodes:  1 -> 2 -> 3 -> 4 -> 5 # has no cycle

	*/
	var head = &Node{Value: 1}
	head.Next = &Node{Value: 2}
	head.Next.Next = &Node{Value: 3}
	head.Next.Next.Next = &Node{Value: 4}
	head.Next.Next.Next = head.Next.Next

	println("isThereACycle(head)", isThereACycle(head))
}
