package main

import "strings"

/*
	Given a list of product names and a letter,
	return the products that have the letter in their name.
*/

// time O(n * m) where n is the length of the list and m is the length of the product name
// space O(n).
func filter(products []string, letter rune) []string {
	out := make([]string, 0)

	for _, product := range products {
		if strings.ContainsRune(product, letter) {
			out = append(out, product)
		}
	}

	return out
}

func main() {
}
