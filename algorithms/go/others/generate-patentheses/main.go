package main

func backtrack(combinations *[]string, current string, open int, close int, max int) {
	if len(current) == max*2 {
		*combinations = append(*combinations, current)
		return
	}

	if open < max {
		backtrack(combinations, current+"(", open+1, close, max)
	}
	if close < open {
		backtrack(combinations, current+")", open, close+1, max)
	}
}

func generateParenthesis(n int) []string {
	var combinations = []string{}
	backtrack(&combinations, "", 0, 0, n)
	return combinations
}

func main() {
	/*
	   Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.

	   For example, given n = 3, a solution set is:

	   [
	     "((()))",
	     "(()())",
	     "(())()",
	     "()(())",
	     "()()()"
	   ]
	*/
	for index, value := range generateParenthesis(3) {
		println(index, value)
	}
}
