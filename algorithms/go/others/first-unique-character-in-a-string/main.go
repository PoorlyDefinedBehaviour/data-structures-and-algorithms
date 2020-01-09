package main

type Pair struct {
	Index     int
	Frequency int
}

/**
* time O(2n) -> O(n)
* space O(1)
**/
func firstUniqueCharacter(text string) int {
	const alphabetLength = 26
	var characterFrequencyTable = make(map[rune]*Pair, alphabetLength)

	for index, character := range text {
		if _, ok := characterFrequencyTable[character]; !ok {
			characterFrequencyTable[character] = &Pair{Index: index, Frequency: 0}
		}

		var pair, _ = characterFrequencyTable[character]
		pair.Frequency++
	}

	for _, character := range text {
		var pair, _ = characterFrequencyTable[character]
		if pair.Frequency == 1 {
			return pair.Index
		}
	}

	return -1

}

func main() {
	/*
	  Given a string, find the first non-repeating character in it and return it's index. If it doesn't exist, return -1.

	   Examples:

	   s = "leetcode"
	   return 0.

	   s = "loveleetcode",
	   return 2.

	   Note: You may assume the string contain only lowercase letters.
	*/
	println("leetcode", firstUniqueCharacter("leetcode"))
	println("loveleetcode", firstUniqueCharacter("loveleetcode"))
}
