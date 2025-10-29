package smallestnumberwithallsetbits
package main

import "fmt"

// time O(1)
// space O(1)
func smallestNumber(n int) int {
	x := 1

	for x < n {
		x = x<<1 | 1
	}

	return x
}

func main() {
	assertEq(7, smallestNumber(5))
	assertEq(15, smallestNumber(10))
	assertEq(3, smallestNumber(3))
	assertEq(1, smallestNumber(1))
}

func assertEq(x, y int) {
	if x != y {
		panic(fmt.Sprintf("expected %+v to equal %+v", x, y))
	}
}
