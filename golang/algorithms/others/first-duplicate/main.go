package main

/**
* time O(n)
* space O(n)
 */
func firstDuplicateHashMap(numbers []int) int {
	var visitedNumbers = make(map[int]bool)

	for _, number := range numbers {
		if ok, _ := visitedNumbers[number]; ok {
			return number
		}

		visitedNumbers[number] = true
	}

	return -1
}

func abs(x int) int {
	if x < 0 {
		return -x
	}

	return x
}

/**
* time O(n)
* space O(1)
**/
func firstDuplicate(numbers []int) int {
	for _, number := range numbers {
		var index = abs(number) - 1

		if numbers[index] < 0 {
			return abs(number)
		}

		numbers[index] = -numbers[index]
	}

	return -1
}

func main() {
	println(firstDuplicateHashMap([]int{2, 3, 5, 3, 2}))
	println(firstDuplicate([]int{2, 3, 5, 3, 2}))
}
