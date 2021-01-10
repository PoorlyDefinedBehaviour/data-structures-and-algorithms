package main

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func printLinkedList(head *node) {
	currentNode := head

	for currentNode != nil {
		print(currentNode.val, " ")
		currentNode = currentNode.next
	}
	print("\n")
}

func TestDeleteDuplicatesExampleOne(t *testing.T) {
	input := &node{val: 1, next: nil}
	input.next = &node{val: 2, next: nil}
	input.next.next = &node{val: 3, next: nil}
	input.next.next.next = &node{val: 3, next: nil}
	input.next.next.next.next = &node{val: 4, next: nil}
	input.next.next.next.next.next = &node{val: 4, next: nil}
	input.next.next.next.next.next.next = &node{val: 5, next: nil}

	expected := &node{val: 1, next: nil}
	expected.next = &node{val: 2, next: nil}
	expected.next.next = &node{val: 5, next: nil}

	actual := DeleteDuplicates(input)

	assert.Equal(t, actual, expected)
}

func TestDeleteDuplicatesExampleTwo(t *testing.T) {
	input := &node{val: 1, next: nil}
	input.next = &node{val: 1, next: nil}
	input.next.next = &node{val: 1, next: nil}
	input.next.next.next = &node{val: 2, next: nil}
	input.next.next.next.next = &node{val: 3, next: nil}

	expected := &node{val: 2, next: nil}
	expected.next = &node{val: 3, next: nil}

	actual := DeleteDuplicates(input)

	printLinkedList(actual)

	assert.Equal(t, actual, expected)
}
