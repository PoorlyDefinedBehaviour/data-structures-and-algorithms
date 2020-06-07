/*
Given a sorted integer array that does not contain any
duplicates, return a summary of the number ranges it
contains.

Example

For nums = [-1, 0, 1, 2, 6, 7, 9], the output should be
composeRanges(nums) = ["-1->2", "6->7", "9"]
*/

/**
 * time O(n)
 * space O(n) or O(1) if return value is not considered
 */
const composeRanges = (numbers: number[]): string[] => {
  const ranges: string[] = [];

  let start = 0;

  for (let i = 0; i < numbers.length; i += 1) {
    if (numbers[i] + 1 !== numbers[i + 1]) {
      ranges.push(
        i !== start ? `${numbers[start]}->${numbers[i]}` : String(numbers[i])
      );
      start = i + 1;
    }
  }

  return ranges;
};

console.log(
  "composeRanges([-1, 0, 1, 2, 6, 7, 9])",
  composeRanges([-1, 0, 1, 2, 6, 7, 9])
);
