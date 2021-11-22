use std::collections::HashMap;

// 169. Majority Element - https://leetcode.com/problems/majority-element/
//
// Given an array nums of size n, return the majority element.
//
// The majority element is the element that appears more than ⌊n / 2⌋ times.
//You may assume that the majority element always exists in the array.
//
// Example 1:
//
// Input: nums = [3,2,3]
// Output: 3
//
// Example 2:
//
// Input: nums = [2,2,1,1,1,2,2]
// Output: 2
//
// Constraints:
//
//     n == nums.length
//     1 <= n <= 5 * 104
//     -231 <= nums[i] <= 231 - 1
//
// Follow-up: Could you solve the problem in linear time and in O(1) space?

// time O(n) where n is the length of [nums]
// space O(n) where n is the length of [nums]
// because we if every number in [nums] was unique we would add every single one of then
// to [ocurrences].
fn majority_element_1(nums: Vec<i32>) -> i32 {
    let mut ocurrences = HashMap::new();

    for num in nums {
        let count = ocurrences.entry(num).or_insert(0);
        *count += 1;
    }

    let (num, _count) = ocurrences.iter().max().unwrap();

    *num
}

// time O(n) where n is the length of [nums]
// space O(n) where n is the length of [nums]
// because we if every number in [nums] was unique we would add every single one of then
// to [ocurrences].
fn majority_element_2(nums: Vec<i32>) -> i32 {
    let mut ocurrences = HashMap::new();

    let mut result = nums[0];
    let mut max = 1;

    for num in nums {
        let count = ocurrences.entry(num).or_insert(0);
        *count += 1;

        if *count > max {
            result = num;
            max = *count;
        }
    }

    result
}

// time O(n) where n is the length of [nums]
// space O(1)
//
// This is only possible because the majority element appears more than ⌊n / 2⌋ times.
fn majority_element_3(nums: Vec<i32>) -> i32 {
    let mut current_majority_num = nums[0];
    let mut count = 0;

    for num in nums {
        if num == current_majority_num {
            count += 1;
        } else {
            count -= 1;
        }

        if count <= 0 {
            current_majority_num = num;
            count = 1;
        }
    }

    current_majority_num
}

fn main() {
    dbg!(majority_element_1(vec![3, 2, 3]));
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check(f: fn(Vec<i32>) -> i32) {
        let tests = vec![(vec![3, 2, 3], 3), (vec![2, 2, 1, 1, 1, 2, 2], 2)];

        for (input, expected) in tests {
            assert_eq!(expected, f(input));
        }
    }

    #[test]
    fn test_majority_element_1() {
        check(majority_element_1);
    }

    #[test]
    fn test_majority_element_2() {
        check(majority_element_2);
    }

    #[test]
    fn test_majority_element_3() {
        check(majority_element_3);
    }
}
