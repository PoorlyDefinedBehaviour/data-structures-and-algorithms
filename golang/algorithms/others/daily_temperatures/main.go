package main

/*
https://leetcode.com/problems/daily-temperatures/

Given an array of integers temperatures represents the daily temperatures,
return an array answer such that answer[i] is the number of days you have to wait after the ith day to get a warmer temperature.
If there is no future day for which this is possible, keep answer[i] == 0 instead.

Example 1:

Input: temperatures = [73,74,75,71,69,72,76,73]
Output: [1,1,4,2,1,1,0,0]

Example 2:

Input: temperatures = [30,40,50,60]
Output: [1,1,1,0]

Example 3:

Input: temperatures = [30,60,90]
Output: [1,1,0]

Constraints:

    1 <= temperatures.length <= 105
    30 <= temperatures[i] <= 100
*/
// time O(n^2)
// space O(1) -- ignoring return value because it is required.
func dailyTemperaturesSlow(temperatures []int) []int {
	result := make([]int, len(temperatures))

	for i, temperature := range temperatures {
		for j := i + 1; j < len(temperatures); j++ {
			if temperatures[j] > temperature {
				result[i] = j - i
				break
			}
		}
	}

	return result
}

type pair struct {
	temperature int
	index       int
}

// Solution:
// Go through every temperature and for each,
// remove all temperatures that are less than the current temperature from the stack
// and the current temperature and its index to the stack.
// By doing this, the stack will always be in decreasing order.
// Whenever a temperature is removed from the stack, compare indices to find out
// how many days it took to find a temperature thats greater than the temperature
// that has just been removed from the stack.
//
// time O(n)
// space O(n)
func dailyTemperatures(temperatures []int) []int {
	result := make([]int, len(temperatures))

	monotonicStack := make([]pair, 0, len(temperatures))

	for index, temperature := range temperatures {
		for len(monotonicStack) > 0 && temperature > monotonicStack[len(monotonicStack)-1].temperature {
			pair := monotonicStack[len(monotonicStack)-1]
			monotonicStack = monotonicStack[:len(monotonicStack)-1]

			result[pair.index] = index - pair.index
		}

		monotonicStack = append(monotonicStack, pair{temperature: temperature, index: index})
	}

	return result
}

func main() {
}
