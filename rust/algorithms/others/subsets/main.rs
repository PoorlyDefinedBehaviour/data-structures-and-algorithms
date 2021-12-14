// 78. Subsets
//
// Given an integer array nums of unique elements,
// return all possible subsets (the power set).
//
// The solution set must not contain duplicate subsets.
// Return the solution in any order.
//
// Example 1:
//
// Input: nums = [1,2,3]
// Output: [[],[1],[2],[1,2],[3],[1,3],[2,3],[1,2,3]]
//
// Example 2:
//
// Input: nums = [0]
// Output: [[],[0]]
//
// Constraints:
//
//     1 <= nums.length <= 10
//     -10 <= nums[i] <= 10
//     All the numbers of nums are unique.

// time O(n * 2^n)
pub fn subsets(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut subsets = Vec::new();
    let mut subset = Vec::new();

    fn dfs(subsets: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, nums: &Vec<i32>, i: usize) {
        if i >= nums.len() {
            subsets.push(subset.clone());
            return;
        }

        subset.push(nums[i]);

        dfs(subsets, subset, nums, i + 1);

        subset.pop();

        dfs(subsets, subset, nums, i + 1);
    }

    dfs(&mut subsets, &mut subset, &nums, 0);

    subsets
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets() {
        let tests = vec![
            (
                vec![1, 2, 3],
                vec![
                    vec![],
                    vec![1],
                    vec![2],
                    vec![1, 2],
                    vec![3],
                    vec![1, 3],
                    vec![2, 3],
                    vec![1, 2, 3],
                ],
            ),
            (vec![0], vec![vec![], vec![0]]),
        ];

        for (input, mut expected) in tests {
            let mut actual = subsets(input);

            expected.sort();
            actual.sort();

            assert_eq!(expected, actual);
        }
    }
}
