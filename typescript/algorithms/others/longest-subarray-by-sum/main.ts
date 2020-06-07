type Pair<T> = [T, T]

/**
 * time O(n)
 * space O(1)
 */
function findLongestSubarrayBySum(
  numbers: number[],
  target: number
): Pair<number> {
  const result: Pair<number> = [-1, -1]

  let sum = 0
  let left = 0

  for (let i = 0; i < numbers.length; ++i) {
    sum += numbers[i]

    while (left < i && sum > target) {
      sum -= numbers[left++]
    }

    if (sum === target && i - left > result[1] - result[0]) {
      result[0] = left
      result[1] = i
    }
  }

  return result
}

function main() {
  console.log(
    findLongestSubarrayBySum([1, 2, 3, 4, 5, 0, 0, 0, 6, 7, 8, 9, 10], 15)
  )
}
main()
