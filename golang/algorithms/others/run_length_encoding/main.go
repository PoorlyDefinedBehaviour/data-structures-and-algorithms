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

func main() {

}
