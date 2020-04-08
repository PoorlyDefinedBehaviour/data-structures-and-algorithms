import assert from "assert";
/*
Given an array of integers nums and an integer k,
determine wether there are two distinct indices i and j
in the array where nums[i] = nums[j] and the absolute
difference between i and j is less than or equal to k

Example

For nums = [0, 1, 2, 3, 5, 2] and k = 3, the 
output should be
containsCloseNums(nums, k) = true
*/

/**
 * time O(n)
 * space O(n)
 */
const containsCloseNums = (nums: number[], k: number): boolean => {
  const cache: { [key: number]: number } = {};

  for (let i = 0; i < nums.length; i += 1) {
    const number = nums[i];
    const seenValue = cache[number];

    if (seenValue && i - seenValue <= k) {
      return true;
    }

    cache[number] = i;
  }

  return false;
};

assert(containsCloseNums([0, 1, 2, 3, 5, 2], 3));
