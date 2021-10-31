package main

import "sort"

// Given an integer array nums,
// return true if any value appears at least
// twice in the array, and return false if
// every element is distinct.
// Example 1:
//
// Input: nums = [1,2,3,1]
// Output: true
//
// Example 2:
//
// Input: nums = [1,2,3,4]
// Output: false
//
// Example 3:
//
// Input: nums = [1,1,1,3,3,4,3,2,4,2]
// Output: true

// time O(n^2)
// space O(1)
func containsDuplicate1(nums []int) bool {
	for i := range nums {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] == nums[j] {
				return true
			}
		}
	}

	return false
}

// time O(n log n)
// space O(1)
func containsDuplicate2(nums []int) bool {
	sort.Slice(nums, func(i, j int) bool { return nums[i] < nums[j] })

	for i := 0; i < len(nums)-1; i++ {
		if nums[i] == nums[i+1] {
			return true
		}
	}

	return false
}

// time O(n)
// space O(n)
func containsDuplicate3(nums []int) bool {
	seen := make(map[int]bool)

	for _, num := range nums {
		if seen[num] {
			return true
		}

		seen[num] = true
	}

	return false
}

func main() {
}
