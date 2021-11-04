package main

/*
Find All Anagrams in a String

Given two strings s and p, return an array of all the start indices of p's anagrams in s.
You may return the answer in any order.

An Anagram is a word or phrase formed by rearranging the letters of a different word or phrase,
typically using all the original letters exactly once.

Example 1:

Input: s = "cbaebabacd", p = "abc"
Output: [0,6]
Explanation:
The substring with start index = 0 is "cba", which is an anagram of "abc".
The substring with start index = 6 is "bac", which is an anagram of "abc".

Example 2:

Input: s = "abab", p = "ab"
Output: [0,1,2]
Explanation:
The substring with start index = 0 is "ab", which is an anagram of "ab".
The substring with start index = 1 is "ba", which is an anagram of "ab".
The substring with start index = 2 is "ab", which is an anagram of "ab"

Constraints:

1 <= s.length, p.length <= 3 * 104
s and p consist of lowercase English letters.
*/

func mapEquals(a, b map[rune]int) bool {
	if len(a) != len(b) {
		return false
	}

	for key := range a {
		if a[key] != b[key] {
			return false
		}
	}

	return true
}

// time O(n * m)
//   where n is len(s)
//   and m is len(p)
//
// space O(1) because we only have letters from a-z

func findAnagrams2(s string, p string) []int {
	anagramsStartingIndeces := make([]int, 0)

	pcount := make(map[rune]int)

	for _, character := range p {
		pcount[character]++
	}

	for i := 0; i < len(s)-len(p)+1; i++ {
		scount := make(map[rune]int)

		for j := range p {
			scount[rune(s[i+j])]++
		}

		if mapEquals(scount, pcount) {
			anagramsStartingIndeces = append(anagramsStartingIndeces, i)
		}
	}

	return anagramsStartingIndeces
}

// This is sliding window problem
//
// time O(n) where n is len(s)
// space O(1) because we only have letters from a-z
func findAnagrams(s string, p string) []int {
	pcount := make(map[rune]int)
	scount := make(map[rune]int)

	for _, character := range p {
		pcount[character]++
		scount[character]++
	}

	anagramsStartingIndeces := make([]int, 0)

	left := 0
	right := len(p)

	for {
		if mapEquals(scount, pcount) {
			anagramsStartingIndeces = append(anagramsStartingIndeces, left)
		}

		if right == len(s) {
			return anagramsStartingIndeces
		}

		scount[rune(s[right])]++

		scount[rune(s[left])]--

		if scount[rune(s[left])] == 0 {
			delete(scount, rune(s[left]))
		}

		left++
		right++
	}
}

func main() {
}
