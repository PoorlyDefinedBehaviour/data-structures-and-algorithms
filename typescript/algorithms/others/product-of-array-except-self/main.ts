/**
 * time O(n)
 * space O(1)
 */
function productOfArrayExceptSelf(numbers: number[]): number[] {
  const result = [1]

  for (let i = 1; i < numbers.length; ++i) {
    result.push(numbers[i - 1] * result[i - 1])
  }

  let r = 1
  for (let i = numbers.length - 1; i > -1; --i) {
    result[i] = result[i] * r
    r = r * numbers[i]
  }

  return result
}

function main() {
  /*
  Given an array nums of n integers where n > 1,  return an array output such that output[i] is equal to the product of all the elements of nums except nums[i].

  Example:

  Input:  [1,2,3,4]
  Output: [24,12,8,6]

  Note: Please solve it without division and in O(n).
  */
  console.log(productOfArrayExceptSelf([1, 2, 3, 4]))
}
main()
