package main

/*
Example 1:

Input: n = 2
Output: [0,1,1]
Explanation:
0 --> 0
1 --> 1
2 --> 10

Example 2:

Input: n = 5
Output: [0,1,1,2,1,2]
Explanation:
0 --> 0
1 --> 1
2 --> 10
3 --> 11
4 --> 100
5 --> 101

Constraints:

    0 <= n <= 105
*/
func countBits(n int) []int {
	dp := make([]int, n+1)

	for i := 0; i <= n; i++ {
		dp[i] = dp[i>>1] + (i & 1)
	}

	return dp
}

func main() {

}
