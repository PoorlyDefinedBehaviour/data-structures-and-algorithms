package main

/*
https://leetcode.com/problems/monotonic-array/description/

An array is monotonic if it is either monotone increasing or monotone decreasing.

An array nums is monotone increasing if for all i <= j, nums[i] <= nums[j].
An array nums is monotone decreasing if for all i <= j, nums[i] >= nums[j].

Return true if and only if the given array nums is monotonic.

Example 1:

Input: nums = [1,2,2,3]
Output: true

Example 2:

Input: nums = [6,5,4,4]
Output: true

Example 3:

Input: nums = [1,3,2]
Output: false

Example 4:

Input: nums = [1,2,4,5]
Output: true

Example 5:

Input: nums = [1,1,1]
Output: true

Note:

    1 <= nums.length <= 50000
    -100000 <= nums[i] <= 100000
*/

// time O(n)
// space O(1)
func isMonotonic(numbers []int) bool {
	isIncreasing := true
	isDecreasing := true

	for i := 0; i < len(numbers)-1; i++ {
		if numbers[i] > numbers[i+1] {
			isIncreasing = false
		}

		if numbers[i] < numbers[i+1] {
			isDecreasing = false
		}

		if !isIncreasing && !isDecreasing {
			return false
		}
	}

	return true
}

func main() {
}
