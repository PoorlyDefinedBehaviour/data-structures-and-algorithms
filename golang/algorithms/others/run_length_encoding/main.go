package main

import (
	"fmt"
	"strings"
)

// time O(n)
// space O(n)
func runLengthEncoding(input string) string {
	stringBuilder := strings.Builder{}

	for i := 0; i < len(input); i++ {
		occurrences := 1

		currentCharacter := input[i]

		for i < len(input)-1 && currentCharacter == input[i+1] {
			occurrences++
			i++
		}

		stringBuilder.WriteString(fmt.Sprintf("%d%c", occurrences, currentCharacter))
	}

	return stringBuilder.String()
}

func countWhile(input string, predicate func(rune) bool) int {
	count := 0

	for _, character := range input {
		if !predicate(character) {
			return count
		}

		count++
	}

	return count
}

// time O(n)
// space O(2n)
func runLengthEncoding2(input string) string {
	stringBuilder := strings.Builder{}

	for i := 0; i < len(input); {
		currentCharacter := rune(input[i])

		count := countWhile(input[i:], func(c rune) bool { return c == currentCharacter })

		i += count

		stringBuilder.WriteString(fmt.Sprintf("%d%c", count, currentCharacter))
	}

	return stringBuilder.String()
}

func main() {

}
