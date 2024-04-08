package main

// Given an integer array nums, return true if
// you can partition the array into two subsets such that the sum of the elements in both subsets is equal or false otherwise.
//
// Example 1:
//
// Input: nums = [1,5,11,5]
// Output: true
// Explanation: The array can be partitioned as [1, 5, 5] and [11].
// Example 2:
//
// Input: nums = [1,2,3,5]
// Output: false
// Explanation: The array cannot be partitioned into equal sum subsets.
//
// Constraints:
//
// 1 <= nums.length <= 200
// 1 <= nums[i] <= 100

import (
	"maps"
)

// time O(n)
// space O(1)
func sumSlice(nums []int) int {
	sum := 0
	for _, num := range nums {
		sum += num
	}
	return sum
}

func canPartition(nums []int) bool {
	// O(n)
	sum := sumSlice(nums)
	// If the sum is odd it is impossible to divide it in 2.
	if sum&1 == 1 {
		return false
	}

	half := sum / 2

	seen := map[int]struct{}{
		0: {},
	}

	// O(n)
	for _, num := range nums {
		newSeen := maps.Clone(seen)
		// O(sum(nums))
		for otherNum := range seen {
			total := num + otherNum
			if total == half {
				return true
			}
			if total < half {
				newSeen[total] = struct{}{}
			}

		}
		seen = newSeen
	}

	return false
}

func main() {

}
