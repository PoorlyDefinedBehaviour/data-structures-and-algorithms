/**
 * time O(2n) -> O(n)
 * space O(2n) -> O(n)
 */
function reverseOnlyLetters(s: string): string {
  let left = 0
  let right = s.length - 1

  let newString = s.split("")

  const isLetter = (char: string) =>
    (char >= "a" && char <= "z") || (char >= "A" && char <= "Z")

  while (left < right) {
    if (!isLetter(newString[left])) {
      ++left
    }
    if (!isLetter(newString[right])) {
      --right
    }

    if (isLetter(newString[left]) && isLetter(newString[right])) {
      const temp = newString[left]
      newString[left++] = newString[right]
      newString[right--] = temp
    }
  }

  return newString.join("")
}

function main() {
  /*
  Given a string S, return the "reversed" string where all characters that are not a letter stay in the same place, 
  and all letters reverse their positions.

  Example 1:

  Input: "ab-cd"
  Output: "dc-ba"

  Example 2:

  Input: "a-bC-dEf-ghIj"
  Output: "j-Ih-gfE-dCba"

  Example 3:

  Input: "Test1ng-Leet=code-Q!"
  Output: "Qedo1ct-eeLg=ntse-T!"
  */
  console.log(reverseOnlyLetters("a-bC-dEf-ghIj"))
}
main()
