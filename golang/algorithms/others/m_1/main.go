package main

import (
	"fmt"
	"math/bits"
	"strconv"
)

// time O(n^2)
// space O(1)
func Solution(cars []string) []int {
	result := make([]int, len(cars))

	for i, car := range cars {

		for j, otherCar := range cars {
			if i != j && hammingDistance(car, otherCar) <= 1 {
				result[i] += 1
			}
		}

	}

	return result
}

// Hamming distance is a way to calculte the difference in bits between two numbers.
func hammingDistance(carA string, carB string) int {
	a, _ := strconv.ParseUint(carA, 2, 64)

	b, _ := strconv.ParseUint(carB, 2, 64)

	return bits.OnesCount64(a ^ b)

}

func isEven(n int) bool {
	return n%2 != 0
}

func Solution2(cars []string) []int {
	featureCount := make(map[string]int)

	for _, car := range cars {
		featureCount[car] += 1
	}

	result := make([]int, len(cars))

	for i, car := range cars {
		result[i] = countSimilarCars(car, featureCount)
	}

	return result
}

func countSimilarCars(car string, featureCount map[string]int) int {
	// Start at -1 to take the car itself into account.
	count := -1
	count += featureCount[car]

	// Note that feature is at most 15 characters.
	for i := range car {
		feature := car[i]
		var newFeature string
		if feature == '0' {
			newFeature = "1"
		} else {
			newFeature = "0"
		}

		newCar := car[:i] + newFeature + car[i+1:]

		count += featureCount[newCar]
	}

	return count
}

func main() {
	// task 1
	// fmt.Println(Solution("acbcbba"))
	// fmt.Println(Solution("axxaxa"))
	// fmt.Println(Solution("aaaa"))

	// task 2
	// fmt.Println(Solution(14))
	// fmt.Println(Solution(10))
	// fmt.Println(Solution(9999))

	// task 3
	fmt.Println(Solution([]string{"01101", "01001"}))
	fmt.Println(Solution([]string{"01101", "11110"}))

	fmt.Println(Solution2([]string{"01101", "01001"}))
	fmt.Println(Solution2([]string{"01101", "11110"}))
}

// task 1

package solution

// you can also use imports, for example:
// import "fmt"
// import "os"

// you can write to stdout for debugging purposes, e.g.
// fmt.Println("this is a debug message")

// time O(n) because the amount of time needed to find the number whose
// sum of digits is equal to the target grows whenever the number grows.
// space O(1)
func Solution(n int) int {
    // The target is the sum of the digits in n times two.
	target := sumNumberDigits(n) * 2

	i := n + 1

    // Brute force every number that's greater than n until we find a
    // number whose sum of digits is equal to the target.
	for {
		if sumNumberDigits(i) == target {
			return i
		}

		i += 1
	}
}

func sumNumberDigits(n int) int {
	result := 0
	for n > 0 {
		result += n % 10
		n /= 10
	}
	return result
}

// task 3

package main

import "fmt"

// time O(n) where n is the length of the string
// space O(1) because we always use an array of length 26 no matter the length
// of the input.
func Solution(s string) int {
	const NUMBER_OF_ASCII_LETTERS = 26

	// Maps each letter in the alphabet to how many times it appears in the string.
	var characterCount [NUMBER_OF_ASCII_LETTERS]int

	// Count how many times each character appears in the string.
	for _, character := range s {
		// ascii letters can be represented using one byte.
		// subtract 'a' from each character so character 'a' to index 0, 'b' to 1 and so on.
		index := int(character - 'a')
		characterCount[index] += 1
	}

	deletions := 0

	// If a character appears an even number of times, we need to delete it at least once.
	for _, count := range characterCount {
		if isEven(count) {
			deletions += 1
		}
	}

	return deletions

}

func isEven(n int) bool {
	return n%2 != 0
}

func main() {
	fmt.Println(Solution("acbcbba"))
	fmt.Println(Solution("axxaxa"))
	fmt.Println(Solution("aaaa"))
}


