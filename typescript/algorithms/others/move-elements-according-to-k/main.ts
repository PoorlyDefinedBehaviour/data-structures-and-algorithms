/**
 * time O(n)
 * space O(n)
 */
function moveElementsAccordingToK(numbers: number[], k: number): number[] {
  const bigger: number[] = []
  const equal: number[] = []
  const smaller: number[] = []

  for (const number of numbers) {
    if (number > k) {
      bigger.push(number)
    } else if (number < k) {
      smaller.push(number)
    } else {
      equal.push(number)
    }
  }

  return [...smaller, ...equal, ...bigger]
}

/**
 * time O(n)
 * space O(1)
 */
function _moveElementsAccordingToK(numbers: number[], k: number): void {
  let pivotIndex = 0

  for (let i = 0; i < numbers.length; ++i) {
    if (numbers[i] < k) {
      const temp = numbers[i]

      numbers[i] = numbers[pivotIndex]
      numbers[pivotIndex] = temp

      ++pivotIndex
    }
  }

  const temp = numbers[pivotIndex]
  numbers[pivotIndex] = numbers[numbers.length - 1]
  numbers[numbers.length - 1] = temp
}

const tap = (fn: Function) => (...xs: any[]) => (fn(...xs), xs)

function main() {
  /**
   * [2, 4, 3, 1, 5, 0] ~ k = 3
   * move elements that are less than k to k's left
   * move elements that are greater than k to k's right
   * [2, 1, 0, 3, 4, 5]
   */
  console.log(moveElementsAccordingToK([2, 4, 3, 1, 5, 0], 3))
  console.log(tap(_moveElementsAccordingToK)([2, 4, 3, 1, 5, 0], 3))
}
main()
