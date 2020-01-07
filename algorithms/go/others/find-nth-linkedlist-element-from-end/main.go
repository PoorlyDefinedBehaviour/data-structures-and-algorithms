package main

import "fmt"

type Node struct {
	Val  int
	Next *Node
}

/**
* time O(n)
* space O(1)
**/
func findNthElementFromEnd(node *Node, index int) *Node {
	var previousNode *Node = node
	var currentNode *Node = node

	for i := 0; i < index; i++ {
		currentNode = currentNode.Next
	}

	for currentNode.Next != nil {
		previousNode = previousNode.Next
		currentNode = currentNode.Next
	}

	return previousNode
}

func main() {
	var head = &Node{Val: 1}
	head.Next = &Node{Val: 2}
	head.Next.Next = &Node{Val: 3}
	head.Next.Next.Next = &Node{Val: 4}
	head.Next.Next.Next.Next = &Node{Val: 5}

	for i := 0; i < 5; i++ {
		fmt.Printf("%v ", findNthElementFromEnd(head, i).Val)
	}
}
