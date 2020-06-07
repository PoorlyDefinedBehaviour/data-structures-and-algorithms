package linkedlist

import "errors"

type Predicate func(value interface{}) bool
type Mapper func(value interface{}) interface{}
type Reducer func(accumulator interface{}, value interface{}) interface{}
type Consumer func(value interface{})

type Node struct {
	next  *Node
	Value interface{}
}

type LinkedList struct {
	head   *Node
	tail   *Node
	length int
}

func New() *LinkedList {
	var list = &LinkedList{}
	list.length = 0
	return list
}

func (list *LinkedList) Length() int {
	return list.length
}

func (list *LinkedList) IsEmpty() bool {
	return list.Length() == 0
}

func (list *LinkedList) Head() *Node {
	return list.head
}

func (list *LinkedList) Tail() *Node {
	return list.tail
}

func (list *LinkedList) assertIndexIsValid(index int) bool {
	return index >= 0 && index <= list.Length()
}

func (list *LinkedList) At(index int) interface{} {
	if !list.assertIndexIsValid(index) {
		return errors.New("Invalid index")
	}

	var i = 0
	var currentNode = list.Head()

	for i != index {
		i++
		currentNode = currentNode.next
	}

	return currentNode.Value
}

func (list *LinkedList) Push(value interface{}) *LinkedList {
	if list.IsEmpty() {
		list.head = &Node{Value: value}
		list.tail = list.head
	} else {
		list.tail.next = &Node{Value: value}
		list.tail = list.tail.next
	}

	list.length++
	return list
}

func (list *LinkedList) FillRange(start int, end int) *LinkedList {
	for start < end {
		list.Push(start)
		start++
	}

	return list
}

func (list *LinkedList) RemoveOne(predicate Predicate) *LinkedList {
	var previousNode *Node = nil
	var currentNode *Node = list.Head()

	for currentNode != nil {
		if predicate(currentNode.Value) {
			list.length--

			if previousNode == nil {
				list.head = currentNode.next
			} else {
				previousNode.next = currentNode.next
			}

			if currentNode.next == nil {
				list.tail = currentNode
			}

			return list
		}

		previousNode = currentNode
		currentNode = currentNode.next
	}

	return list
}

func (list *LinkedList) RemoveAll(predicate Predicate) *LinkedList {
	var previousNode *Node = nil
	var currentNode *Node = list.Head()

	for currentNode != nil {
		if predicate(currentNode.Value) {
			list.length--

			if previousNode == nil {
				list.head = currentNode.next
			} else {
				previousNode.next = currentNode.next
			}

			if currentNode.next == nil {
				list.tail = currentNode
			}

		}

		previousNode = currentNode
		currentNode = currentNode.next
	}

	return list
}

func (list *LinkedList) Includes(predicate Predicate) bool {
	var currentNode *Node = list.Head()

	for currentNode != nil {
		if predicate(currentNode.Value) {
			return true
		}
		currentNode = currentNode.next
	}

	return false
}

func (list *LinkedList) All(predicate Predicate) bool {
	var currentNode *Node = list.Head()

	for currentNode != nil {
		if !predicate(currentNode.Value) {
			return false
		}
		currentNode = currentNode.next
	}

	return true
}

func (list *LinkedList) Any(predicate Predicate) bool {
	var currentNode *Node = list.Head()

	for currentNode != nil {
		if predicate(currentNode.Value) {
			return true
		}
		currentNode = currentNode.next
	}

	return false
}

func (list *LinkedList) None(predicate Predicate) bool {
	return !list.All(predicate)
}

func (list *LinkedList) Map(mapper Mapper) *LinkedList {
	var newList = &LinkedList{}

	var currentNode *Node = list.Head()

	for currentNode != nil {
		newList.Push(mapper(currentNode.Value))
		currentNode = currentNode.next
	}

	return newList
}

func (list *LinkedList) Filter(predicate Predicate) *LinkedList {
	var newList = &LinkedList{}

	var currentNode *Node = list.Head()

	for currentNode != nil {
		if predicate(currentNode.Value) {
			newList.Push(currentNode.Value)
		}
		currentNode = currentNode.next
	}

	return newList
}

func (list *LinkedList) Reduce(initialValue interface{}, reducer Reducer) interface{} {
	var accumulator interface{} = initialValue
	var currentNode *Node = list.Head()

	for currentNode != nil {
		accumulator = reducer(accumulator, currentNode.Value)
		currentNode = currentNode.next
	}

	return accumulator
}

func (list *LinkedList) ForEach(consumer Consumer) *LinkedList {
	var currentNode *Node = list.Head()

	for currentNode != nil {
		consumer(currentNode.Value)
		currentNode = currentNode.next
	}

	return list
}
