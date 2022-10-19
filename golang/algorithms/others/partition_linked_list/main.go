// Partition: Write code to partition a linked list around a value x, such that all nodes less than x come
// before all nodes greater than or equal to x. If x is contained within the list, the values of x only need
// to be after the elements less than x (see below). The partition element x can appear anywhere in the
// "right partition"; it does not need to appear between the left and right partitions.
// EXAMPLE
// Input:
// Output:
// 3 -> 5 -> 8 -> 5 -> 10 -> 2 -> 1 [partition= 5]
// 3 -> 1 -> 2 -> 10 -> 5 -> 5 -> 8

package main

import (
	"fmt"
	"strings"
)

type Node struct {
	value int
	next  *Node
}

func (node *Node) String() string {
	buffer := strings.Builder{}

	current := node
	for current != nil {
		buffer.WriteString(fmt.Sprintf("%d", current.value))
		if current.next != nil {
			buffer.WriteString(" -> ")
		}

		current = current.next
	}

	return buffer.String()
}

func cons(value int, node *Node) *Node {
	return &Node{value: value, next: node}
}

/// time O(n)
/// space O(1)
func partition(list *Node, value int) *Node {
	if list == nil {
		return nil
	}

	head := list
	tail := list
	current := list

	for current != nil {
		if current.value > value {
			next := current.next

			tail.next = current
			tail = tail.next

			current = next
		} else {
			next := current.next

			current.next = head
			head = current

			current = next
		}
	}

	tail.next = nil

	return head
}

func main() {
	list := cons(5, cons(4, cons(3, cons(2, cons(1, nil)))))

	list = partition(list, 3)

	fmt.Println(list.String())
}
