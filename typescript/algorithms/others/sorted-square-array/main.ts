function square(x: number): number {
  return x * x
}

/**
 * time O(n)
 * space O(n)
 */
function sortedSquaredArray(array: number[]): number[] {
  const result = new Array<number>(array.length)

  let left = 0
  let right = array.length - 1
  let i = array.length - 1

  while (i > -1) {
    if (Math.abs(array[left]) > array[right]) {
      result[i--] = square(array[left++])
    } else {
      result[i--] = square(array[right--])
    }
  }

  return result
}

function main(): void {
  /**
   * Given an array of sorted numbers between -10k & 10k,
   * return a sorted array with every number squared
   * [-6, -4, 1, 2, 3, 5] => [1, 4, 9, 16, 25, 36]
   */
  console.log(sortedSquaredArray([-6, -4, 1, 2, 3, 5]))
}
main()
