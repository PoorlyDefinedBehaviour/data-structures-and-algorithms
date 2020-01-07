package main

type Node struct {
	Val  int
	Next *Node
}

/**
* time O(m + n)
* space O(m + n)
**/
func doesLinkedListsIntersect(first *Node, second *Node) bool {
	var visitedNodes = make(map[*Node]bool)

	for first != nil || second != nil {
		if first != nil {
			if visited, ok := visitedNodes[first]; ok && visited {
				return true
			} else {
				visitedNodes[first] = true
			}

			first = first.Next
		}

		if second != nil {
			if visited, ok := visitedNodes[second]; ok && visited {
				return true
			} else {
				visitedNodes[second] = true
			}

			second = second.Next
		}
	}

	return false
}

func main() {
	var first = &Node{Val: 1}
	first.Next = &Node{Val: 2}
	first.Next.Next = &Node{Val: 3}
	first.Next.Next.Next = &Node{Val: 4}

	var second = &Node{Val: 10}
	second.Next = &Node{Val: 20}
	second.Next.Next = &Node{Val: 30}
	second.Next.Next.Next = first.Next.Next.Next

	println("doesLinkedListsIntersect(first, second)", doesLinkedListsIntersect(first, second))
}
