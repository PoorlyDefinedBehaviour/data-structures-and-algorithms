/**
 * time O(n)
 * space O(1)
 */
function maxSubArray(numbers: number[]): number {
  let [sum] = numbers
  let [currentSum] = numbers

  for (let i = 1; i < numbers.length; ++i) {
    currentSum = Math.max(currentSum + numbers[i], numbers[i])
    sum = Math.max(sum, currentSum)
  }

  return sum
}

function main() {
  /*
Given an integer array nums,
find the contiguous subarray (containing at least one number) 
which has the largest sum and return its sum.

Example:

Input: [-2,1,-3,4,-1,2,1,-5,4],
Output: 6
Explanation: [4,-1,2,1] has the largest sum = 6.
*/
  console.log(
    "[-2,1,-3,4,-1,2,1,-5,4]",
    maxSubArray([-2, 1, -3, 4, -1, 2, 1, -5, 4])
  )
}
main()
