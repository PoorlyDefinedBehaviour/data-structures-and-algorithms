/**
 * time O(n)
 * space O(n)
 */
function firstDuplicateHashMap(numbers: number[]): number {
  const visitedNumbers = {}

  for (const number of numbers) {
    if (number in visitedNumbers) {
      return number
    }

    visitedNumbers[number] = true
  }

  return -1
}

/**
 * time O(n)
 * space O(1)
 */
function firstDuplicate(numbers: number[]): number {
  for (const number of numbers) {
    const index = Math.abs(number) - 1

    if (numbers[index] < 0) {
      return Math.abs(number)
    }

    numbers[index] = -numbers[index]
  }

  return -1
}

function main() {
  console.log(firstDuplicateHashMap([2, 3, 5, 3, 2]))
  console.log(firstDuplicate([2, 3, 5, 3, 2]))
}
main()
