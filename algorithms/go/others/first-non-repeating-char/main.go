package main

/**
* time O(2n) -> O(n)
* space O(n)
**/
func firstNonDuplicateCharacter(characters string) rune {
	var characterCountTable = make(map[rune]int)

	for _, character := range characters {
		if count, ok := characterCountTable[character]; ok {
			characterCountTable[character] = count + 1
		} else {
			characterCountTable[character] = 1
		}
	}

	for character, count := range characterCountTable {
		if count == 1 {
			return character
		}
	}

	return '$'
}

func main() {
	println(string(firstNonDuplicateCharacter("abacccccddddddbfgf")))
}
