package main

/**
* time O(n)
* space O(n)
**/
func containsDuplicate(nums []int) bool {
	var cache = make(map[int]bool)

	for _, number := range nums {
		if cache[number] {
			return true
		}
		cache[number] = true
	}

	return false
}

func main() {
	/*
	   Given an array of integers, find if the array contains any duplicates.

	   Your function should return true if any value appears at least twice in the array, and it should return false if every element is distinct.

	   Example 1:

	   Input: [1,2,3,1]
	   Output: true

	   Example 2:

	   Input: [1,2,3,4]
	   Output: false

	   Example 3:

	   Input: [1,1,1,3,3,4,3,2,4,2]
	   Output: true
	*/
	println("[1, 2, 3, 1]", containsDuplicate([]int{1, 2, 3, 1}))
	println("[1, 2, 3, 4,", containsDuplicate([]int{1, 2, 3, 4}))
	println("[1,1,1,3,3,4,3,2,4,2]", containsDuplicate([]int{1, 1, 1, 3, 3, 4, 3, 2, 4, 2}))
}
