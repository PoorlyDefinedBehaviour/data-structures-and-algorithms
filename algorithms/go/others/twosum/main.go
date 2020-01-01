package main

import "fmt"

/**
*	time O(n)
* space O(n)
 */
func twoSum(numbers []int, target int) (int, int) {
	var cache = make(map[int]int)

	for index, number := range numbers {
		cache[number] = index
		var difference = target - number

		var differenceIndex, ok = cache[difference]
		if ok && difference != index {
			return differenceIndex, index
		}
	}

	return -1, -1
}

func main() {
	/*Given an array of integers, return indices of the two numbers such that they add up to a specific target.
	  You may assume that each input would have exactly one solution, and you may not use the same element twice.

	  Example:
	  Given nums = [2, 7, 11, 15], target = 9,
	  Because nums[0] + nums[1] = 2 + 7 = 9,
	  return [0, 1].
	*/
	var numbers = [...]int{2, 7, 11, 15}
	fmt.Println(twoSum(numbers[:], 9))
}
