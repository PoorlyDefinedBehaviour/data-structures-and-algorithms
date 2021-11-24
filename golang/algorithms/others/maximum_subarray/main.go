package main

import "fmt"

// 53. Maximum Subarray
//
// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//
// A subarray is a contiguous part of an array.
//
// Example 1:
//
// Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
// Output: 6
// Explanation: [4,-1,2,1] has the largest sum = 6.
//
// Example 2:
//
// Input: nums = [1]
// Output: 1
//
// Example 3:
//
// Input: nums = [5,4,-1,7,8]
// Output: 23
//
// Constraints:
//     1 <= nums.length <= 105
//     -104 <= nums[i] <= 104

// time O(n)
// space O(1)
//
// We keep track of the greatest sub array sum and the current sub array sum.
// When a find a sub array sum that's greater than the max sub array sum,
// the max sub array sum becomes the current one.
func maxSubArray(nums []int) int {
	maxSubArray := nums[0]
	currentSubArray := nums[0]

	for i := 1; i < len(nums); i++ {
		currentNum := nums[i]
		value := currentSubArray + currentNum

		if nums[i] > value {
			currentSubArray = nums[i]
		} else {
			currentSubArray = value
		}

		if maxSubArray < currentSubArray {
			maxSubArray = currentSubArray
		}
	}

	return maxSubArray
}

func main() {
	fmt.Println(maxSubArray([]int{-2, 1, -3, 4, -1, 2, 1, -5, 4}))
}
