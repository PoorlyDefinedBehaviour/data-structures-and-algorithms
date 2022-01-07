package main

import (
	"math"
)

// time O(n) where n is len(s) because we loop through the string to count
// how many times each move appears.
// space O(1) because there are only 4 characters: U, D, L, and R.
func getMaxDeletions(s string) int32 {
	instructionCount := make(map[string]int32, 0)

	// Count how many times each move appears.
	for _, instruction := range s {
		instructionCount[string(instruction)] += 1
	}

	// Given the input URDR:
	// After counting the moves we would have this map:
	// {
	// 	"D": 1,
	// 	"R": 2,
	// 	"U": 1
	// }
	//	abs(U - D) = abs(1 - 1) = 0
	// abs(L - R) = abs(0 - 2) = 2
	// len(s) - abs(U - D) + abs(L - R) = 4 - 2 = 2
	//
	// Note that the type casting is necessary because Go functions are not generic.
	movesDifference := math.Abs(float64(instructionCount["U"]-instructionCount["D"])) +
		math.Abs(float64(instructionCount["L"]-instructionCount["R"]))

	return int32(len(s) - int(movesDifference))
}

func main() {

}
