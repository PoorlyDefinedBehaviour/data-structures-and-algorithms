/*
Given an unsorted array of integers, find the length of the longest consecutive elements sequence.

Your algorithm should run in O(n) complexity.

Example:

Input: [100, 4, 200, 1, 3, 2]
Output: 4
Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.
*/

/*
  time O(2n) -> O(n)
  space O(n)
*/
const longestConsecutive = (numbers) => {
  const numbersMap = numbers.reduce(
    (map, number) => ({ ...map, [number]: number }),
    {}
  );

  const result = numbers.reduce(
    (state, number) => {
      if (number + 1 in numbersMap) {
        return {
          ...state,
          longest: state.longest + 1,
        };
      }
      return { ...state, longest: state.current, current: 1 };
    },
    { longest: 0, current: 0 }
  );

  return result.longest;
};

// eslint-disable-next-line no-console
console.log(
  "longestConsecutive([100, 4, 200, 1, 3, 2])",
  longestConsecutive([100, 4, 200, 1, 3, 2])
);
