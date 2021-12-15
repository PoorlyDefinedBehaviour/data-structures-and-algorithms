// 90. Subsets II
//
// Given an integer array nums that may contain duplicates, return all possible subsets (the power set).
//
// The solution set must not contain duplicate subsets. Return the solution in any order.
//
// Example 1:
//
// Input: nums = [1,2,2]
// Output: [[],[1],[1,2],[1,2,2],[2],[2,2]]
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

// time O(n * 2^n)
pub fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    fn dfs(subsets: &mut Vec<Vec<i32>>, subset: &mut Vec<i32>, nums: &Vec<i32>, mut i: usize) {
        if i >= nums.len() {
            subsets.push(subset.clone());
            return;
        }

        subset.push(nums[i]);

        dfs(subsets, subset, nums, i + 1);

        // Backtrack
        subset.pop();

        // Given nums = [1, 2, 2, 3] and i = 1
        // we just added 2 to the subset
        // so we want to skip the other 2.
        // This way we don't end with duplicated subsets.
        while i + 1 < nums.len() && nums[i] == nums[i + 1] {
            i += 1;
        }

        dfs(subsets, subset, nums, i + 1);
    }

    let mut subsets = Vec::new();
    let mut subset = Vec::new();

    nums.sort();

    dfs(&mut subsets, &mut subset, &nums, 0);

    subsets
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subsets_with_dup() {
        let tests = vec![
            (
                vec![1, 2, 2],
                vec![
                    vec![],
                    vec![1],
                    vec![1, 2],
                    vec![1, 2, 2],
                    vec![2],
                    vec![2, 2],
                ],
            ),
            (vec![0], vec![vec![], vec![0]]),
        ];

        for (input, mut expected) in tests {
            let mut actual = subsets_with_dup(input);

            expected.sort();
            actual.sort();

            assert_eq!(expected, actual);
        }
    }
}
