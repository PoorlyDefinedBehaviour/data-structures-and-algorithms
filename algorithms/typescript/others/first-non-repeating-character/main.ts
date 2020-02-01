/**
 * time O(2n) -> O(n)
 * space O(n)
 */
function firstNonRepeatingCharacter(characters: string): string {
  const characterCountTable = new Map<string, number>()

  for (const character of characters) {
    const count = characterCountTable.get(character)
    characterCountTable.set(character, count ? count + 1 : 1)
  }

  for (const [character, count] of characterCountTable.entries()) {
    if (count == 1) {
      return character
    }
  }

  return "$"
}
function main() {
  console.log(firstNonRepeatingCharacter("abbcdcdefg"))
}
main()
