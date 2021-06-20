package main

/*
Given an array of integers nums containing n + 1 integers where each integer is in the range [1, n] inclusive.

There is only one repeated number in nums, return this repeated number.

You must solve the problem without modifying the array nums and uses only constant extra space.

Example 1:

Input: nums = [1,3,4,2,2]
Output: 2

Example 2:

Input: nums = [3,1,3,4,2]
Output: 3

Example 3:

Input: nums = [1,1]
Output: 1

Example 4:

Input: nums = [1,1,2]
Output: 1

Constraints:

    1 <= n <= 105
    nums.length == n + 1
    1 <= nums[i] <= n
    All the integers in nums appear only once except for precisely one integer which appears two or more times.

Follow up:

    How can we prove that at least one duplicate number must exist in nums?
    Can you solve the problem in linear runtime complexity?
*/

// time O(n)
// space O(n)
func findDuplicateUsingSet(numbers []int) int {
	seen := make(map[int]bool)

	for _, number := range numbers {
		if seen[number] {
			return number
		}

		seen[number] = true
	}

	panic("a duplicate should always exist")
}

// Solution:
// If we treat the list of numbers as a linked list
// where each number points to a node at position n
// we will see that a cycle always exists.
// To solve the problem we find a point that's part of the cycle
// and then the node where the cycle begins.
// The node where the cycle begins, is the duplicated number.
//
// time O(n)
// space O(1)
func findDuplicateUsingFloydsAlgorithm(numbers []int) int {
	tortoise := 0
	hare := 0

	// move pointers until cycle is found
	for {
		tortoise = numbers[tortoise]
		hare = numbers[numbers[hare]]

		if tortoise == hare {
			break
		}
	}

	// find node where cycle begins
	tortoise2 := 0
	for {
		tortoise2 = numbers[tortoise2]
		number := numbers[tortoise]
		if number == tortoise2 {
			return number
		}
	}
}

func main() {
}
