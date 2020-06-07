package main

import (
	"container/heap"
)

type IntHeap []int

func (h IntHeap) Len() int           { return len(h) }
func (h IntHeap) Less(i, j int) bool { return h[i] > h[j] }
func (h IntHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }

func (h *IntHeap) Push(x interface{}) {
	*h = append(*h, x.(int))
}

func (h *IntHeap) Pop() interface{} {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

/**
* time O(n log n)
* space O(n)
**/
func lastStoneWeight(stones []int) int {
	var h = &IntHeap{2, 1, 5}
	heap.Init(h)

	for _, stone := range stones {
		heap.Push(h, stone)
	}

	for h.Len() > 1 {
		var x = h.Pop().(int)
		var y = h.Pop().(int)

		if x != y {
			heap.Push(h, y-x)
		}
	}

	return h.Pop().(int)
}

func main() {
	/*
	   We have a collection of rocks, each rock has a positive integer weight.

	   Each turn, we choose the two heaviest rocks and smash them together.  Suppose the stones have weights x and y with x <= y.  The result of this smash is:

	       If x == y, both stones are totally destroyed;
	       If x != y, the stone of weight x is totally destroyed, and the stone of weight y has new weight y-x.

	   At the end, there is at most 1 stone left.  Return the weight of this stone (or 0 if there are no stones left.)

	   Example 1:

	   Input: [2,7,4,1,8,1]
	   Output: 1
	   Explanation:
	   We combine 7 and 8 to get 1 so the array converts to [2,4,1,1,1] then,
	   we combine 2 and 4 to get 2 so the array converts to [2,1,1,1] then,
	   we combine 2 and 1 to get 1 so the array converts to [1,1,1] then,
	   we combine 1 and 1 to get 0 so the array converts to [1] then that's the value of last stone.
	*/
	println(lastStoneWeight([]int{2, 7, 4, 1, 8, 1}))
}
