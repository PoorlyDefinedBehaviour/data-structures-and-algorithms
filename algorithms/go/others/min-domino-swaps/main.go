package main

import "math"

func min(numbers ...int) int {
	min := numbers[0]

	for _, i := range numbers {
		if min > i {
			min = i
		}
	}

	return min
}

func minSwaps(target int, a []int, b []int) int {
	var swaps = 0

	for i, _ := range a {
		if a[i] != target && b[i] != target {
			return math.MaxInt64
		}

		if a[i] != target && b[i] == target {
			swaps += 1
		}
	}

	return swaps
}

/**
time O(4) -> O(n)
space O(1)
*/
func minDominoRotations(a []int, b []int) int {
	var swaps = min(minSwaps(a[0], a, b), minSwaps(b[0], a, b), minSwaps(a[0], b, a), minSwaps(b[0], b, a))

	if swaps == math.MaxInt64 {
		return -1
	}

	return swaps
}

func main() {
	/*
			 In a row of dominoes, A[i] and B[i] represent the top and bottom halves of the i-th domino.
			  (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

		   We may rotate the i-th domino, so that A[i] and B[i] swap values.

			 Return the minimum number of rotations so that all the values in A are the same,
			 or all the values in B are the same.

		   If it cannot be done, return -1.

		   Example 1:

		   Input: A = [2,1,2,4,2,2], B = [5,2,6,2,3,2]
		   Output: 2

		   Example 2:

		   Input: A = [3,5,1,2,3], B = [3,6,3,3,4]
		   Output: -1
		   Explanation:
		   In this case, it is not possible to rotate the dominoes to make one row of values equal.

		   Note:

		       1 <= A[i], B[i] <= 6
		       2 <= A.length == B.length <= 20000
	*/

	println(minDominoRotations([]int{2, 1, 2, 4, 2, 2}, []int{5, 2, 6, 2, 3, 2}))
	println(minDominoRotations([]int{3, 5, 1, 2, 3}, []int{3, 6, 3, 3, 4}))
}
