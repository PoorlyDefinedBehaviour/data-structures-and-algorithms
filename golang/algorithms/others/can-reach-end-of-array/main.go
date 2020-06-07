package main

/**
*	time O(n)
* space O(1)
**/
func canReachEndOfArray(array []int) bool {
	var lastVisitedIndex = len(array) - 1

	for i := lastVisitedIndex; i > -1; i-- {
		if i+array[i] >= lastVisitedIndex {
			lastVisitedIndex = i
		}
	}

	return lastVisitedIndex == 0
}

func main() {
	/*
	   Given an array of non-negative integers, you are initially positioned at the first index of the array.

	   Each element in the array represents your maximum jump length at that position.

	   Determine if you are able to reach the last index.

	   Example 1:

	   Input: [2,3,1,1,4]
	   Output: true
	   Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

	   Example 2:

	   Input: [3,2,1,0,4]
	   Output: false
	   Explanation: You will always arrive at index 3 no matter what. Its maximum
	                jump length is 0, which makes it impossible to reach the last index.
	*/
	println("[2,3,1,1,4]", canReachEndOfArray([]int{2, 3, 1, 1, 4}))
	println("[3,2,1,0,4]", canReachEndOfArray([]int{3, 2, 1, 0, 4}))
}
