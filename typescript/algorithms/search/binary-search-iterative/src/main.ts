import * as R from "ramda"
import * as Option from "fp-ts/lib/Option"

/**
 * time O(log n)
 * space O(1)
 */
const binarySearch = R.curry((target: number, xs: number[]) => {
  let start = 0
  let end = xs.length - 1

  while (start <= end) {
    const middle = Math.floor((start + end) / 2)

    if (target === xs[middle]) {
      return Option.some(middle)
    }

    if (target < xs[middle]) {
      end = middle - 1
    }

    if (target > xs[middle]) {
      start = middle + 1
    }
  }

  return Option.none
})

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(11, [-2, 3, 4, 7, 8, 9, 11, 13]))

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(11, [13, -2, 3, 4, 7, 8, 9, 11]))

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(11, [8, 9, 11, 13, -2, 3, 4, 7]))

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(3, [1, 2, 3, 4, 5]))

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(3, [5, 4, 3, 2, 1]))

Option.fold(
  () => console.log("target not found"),
  index => console.log(`target found at ${index}`)
)(binarySearch(333, [5, 4, 3, 2, 1]))
